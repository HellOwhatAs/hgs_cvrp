//! Local search functions, including the SWAP* neighborhood.
//!
//! The C++ implementation represents the solution as a doubly linked list of `Node*`.
//! Here the nodes live in a single arena (`Vec<Node>`) and links are arena indices,
//! which keeps the same O(1) updates without any reference counting or unsafe code.
//!
//! Arena layout: indices `0..=nb_clients` are the client nodes (0 is a sentinel),
//! then one start depot per route, then one end depot per route.

use std::collections::BTreeSet;

use crate::circle_sector::CircleSector;
use crate::individual::Individual;
use crate::params::Params;
use crate::util::MY_EPSILON;

#[derive(Clone, Default)]
struct Node {
    /// Tells whether this node represents a depot or not.
    is_depot: bool,
    /// Node index (client number, 0 for depots): used for distance matrix lookups.
    cour: usize,
    /// Position in the route.
    position: usize,
    /// "When" the RI moves for this node have been last tested.
    when_last_tested_ri: i32,
    /// Next node in the route order (arena index).
    next: usize,
    /// Previous node in the route order (arena index).
    prev: usize,
    /// Associated route index.
    route: usize,
    /// Cumulated load on this route until the customer (including itself).
    cumulated_load: f64,
    /// Cumulated time on this route until the customer (including itself).
    cumulated_time: f64,
    /// Difference of cost if the segment (0...cour) is reversed (useful for 2-opt asymmetric).
    cumulated_reversal_distance: f64,
    /// Difference of cost in the current route if the node is removed (used in SWAP*).
    delta_removal: f64,
}

#[derive(Clone, Default)]
struct Route {
    /// Route index.
    cour: usize,
    /// Number of customers visited in the route.
    nb_customers: usize,
    /// "When" this route has been last modified.
    when_last_modified: i32,
    /// "When" the SWAP* moves for this route have been last tested.
    when_last_tested_swap_star: i32,
    /// Arena index of the associated start depot.
    depot: usize,
    /// Total time on the route.
    duration: f64,
    /// Total load on the route.
    load: f64,
    /// Difference of cost if the route is reversed.
    reversal_distance: f64,
    /// Current sum of load and duration penalties.
    penalty: f64,
    /// Polar angle of the barycenter of the route.
    polar_angle_barycenter: f64,
    /// Circle sector associated to the set of customers.
    sector: CircleSector,
}

/// Structure used in SWAP* to remember the three best insertion positions
/// of a customer in a given route.
#[derive(Clone)]
struct ThreeBestInsert {
    when_last_calculated: i32,
    best_cost: [f64; 3],
    best_location: [Option<usize>; 3],
}

impl Default for ThreeBestInsert {
    fn default() -> Self {
        Self {
            when_last_calculated: 0,
            best_cost: [1.0e30; 3],
            best_location: [None; 3],
        }
    }
}

impl ThreeBestInsert {
    fn compare_and_add(&mut self, cost_insert: f64, place_insert: usize) {
        if cost_insert >= self.best_cost[2] {
            return;
        }
        if cost_insert >= self.best_cost[1] {
            self.best_cost[2] = cost_insert;
            self.best_location[2] = Some(place_insert);
        } else if cost_insert >= self.best_cost[0] {
            self.best_cost[2] = self.best_cost[1];
            self.best_location[2] = self.best_location[1];
            self.best_cost[1] = cost_insert;
            self.best_location[1] = Some(place_insert);
        } else {
            self.best_cost[2] = self.best_cost[1];
            self.best_location[2] = self.best_location[1];
            self.best_cost[1] = self.best_cost[0];
            self.best_location[1] = self.best_location[0];
            self.best_cost[0] = cost_insert;
            self.best_location[0] = Some(place_insert);
        }
    }

    /// Resets the structure (no insertion calculated).
    fn reset(&mut self) {
        self.best_cost = [1.0e30; 3];
        self.best_location = [None; 3];
    }
}

/// Structure used to keep track of the best SWAP* move.
struct SwapStarElement {
    move_cost: f64,
    u: Option<usize>,
    best_position_u: Option<usize>,
    v: Option<usize>,
    best_position_v: Option<usize>,
}

impl Default for SwapStarElement {
    fn default() -> Self {
        Self {
            move_cost: 1.0e30,
            u: None,
            best_position_u: None,
            v: None,
            best_position_v: None,
        }
    }
}

/// Main local search structure.
pub struct LocalSearch {
    nb_clients: usize,
    nb_vehicles: usize,

    /// Tells whether all moves have been evaluated without success.
    search_completed: bool,
    /// Total number of moves (RI and SWAP*) applied during the local search.
    /// This is not only a counter, it is also used to avoid repeating move evaluations.
    nb_moves: i32,
    /// Randomized order for checking the nodes in the RI local search.
    order_nodes: Vec<usize>,
    /// Randomized order for checking the routes in the SWAP* local search.
    order_routes: Vec<usize>,
    /// Indices of all empty routes.
    empty_routes: BTreeSet<usize>,
    /// Current loop index.
    loop_id: i32,

    /* THE SOLUTION IS REPRESENTED AS A LINKED LIST OF ELEMENTS */
    nodes: Vec<Node>,
    routes: Vec<Route>,
    /// (SWAP*) For each route and node, the cheapest insertion cost, flat layout
    /// `route * (nb_clients + 1) + client`.
    best_insert_client: Vec<ThreeBestInsert>,

    /* TEMPORARY VARIABLES USED IN THE LOCAL SEARCH LOOPS */
    // node_u_prev -> node_u -> node_x -> node_x_next
    // node_v_prev -> node_v -> node_y -> node_y_next
    node_u: usize,
    node_x: usize,
    node_v: usize,
    node_y: usize,
    route_u: usize,
    route_v: usize,
    node_u_prev_index: usize,
    node_u_index: usize,
    node_x_index: usize,
    node_x_next_index: usize,
    node_v_prev_index: usize,
    node_v_index: usize,
    node_y_index: usize,
    node_y_next_index: usize,
    load_u: f64,
    load_x: f64,
    load_v: f64,
    load_y: f64,
    service_u: f64,
    service_x: f64,
    service_v: f64,
    service_y: f64,
    penalty_capacity_ls: f64,
    penalty_duration_ls: f64,
    intra_route_move: bool,
}

impl LocalSearch {
    pub fn new(params: &Params) -> Self {
        let nb_clients = params.nb_clients;
        let nb_vehicles = params.nb_vehicles;

        let mut nodes = vec![Node::default(); nb_clients + 1 + 2 * nb_vehicles];
        for (i, node) in nodes.iter_mut().enumerate().take(nb_clients + 1) {
            node.cour = i;
            node.is_depot = false;
        }
        let mut routes = Vec::with_capacity(nb_vehicles);
        for r in 0..nb_vehicles {
            let depot = nb_clients + 1 + r;
            let depot_end = nb_clients + 1 + nb_vehicles + r;
            nodes[depot].cour = 0;
            nodes[depot].is_depot = true;
            nodes[depot].route = r;
            nodes[depot_end].cour = 0;
            nodes[depot_end].is_depot = true;
            nodes[depot_end].route = r;
            routes.push(Route {
                cour: r,
                depot,
                ..Route::default()
            });
        }

        Self {
            nb_clients,
            nb_vehicles,
            search_completed: false,
            nb_moves: 0,
            order_nodes: (1..=nb_clients).collect(),
            order_routes: (0..nb_vehicles).collect(),
            empty_routes: BTreeSet::new(),
            loop_id: 0,
            nodes,
            routes,
            best_insert_client: vec![ThreeBestInsert::default(); nb_vehicles * (nb_clients + 1)],
            node_u: 0,
            node_x: 0,
            node_v: 0,
            node_y: 0,
            route_u: 0,
            route_v: 0,
            node_u_prev_index: 0,
            node_u_index: 0,
            node_x_index: 0,
            node_x_next_index: 0,
            node_v_prev_index: 0,
            node_v_index: 0,
            node_y_index: 0,
            node_y_next_index: 0,
            load_u: 0.0,
            load_x: 0.0,
            load_v: 0.0,
            load_y: 0.0,
            service_u: 0.0,
            service_x: 0.0,
            service_v: 0.0,
            service_y: 0.0,
            penalty_capacity_ls: 0.0,
            penalty_duration_ls: 0.0,
            intra_route_move: false,
        }
    }

    #[inline]
    fn depot_node(&self, route: usize) -> usize {
        self.nb_clients + 1 + route
    }

    #[inline]
    fn depot_end_node(&self, route: usize) -> usize {
        self.nb_clients + 1 + self.nb_vehicles + route
    }

    #[inline]
    fn bi_index(&self, route: usize, client: usize) -> usize {
        route * (self.nb_clients + 1) + client
    }

    #[inline]
    fn penalty_excess_duration(&self, params: &Params, duration: f64) -> f64 {
        (duration - params.duration_limit).max(0.0) * self.penalty_duration_ls
    }

    #[inline]
    fn penalty_excess_load(&self, params: &Params, load: f64) -> f64 {
        (load - params.vehicle_capacity).max(0.0) * self.penalty_capacity_ls
    }

    /// Runs the local search with the specified penalty values.
    pub fn run(
        &mut self,
        params: &mut Params,
        indiv: &mut Individual,
        penalty_capacity_ls: f64,
        penalty_duration_ls: f64,
    ) {
        self.penalty_capacity_ls = penalty_capacity_ls;
        self.penalty_duration_ls = penalty_duration_ls;
        self.load_individual(params, indiv);

        // Shuffling the order of the nodes explored by the LS to allow for more diversity in the search
        params.rng.shuffle(&mut self.order_nodes);
        params.rng.shuffle(&mut self.order_routes);
        {
            let nb_granular = params.ap.nb_granular as u32;
            let Params {
                rng,
                correlated_vertices,
                ..
            } = params;
            for i in 1..=self.nb_clients {
                // O(n/nbGranular) shuffles on average, to keep linear-time complexity overall
                if rng.next() % nb_granular == 0 {
                    rng.shuffle(&mut correlated_vertices[i]);
                }
            }
        }
        let params: &Params = params;

        self.search_completed = false;
        self.loop_id = 0;
        while !self.search_completed {
            // Allows at least two loops since some moves involving empty routes are not checked at the first loop
            if self.loop_id > 1 {
                self.search_completed = true;
            }

            /* CLASSICAL ROUTE IMPROVEMENT (RI) MOVES SUBJECT TO A PROXIMITY RESTRICTION */
            for pos_u in 0..self.nb_clients {
                self.node_u = self.order_nodes[pos_u];
                let last_test_ri_node_u = self.nodes[self.node_u].when_last_tested_ri;
                self.nodes[self.node_u].when_last_tested_ri = self.nb_moves;
                let u_client = self.nodes[self.node_u].cour;
                'pos_v: for pos_v in 0..params.correlated_vertices[u_client].len() {
                    self.node_v = params.correlated_vertices[u_client][pos_v];
                    let route_u_modified =
                        self.routes[self.nodes[self.node_u].route].when_last_modified;
                    let route_v_modified =
                        self.routes[self.nodes[self.node_v].route].when_last_modified;
                    // Only evaluate moves involving routes that have been modified since
                    // the last move evaluations for node_u
                    if self.loop_id == 0
                        || route_u_modified.max(route_v_modified) > last_test_ri_node_u
                    {
                        // Randomizing the order of the neighborhoods within this loop does not matter much
                        // as we are already randomizing the order of the node pairs (and it's not very common
                        // to find improving moves of different types for the same node pair)
                        self.set_local_variables_route_u(params);
                        self.set_local_variables_route_v(params);
                        if self.move1(params) {
                            continue 'pos_v;
                        } // RELOCATE
                        if self.move2(params) {
                            continue 'pos_v;
                        } // RELOCATE
                        if self.move3(params) {
                            continue 'pos_v;
                        } // RELOCATE
                        if self.node_u_index <= self.node_v_index && self.move4(params) {
                            continue 'pos_v;
                        } // SWAP
                        if self.move5(params) {
                            continue 'pos_v;
                        } // SWAP
                        if self.node_u_index <= self.node_v_index && self.move6(params) {
                            continue 'pos_v;
                        } // SWAP
                        if self.intra_route_move && self.move7(params) {
                            continue 'pos_v;
                        } // 2-OPT
                        if !self.intra_route_move && self.move8(params) {
                            continue 'pos_v;
                        } // 2-OPT*
                        if !self.intra_route_move && self.move9(params) {
                            continue 'pos_v;
                        } // 2-OPT*

                        // Trying moves that insert node_u directly after the depot
                        if self.nodes[self.nodes[self.node_v].prev].is_depot {
                            self.node_v = self.nodes[self.node_v].prev;
                            self.set_local_variables_route_v(params);
                            if self.move1(params) {
                                continue 'pos_v;
                            } // RELOCATE
                            if self.move2(params) {
                                continue 'pos_v;
                            } // RELOCATE
                            if self.move3(params) {
                                continue 'pos_v;
                            } // RELOCATE
                            if !self.intra_route_move && self.move8(params) {
                                continue 'pos_v;
                            } // 2-OPT*
                            if !self.intra_route_move && self.move9(params) {
                                continue 'pos_v;
                            } // 2-OPT*
                        }
                    }
                }

                /* MOVES INVOLVING AN EMPTY ROUTE -- NOT TESTED IN THE FIRST LOOP TO AVOID INCREASING TOO MUCH THE FLEET SIZE */
                if self.loop_id > 0 && !self.empty_routes.is_empty() {
                    let empty_route = *self.empty_routes.iter().next().unwrap();
                    self.node_v = self.routes[empty_route].depot;
                    self.set_local_variables_route_u(params);
                    self.set_local_variables_route_v(params);
                    if self.move1(params) {
                        continue;
                    } // RELOCATE
                    if self.move2(params) {
                        continue;
                    } // RELOCATE
                    if self.move3(params) {
                        continue;
                    } // RELOCATE
                    if self.move9(params) {
                        continue;
                    } // 2-OPT*
                }
            }

            if params.ap.use_swap_star && params.are_coordinates_provided {
                /* (SWAP*) MOVES LIMITED TO ROUTE PAIRS WHOSE CIRCLE SECTORS OVERLAP */
                for r_u in 0..self.nb_vehicles {
                    self.route_u = self.order_routes[r_u];
                    let last_test_swap_star_route_u =
                        self.routes[self.route_u].when_last_tested_swap_star;
                    self.routes[self.route_u].when_last_tested_swap_star = self.nb_moves;
                    for r_v in 0..self.nb_vehicles {
                        self.route_v = self.order_routes[r_v];
                        let route_u = &self.routes[self.route_u];
                        let route_v = &self.routes[self.route_v];
                        if route_u.nb_customers > 0
                            && route_v.nb_customers > 0
                            && route_u.cour < route_v.cour
                            && (self.loop_id == 0
                                || route_u.when_last_modified.max(route_v.when_last_modified)
                                    > last_test_swap_star_route_u)
                            && CircleSector::overlap(&route_u.sector, &route_v.sector)
                        {
                            self.swap_star(params);
                        }
                    }
                }
            }

            self.loop_id += 1;
        }

        // Register the solution produced by the LS in the individual
        self.export_individual(params, indiv);
    }

    fn set_local_variables_route_u(&mut self, params: &Params) {
        self.route_u = self.nodes[self.node_u].route;
        self.node_x = self.nodes[self.node_u].next;
        self.node_x_next_index = self.nodes[self.nodes[self.node_x].next].cour;
        self.node_u_index = self.nodes[self.node_u].cour;
        self.node_u_prev_index = self.nodes[self.nodes[self.node_u].prev].cour;
        self.node_x_index = self.nodes[self.node_x].cour;
        self.load_u = params.clients[self.node_u_index].demand;
        self.service_u = params.clients[self.node_u_index].service_duration;
        self.load_x = params.clients[self.node_x_index].demand;
        self.service_x = params.clients[self.node_x_index].service_duration;
    }

    fn set_local_variables_route_v(&mut self, params: &Params) {
        self.route_v = self.nodes[self.node_v].route;
        self.node_y = self.nodes[self.node_v].next;
        self.node_y_next_index = self.nodes[self.nodes[self.node_y].next].cour;
        self.node_v_index = self.nodes[self.node_v].cour;
        self.node_v_prev_index = self.nodes[self.nodes[self.node_v].prev].cour;
        self.node_y_index = self.nodes[self.node_y].cour;
        self.load_v = params.clients[self.node_v_index].demand;
        self.service_v = params.clients[self.node_v_index].service_duration;
        self.load_y = params.clients[self.node_y_index].demand;
        self.service_y = params.clients[self.node_y_index].service_duration;
        self.intra_route_move = self.route_u == self.route_v;
    }

    /// If U is a client node, remove U and insert it after V.
    fn move1(&mut self, p: &Params) -> bool {
        let d = |i: usize, j: usize| p.time_cost.get(i, j);
        let mut cost_supp_u = d(self.node_u_prev_index, self.node_x_index)
            - d(self.node_u_prev_index, self.node_u_index)
            - d(self.node_u_index, self.node_x_index);
        let mut cost_supp_v = d(self.node_v_index, self.node_u_index)
            + d(self.node_u_index, self.node_y_index)
            - d(self.node_v_index, self.node_y_index);

        if !self.intra_route_move {
            let route_u = &self.routes[self.route_u];
            let route_v = &self.routes[self.route_v];
            // Early move pruning to save CPU time: this move cannot improve
            // without checking additional (load, duration...) constraints
            if cost_supp_u + cost_supp_v >= route_u.penalty + route_v.penalty {
                return false;
            }

            cost_supp_u += self
                .penalty_excess_duration(p, route_u.duration + cost_supp_u - self.service_u)
                + self.penalty_excess_load(p, route_u.load - self.load_u)
                - route_u.penalty;

            cost_supp_v += self
                .penalty_excess_duration(p, route_v.duration + cost_supp_v + self.service_u)
                + self.penalty_excess_load(p, route_v.load + self.load_u)
                - route_v.penalty;
        }

        if cost_supp_u + cost_supp_v > -MY_EPSILON {
            return false;
        }
        if self.node_u_index == self.node_y_index {
            return false;
        }

        self.insert_node(self.node_u, self.node_v);
        self.nb_moves += 1; // Increment move counter before updating route data
        self.search_completed = false;
        self.update_route_data(p, self.route_u);
        if !self.intra_route_move {
            self.update_route_data(p, self.route_v);
        }
        true
    }

    /// If U and X are client nodes, remove them and insert (U,X) after V.
    fn move2(&mut self, p: &Params) -> bool {
        let d = |i: usize, j: usize| p.time_cost.get(i, j);
        let mut cost_supp_u = d(self.node_u_prev_index, self.node_x_next_index)
            - d(self.node_u_prev_index, self.node_u_index)
            - d(self.node_x_index, self.node_x_next_index);
        let mut cost_supp_v = d(self.node_v_index, self.node_u_index)
            + d(self.node_x_index, self.node_y_index)
            - d(self.node_v_index, self.node_y_index);

        if !self.intra_route_move {
            let route_u = &self.routes[self.route_u];
            let route_v = &self.routes[self.route_v];
            if cost_supp_u + cost_supp_v >= route_u.penalty + route_v.penalty {
                return false;
            }

            cost_supp_u += self.penalty_excess_duration(
                p,
                route_u.duration + cost_supp_u
                    - d(self.node_u_index, self.node_x_index)
                    - self.service_u
                    - self.service_x,
            ) + self
                .penalty_excess_load(p, route_u.load - self.load_u - self.load_x)
                - route_u.penalty;

            cost_supp_v += self.penalty_excess_duration(
                p,
                route_v.duration
                    + cost_supp_v
                    + d(self.node_u_index, self.node_x_index)
                    + self.service_u
                    + self.service_x,
            ) + self
                .penalty_excess_load(p, route_v.load + self.load_u + self.load_x)
                - route_v.penalty;
        }

        if cost_supp_u + cost_supp_v > -MY_EPSILON {
            return false;
        }
        if self.node_u == self.node_y
            || self.node_v == self.node_x
            || self.nodes[self.node_x].is_depot
        {
            return false;
        }

        self.insert_node(self.node_u, self.node_v);
        self.insert_node(self.node_x, self.node_u);
        self.nb_moves += 1;
        self.search_completed = false;
        self.update_route_data(p, self.route_u);
        if !self.intra_route_move {
            self.update_route_data(p, self.route_v);
        }
        true
    }

    /// If U and X are client nodes, remove them and insert (X,U) after V.
    fn move3(&mut self, p: &Params) -> bool {
        let d = |i: usize, j: usize| p.time_cost.get(i, j);
        let mut cost_supp_u = d(self.node_u_prev_index, self.node_x_next_index)
            - d(self.node_u_prev_index, self.node_u_index)
            - d(self.node_u_index, self.node_x_index)
            - d(self.node_x_index, self.node_x_next_index);
        let mut cost_supp_v = d(self.node_v_index, self.node_x_index)
            + d(self.node_x_index, self.node_u_index)
            + d(self.node_u_index, self.node_y_index)
            - d(self.node_v_index, self.node_y_index);

        if !self.intra_route_move {
            let route_u = &self.routes[self.route_u];
            let route_v = &self.routes[self.route_v];
            if cost_supp_u + cost_supp_v >= route_u.penalty + route_v.penalty {
                return false;
            }

            cost_supp_u += self.penalty_excess_duration(
                p,
                route_u.duration + cost_supp_u - self.service_u - self.service_x,
            ) + self
                .penalty_excess_load(p, route_u.load - self.load_u - self.load_x)
                - route_u.penalty;

            cost_supp_v += self.penalty_excess_duration(
                p,
                route_v.duration + cost_supp_v + self.service_u + self.service_x,
            ) + self
                .penalty_excess_load(p, route_v.load + self.load_u + self.load_x)
                - route_v.penalty;
        }

        if cost_supp_u + cost_supp_v > -MY_EPSILON {
            return false;
        }
        if self.node_u == self.node_y
            || self.node_x == self.node_v
            || self.nodes[self.node_x].is_depot
        {
            return false;
        }

        self.insert_node(self.node_x, self.node_v);
        self.insert_node(self.node_u, self.node_x);
        self.nb_moves += 1;
        self.search_completed = false;
        self.update_route_data(p, self.route_u);
        if !self.intra_route_move {
            self.update_route_data(p, self.route_v);
        }
        true
    }

    /// If U and V are client nodes, swap U and V.
    fn move4(&mut self, p: &Params) -> bool {
        let d = |i: usize, j: usize| p.time_cost.get(i, j);
        let mut cost_supp_u = d(self.node_u_prev_index, self.node_v_index)
            + d(self.node_v_index, self.node_x_index)
            - d(self.node_u_prev_index, self.node_u_index)
            - d(self.node_u_index, self.node_x_index);
        let mut cost_supp_v = d(self.node_v_prev_index, self.node_u_index)
            + d(self.node_u_index, self.node_y_index)
            - d(self.node_v_prev_index, self.node_v_index)
            - d(self.node_v_index, self.node_y_index);

        if !self.intra_route_move {
            let route_u = &self.routes[self.route_u];
            let route_v = &self.routes[self.route_v];
            if cost_supp_u + cost_supp_v >= route_u.penalty + route_v.penalty {
                return false;
            }

            cost_supp_u += self.penalty_excess_duration(
                p,
                route_u.duration + cost_supp_u + self.service_v - self.service_u,
            ) + self
                .penalty_excess_load(p, route_u.load + self.load_v - self.load_u)
                - route_u.penalty;

            cost_supp_v += self.penalty_excess_duration(
                p,
                route_v.duration + cost_supp_v - self.service_v + self.service_u,
            ) + self
                .penalty_excess_load(p, route_v.load + self.load_u - self.load_v)
                - route_v.penalty;
        }

        if cost_supp_u + cost_supp_v > -MY_EPSILON {
            return false;
        }
        if self.node_u_index == self.node_v_prev_index || self.node_u_index == self.node_y_index {
            return false;
        }

        self.swap_node(self.node_u, self.node_v);
        self.nb_moves += 1;
        self.search_completed = false;
        self.update_route_data(p, self.route_u);
        if !self.intra_route_move {
            self.update_route_data(p, self.route_v);
        }
        true
    }

    /// If U, X and V are client nodes, swap (U,X) and V.
    fn move5(&mut self, p: &Params) -> bool {
        let d = |i: usize, j: usize| p.time_cost.get(i, j);
        let mut cost_supp_u = d(self.node_u_prev_index, self.node_v_index)
            + d(self.node_v_index, self.node_x_next_index)
            - d(self.node_u_prev_index, self.node_u_index)
            - d(self.node_x_index, self.node_x_next_index);
        let mut cost_supp_v = d(self.node_v_prev_index, self.node_u_index)
            + d(self.node_x_index, self.node_y_index)
            - d(self.node_v_prev_index, self.node_v_index)
            - d(self.node_v_index, self.node_y_index);

        if !self.intra_route_move {
            let route_u = &self.routes[self.route_u];
            let route_v = &self.routes[self.route_v];
            if cost_supp_u + cost_supp_v >= route_u.penalty + route_v.penalty {
                return false;
            }

            cost_supp_u += self.penalty_excess_duration(
                p,
                route_u.duration + cost_supp_u - d(self.node_u_index, self.node_x_index)
                    + self.service_v
                    - self.service_u
                    - self.service_x,
            ) + self
                .penalty_excess_load(p, route_u.load + self.load_v - self.load_u - self.load_x)
                - route_u.penalty;

            cost_supp_v += self.penalty_excess_duration(
                p,
                route_v.duration + cost_supp_v + d(self.node_u_index, self.node_x_index)
                    - self.service_v
                    + self.service_u
                    + self.service_x,
            ) + self
                .penalty_excess_load(p, route_v.load + self.load_u + self.load_x - self.load_v)
                - route_v.penalty;
        }

        if cost_supp_u + cost_supp_v > -MY_EPSILON {
            return false;
        }
        if self.node_u == self.nodes[self.node_v].prev
            || self.node_x == self.nodes[self.node_v].prev
            || self.node_u == self.node_y
            || self.nodes[self.node_x].is_depot
        {
            return false;
        }

        self.swap_node(self.node_u, self.node_v);
        self.insert_node(self.node_x, self.node_u);
        self.nb_moves += 1;
        self.search_completed = false;
        self.update_route_data(p, self.route_u);
        if !self.intra_route_move {
            self.update_route_data(p, self.route_v);
        }
        true
    }

    /// If (U,X) and (V,Y) are client nodes, swap (U,X) and (V,Y).
    fn move6(&mut self, p: &Params) -> bool {
        let d = |i: usize, j: usize| p.time_cost.get(i, j);
        let mut cost_supp_u = d(self.node_u_prev_index, self.node_v_index)
            + d(self.node_y_index, self.node_x_next_index)
            - d(self.node_u_prev_index, self.node_u_index)
            - d(self.node_x_index, self.node_x_next_index);
        let mut cost_supp_v = d(self.node_v_prev_index, self.node_u_index)
            + d(self.node_x_index, self.node_y_next_index)
            - d(self.node_v_prev_index, self.node_v_index)
            - d(self.node_y_index, self.node_y_next_index);

        if !self.intra_route_move {
            let route_u = &self.routes[self.route_u];
            let route_v = &self.routes[self.route_v];
            if cost_supp_u + cost_supp_v >= route_u.penalty + route_v.penalty {
                return false;
            }

            cost_supp_u += self.penalty_excess_duration(
                p,
                route_u.duration + cost_supp_u - d(self.node_u_index, self.node_x_index)
                    + d(self.node_v_index, self.node_y_index)
                    + self.service_v
                    + self.service_y
                    - self.service_u
                    - self.service_x,
            ) + self.penalty_excess_load(
                p,
                route_u.load + self.load_v + self.load_y - self.load_u - self.load_x,
            ) - route_u.penalty;

            cost_supp_v += self.penalty_excess_duration(
                p,
                route_v.duration + cost_supp_v + d(self.node_u_index, self.node_x_index)
                    - d(self.node_v_index, self.node_y_index)
                    - self.service_v
                    - self.service_y
                    + self.service_u
                    + self.service_x,
            ) + self.penalty_excess_load(
                p,
                route_v.load + self.load_u + self.load_x - self.load_v - self.load_y,
            ) - route_v.penalty;
        }

        if cost_supp_u + cost_supp_v > -MY_EPSILON {
            return false;
        }
        if self.nodes[self.node_x].is_depot
            || self.nodes[self.node_y].is_depot
            || self.node_y == self.nodes[self.node_u].prev
            || self.node_u == self.node_y
            || self.node_x == self.node_v
            || self.node_v == self.nodes[self.node_x].next
        {
            return false;
        }

        self.swap_node(self.node_u, self.node_v);
        self.swap_node(self.node_x, self.node_y);
        self.nb_moves += 1;
        self.search_completed = false;
        self.update_route_data(p, self.route_u);
        if !self.intra_route_move {
            self.update_route_data(p, self.route_v);
        }
        true
    }

    /// If route(U) == route(V), replace (U,X) and (V,Y) by (U,V) and (X,Y).
    fn move7(&mut self, p: &Params) -> bool {
        if self.nodes[self.node_u].position > self.nodes[self.node_v].position {
            return false;
        }

        let d = |i: usize, j: usize| p.time_cost.get(i, j);
        let cost = d(self.node_u_index, self.node_v_index)
            + d(self.node_x_index, self.node_y_index)
            - d(self.node_u_index, self.node_x_index)
            - d(self.node_v_index, self.node_y_index)
            + self.nodes[self.node_v].cumulated_reversal_distance
            - self.nodes[self.node_x].cumulated_reversal_distance;

        if cost > -MY_EPSILON {
            return false;
        }
        if self.nodes[self.node_u].next == self.node_v {
            return false;
        }

        // Reverse the segment between X and V
        let mut node_num = self.nodes[self.node_x].next;
        self.nodes[self.node_x].prev = node_num;
        self.nodes[self.node_x].next = self.node_y;

        while node_num != self.node_v {
            let temp = self.nodes[node_num].next;
            let node = &mut self.nodes[node_num];
            std::mem::swap(&mut node.next, &mut node.prev);
            node_num = temp;
        }

        self.nodes[self.node_v].next = self.nodes[self.node_v].prev;
        self.nodes[self.node_v].prev = self.node_u;
        self.nodes[self.node_u].next = self.node_v;
        self.nodes[self.node_y].prev = self.node_x;

        self.nb_moves += 1;
        self.search_completed = false;
        self.update_route_data(p, self.route_u);
        true
    }

    /// If route(U) != route(V), replace (U,X) and (V,Y) by (U,V) and (X,Y).
    fn move8(&mut self, p: &Params) -> bool {
        let d = |i: usize, j: usize| p.time_cost.get(i, j);
        let mut cost = {
            let route_u = &self.routes[self.route_u];
            let route_v = &self.routes[self.route_v];
            d(self.node_u_index, self.node_v_index) + d(self.node_x_index, self.node_y_index)
                - d(self.node_u_index, self.node_x_index)
                - d(self.node_v_index, self.node_y_index)
                + self.nodes[self.node_v].cumulated_reversal_distance
                + route_u.reversal_distance
                - self.nodes[self.node_x].cumulated_reversal_distance
                - route_u.penalty
                - route_v.penalty
        };

        // Early move pruning to save CPU time: this move cannot improve
        // without checking additional (load, duration...) constraints
        if cost >= 0.0 {
            return false;
        }

        {
            let route_u = &self.routes[self.route_u];
            let route_v = &self.routes[self.route_v];
            let node_u = &self.nodes[self.node_u];
            let node_v = &self.nodes[self.node_v];
            let node_x = &self.nodes[self.node_x];
            cost += self.penalty_excess_duration(
                p,
                node_u.cumulated_time
                    + node_v.cumulated_time
                    + node_v.cumulated_reversal_distance
                    + d(self.node_u_index, self.node_v_index),
            ) + self.penalty_excess_duration(
                p,
                route_u.duration - node_u.cumulated_time - d(self.node_u_index, self.node_x_index)
                    + route_u.reversal_distance
                    - node_x.cumulated_reversal_distance
                    + route_v.duration
                    - node_v.cumulated_time
                    - d(self.node_v_index, self.node_y_index)
                    + d(self.node_x_index, self.node_y_index),
            ) + self.penalty_excess_load(p, node_u.cumulated_load + node_v.cumulated_load)
                + self.penalty_excess_load(
                    p,
                    route_u.load + route_v.load - node_u.cumulated_load - node_v.cumulated_load,
                );
        }

        if cost > -MY_EPSILON {
            return false;
        }

        let depot_u = self.routes[self.route_u].depot;
        let depot_v = self.routes[self.route_v].depot;
        let depot_u_fin = self.nodes[depot_u].prev;
        let depot_v_fin = self.nodes[depot_v].prev;
        let depot_v_suiv = self.nodes[depot_v].next;

        // Reverse the tail of route U and append it to route V (and vice versa)
        let mut xx = self.node_x;
        while !self.nodes[xx].is_depot {
            let temp = self.nodes[xx].next;
            let node = &mut self.nodes[xx];
            std::mem::swap(&mut node.next, &mut node.prev);
            node.route = self.route_v;
            xx = temp;
        }

        let mut vv = self.node_v;
        while !self.nodes[vv].is_depot {
            let temp = self.nodes[vv].prev;
            let node = &mut self.nodes[vv];
            std::mem::swap(&mut node.prev, &mut node.next);
            node.route = self.route_u;
            vv = temp;
        }

        self.nodes[self.node_u].next = self.node_v;
        self.nodes[self.node_v].prev = self.node_u;
        self.nodes[self.node_x].next = self.node_y;
        self.nodes[self.node_y].prev = self.node_x;

        if self.nodes[self.node_x].is_depot {
            self.nodes[depot_u_fin].next = depot_u;
            self.nodes[depot_u_fin].prev = depot_v_suiv;
            self.nodes[depot_v_suiv].next = depot_u_fin;
            self.nodes[depot_v].next = self.node_y;
            self.nodes[self.node_y].prev = depot_v;
        } else if self.nodes[self.node_v].is_depot {
            self.nodes[depot_v].next = self.nodes[depot_u_fin].prev;
            let new_next = self.nodes[depot_v].next;
            self.nodes[new_next].prev = depot_v;
            self.nodes[depot_v].prev = depot_v_fin;
            self.nodes[depot_u_fin].prev = self.node_u;
            self.nodes[self.node_u].next = depot_u_fin;
        } else {
            self.nodes[depot_v].next = self.nodes[depot_u_fin].prev;
            let new_next = self.nodes[depot_v].next;
            self.nodes[new_next].prev = depot_v;
            self.nodes[depot_u_fin].prev = depot_v_suiv;
            self.nodes[depot_v_suiv].next = depot_u_fin;
        }

        self.nb_moves += 1;
        self.search_completed = false;
        self.update_route_data(p, self.route_u);
        self.update_route_data(p, self.route_v);
        true
    }

    /// If route(U) != route(V), replace (U,X) and (V,Y) by (U,Y) and (V,X).
    fn move9(&mut self, p: &Params) -> bool {
        let d = |i: usize, j: usize| p.time_cost.get(i, j);
        let mut cost = {
            let route_u = &self.routes[self.route_u];
            let route_v = &self.routes[self.route_v];
            d(self.node_u_index, self.node_y_index) + d(self.node_v_index, self.node_x_index)
                - d(self.node_u_index, self.node_x_index)
                - d(self.node_v_index, self.node_y_index)
                - route_u.penalty
                - route_v.penalty
        };

        // Early move pruning to save CPU time
        if cost >= 0.0 {
            return false;
        }

        {
            let route_u = &self.routes[self.route_u];
            let route_v = &self.routes[self.route_v];
            let node_u = &self.nodes[self.node_u];
            let node_v = &self.nodes[self.node_v];
            cost += self.penalty_excess_duration(
                p,
                node_u.cumulated_time + route_v.duration
                    - node_v.cumulated_time
                    - d(self.node_v_index, self.node_y_index)
                    + d(self.node_u_index, self.node_y_index),
            ) + self.penalty_excess_duration(
                p,
                route_u.duration - node_u.cumulated_time - d(self.node_u_index, self.node_x_index)
                    + node_v.cumulated_time
                    + d(self.node_v_index, self.node_x_index),
            ) + self.penalty_excess_load(
                p,
                node_u.cumulated_load + route_v.load - node_v.cumulated_load,
            ) + self.penalty_excess_load(
                p,
                node_v.cumulated_load + route_u.load - node_u.cumulated_load,
            );
        }

        if cost > -MY_EPSILON {
            return false;
        }

        let depot_u = self.routes[self.route_u].depot;
        let depot_v = self.routes[self.route_v].depot;
        let depot_u_fin = self.nodes[depot_u].prev;
        let depot_v_fin = self.nodes[depot_v].prev;
        let depot_u_pred = self.nodes[depot_u_fin].prev;

        // Swap the tails of the two routes
        let mut count = self.node_y;
        while !self.nodes[count].is_depot {
            self.nodes[count].route = self.route_u;
            count = self.nodes[count].next;
        }

        count = self.node_x;
        while !self.nodes[count].is_depot {
            self.nodes[count].route = self.route_v;
            count = self.nodes[count].next;
        }

        self.nodes[self.node_u].next = self.node_y;
        self.nodes[self.node_y].prev = self.node_u;
        self.nodes[self.node_v].next = self.node_x;
        self.nodes[self.node_x].prev = self.node_v;

        if self.nodes[self.node_x].is_depot {
            self.nodes[depot_u_fin].prev = self.nodes[depot_v_fin].prev;
            let new_prev = self.nodes[depot_u_fin].prev;
            self.nodes[new_prev].next = depot_u_fin;
            self.nodes[self.node_v].next = depot_v_fin;
            self.nodes[depot_v_fin].prev = self.node_v;
        } else {
            self.nodes[depot_u_fin].prev = self.nodes[depot_v_fin].prev;
            let new_prev = self.nodes[depot_u_fin].prev;
            self.nodes[new_prev].next = depot_u_fin;
            self.nodes[depot_v_fin].prev = depot_u_pred;
            self.nodes[depot_u_pred].next = depot_v_fin;
        }

        self.nb_moves += 1;
        self.search_completed = false;
        self.update_route_data(p, self.route_u);
        self.update_route_data(p, self.route_v);
        true
    }

    /// Calculates all SWAP* moves between route_u and route_v and applies the best improving one.
    fn swap_star(&mut self, p: &Params) -> bool {
        let mut best = SwapStarElement::default();

        // Preprocessing insertion costs
        self.preprocess_insertions(p, self.route_u, self.route_v);
        self.preprocess_insertions(p, self.route_v, self.route_u);

        let route_u = self.route_u;
        let route_v = self.route_v;
        let depot_u_next = self.nodes[self.routes[route_u].depot].next;
        let depot_v_next = self.nodes[self.routes[route_v].depot].next;
        let d = |i: usize, j: usize| p.time_cost.get(i, j);

        // Evaluating the moves
        let mut u = depot_u_next;
        while !self.nodes[u].is_depot {
            let u_cour = self.nodes[u].cour;
            let mut v = depot_v_next;
            while !self.nodes[v].is_depot {
                let v_cour = self.nodes[v].cour;
                let delta_pen_route_u = self.penalty_excess_load(
                    p,
                    self.routes[route_u].load + p.clients[v_cour].demand - p.clients[u_cour].demand,
                ) - self.routes[route_u].penalty;
                let delta_pen_route_v = self.penalty_excess_load(
                    p,
                    self.routes[route_v].load + p.clients[u_cour].demand - p.clients[v_cour].demand,
                ) - self.routes[route_v].penalty;

                // Quick filter: possibly early elimination of many SWAP* due to the
                // capacity constraints/penalties and bounds on insertion costs
                if delta_pen_route_u
                    + self.nodes[u].delta_removal
                    + delta_pen_route_v
                    + self.nodes[v].delta_removal
                    <= 0.0
                {
                    // Evaluate best reinsertion cost of U in the route of V where V has been removed
                    let (extra_v, best_position_u) =
                        self.get_cheapest_insert_simult_removal(p, u, v);
                    // Evaluate best reinsertion cost of V in the route of U where U has been removed
                    let (extra_u, best_position_v) =
                        self.get_cheapest_insert_simult_removal(p, v, u);

                    // Evaluating final cost
                    let move_cost = delta_pen_route_u
                        + self.nodes[u].delta_removal
                        + extra_u
                        + delta_pen_route_v
                        + self.nodes[v].delta_removal
                        + extra_v
                        + self.penalty_excess_duration(
                            p,
                            self.routes[route_u].duration
                                + self.nodes[u].delta_removal
                                + extra_u
                                + p.clients[v_cour].service_duration
                                - p.clients[u_cour].service_duration,
                        )
                        + self.penalty_excess_duration(
                            p,
                            self.routes[route_v].duration + self.nodes[v].delta_removal + extra_v
                                - p.clients[v_cour].service_duration
                                + p.clients[u_cour].service_duration,
                        );

                    if move_cost < best.move_cost {
                        best = SwapStarElement {
                            move_cost,
                            u: Some(u),
                            best_position_u,
                            v: Some(v),
                            best_position_v,
                        };
                    }
                }
                v = self.nodes[v].next;
            }
            u = self.nodes[u].next;
        }

        // Including RELOCATE from node_u towards route_v (costs nothing to include in the evaluation
        // at this step since we already have the best insertion location).
        // Moreover, since the granularity criterion is different, this can lead to different improving moves.
        let mut u = depot_u_next;
        while !self.nodes[u].is_depot {
            let u_cour = self.nodes[u].cour;
            let bi = &self.best_insert_client[self.bi_index(route_v, u_cour)];
            let best_position_u = bi.best_location[0];
            let delta_dist_route_v = bi.best_cost[0];
            let u_prev_cour = self.nodes[self.nodes[u].prev].cour;
            let u_next_cour = self.nodes[self.nodes[u].next].cour;
            let delta_dist_route_u =
                d(u_prev_cour, u_next_cour) - d(u_prev_cour, u_cour) - d(u_cour, u_next_cour);
            let move_cost = delta_dist_route_u
                + delta_dist_route_v
                + self.penalty_excess_load(p, self.routes[route_u].load - p.clients[u_cour].demand)
                - self.routes[route_u].penalty
                + self.penalty_excess_load(p, self.routes[route_v].load + p.clients[u_cour].demand)
                - self.routes[route_v].penalty
                + self.penalty_excess_duration(
                    p,
                    self.routes[route_u].duration + delta_dist_route_u
                        - p.clients[u_cour].service_duration,
                )
                + self.penalty_excess_duration(
                    p,
                    self.routes[route_v].duration
                        + delta_dist_route_v
                        + p.clients[u_cour].service_duration,
                );

            if move_cost < best.move_cost {
                best = SwapStarElement {
                    move_cost,
                    u: Some(u),
                    best_position_u,
                    v: None,
                    best_position_v: None,
                };
            }
            u = self.nodes[u].next;
        }

        // Including RELOCATE from node_v towards route_u
        let mut v = depot_v_next;
        while !self.nodes[v].is_depot {
            let v_cour = self.nodes[v].cour;
            let bi = &self.best_insert_client[self.bi_index(route_u, v_cour)];
            let best_position_v = bi.best_location[0];
            let delta_dist_route_u = bi.best_cost[0];
            let v_prev_cour = self.nodes[self.nodes[v].prev].cour;
            let v_next_cour = self.nodes[self.nodes[v].next].cour;
            let delta_dist_route_v =
                d(v_prev_cour, v_next_cour) - d(v_prev_cour, v_cour) - d(v_cour, v_next_cour);
            let move_cost = delta_dist_route_u
                + delta_dist_route_v
                + self.penalty_excess_load(p, self.routes[route_u].load + p.clients[v_cour].demand)
                - self.routes[route_u].penalty
                + self.penalty_excess_load(p, self.routes[route_v].load - p.clients[v_cour].demand)
                - self.routes[route_v].penalty
                + self.penalty_excess_duration(
                    p,
                    self.routes[route_u].duration
                        + delta_dist_route_u
                        + p.clients[v_cour].service_duration,
                )
                + self.penalty_excess_duration(
                    p,
                    self.routes[route_v].duration + delta_dist_route_v
                        - p.clients[v_cour].service_duration,
                );

            if move_cost < best.move_cost {
                best = SwapStarElement {
                    move_cost,
                    u: None,
                    best_position_u: None,
                    v: Some(v),
                    best_position_v,
                };
            }
            v = self.nodes[v].next;
        }

        if best.move_cost > -MY_EPSILON {
            return false;
        }

        // Applying the best move in case of improvement
        if let Some(position) = best.best_position_u {
            self.insert_node(
                best.u.expect("U is set together with its position"),
                position,
            );
        }
        if let Some(position) = best.best_position_v {
            self.insert_node(
                best.v.expect("V is set together with its position"),
                position,
            );
        }
        self.nb_moves += 1;
        self.search_completed = false;
        self.update_route_data(p, route_u);
        self.update_route_data(p, route_v);
        true
    }

    /// Calculates the insertion cost and position of U in the route of V, where V is omitted.
    /// Returns (best_cost, best_position).
    fn get_cheapest_insert_simult_removal(
        &self,
        p: &Params,
        u: usize,
        v: usize,
    ) -> (f64, Option<usize>) {
        let best_insert =
            &self.best_insert_client[self.bi_index(self.nodes[v].route, self.nodes[u].cour)];

        // Find the best insertion in the route such that V is not next or pred
        // (the optimal insertion in that case can only belong to the top three locations)
        let mut best_position = best_insert.best_location[0];
        let mut best_cost = best_insert.best_cost[0];
        let mut found = match best_position {
            Some(position) => position != v && self.nodes[position].next != v,
            None => false,
        };
        if !found && best_insert.best_location[1].is_some() {
            best_position = best_insert.best_location[1];
            best_cost = best_insert.best_cost[1];
            let position = best_position.expect("checked above");
            found = position != v && self.nodes[position].next != v;
            if !found && best_insert.best_location[2].is_some() {
                best_position = best_insert.best_location[2];
                best_cost = best_insert.best_cost[2];
                found = true;
            }
        }

        // Also test the insertion in the place of V
        let d = |i: usize, j: usize| p.time_cost.get(i, j);
        let v_prev = self.nodes[v].prev;
        let v_next = self.nodes[v].next;
        let delta_cost = d(self.nodes[v_prev].cour, self.nodes[u].cour)
            + d(self.nodes[u].cour, self.nodes[v_next].cour)
            - d(self.nodes[v_prev].cour, self.nodes[v_next].cour);
        if !found || delta_cost < best_cost {
            best_position = Some(v_prev);
            best_cost = delta_cost;
        }

        (best_cost, best_position)
    }

    /// Preprocesses all insertion costs of customers of route r1 in route r2.
    fn preprocess_insertions(&mut self, p: &Params, r1: usize, r2: usize) {
        let d = |i: usize, j: usize| p.time_cost.get(i, j);
        let mut u = self.nodes[self.routes[r1].depot].next;
        while !self.nodes[u].is_depot {
            // Compute the cost of removing U from its route
            let u_cour = self.nodes[u].cour;
            let u_prev_cour = self.nodes[self.nodes[u].prev].cour;
            let u_next = self.nodes[u].next;
            let u_next_cour = self.nodes[u_next].cour;
            self.nodes[u].delta_removal =
                d(u_prev_cour, u_next_cour) - d(u_prev_cour, u_cour) - d(u_cour, u_next_cour);

            // Recompute the table of best insertions in r2 only if it was modified since
            let bi_idx = self.bi_index(r2, u_cour);
            if self.routes[r2].when_last_modified
                > self.best_insert_client[bi_idx].when_last_calculated
            {
                self.best_insert_client[bi_idx].reset();
                self.best_insert_client[bi_idx].when_last_calculated = self.nb_moves;

                let depot2 = self.routes[r2].depot;
                let first = self.nodes[depot2].next;
                let first_cour = self.nodes[first].cour;
                self.best_insert_client[bi_idx].best_cost[0] =
                    d(0, u_cour) + d(u_cour, first_cour) - d(0, first_cour);
                self.best_insert_client[bi_idx].best_location[0] = Some(depot2);

                let mut v = first;
                while !self.nodes[v].is_depot {
                    let v_cour = self.nodes[v].cour;
                    let v_next = self.nodes[v].next;
                    let v_next_cour = self.nodes[v_next].cour;
                    let delta_cost =
                        d(v_cour, u_cour) + d(u_cour, v_next_cour) - d(v_cour, v_next_cour);
                    self.best_insert_client[bi_idx].compare_and_add(delta_cost, v);
                    v = v_next;
                }
            }
            u = u_next;
        }
    }

    /// Solution update: insert U after V.
    fn insert_node(&mut self, u: usize, v: usize) {
        let u_prev = self.nodes[u].prev;
        let u_next = self.nodes[u].next;
        self.nodes[u_prev].next = u_next;
        self.nodes[u_next].prev = u_prev;
        let v_next = self.nodes[v].next;
        self.nodes[v_next].prev = u;
        self.nodes[u].prev = v;
        self.nodes[u].next = v_next;
        self.nodes[v].next = u;
        self.nodes[u].route = self.nodes[v].route;
    }

    /// Solution update: swap U and V.
    fn swap_node(&mut self, u: usize, v: usize) {
        let v_prev = self.nodes[v].prev;
        let v_next = self.nodes[v].next;
        let u_prev = self.nodes[u].prev;
        let u_next = self.nodes[u].next;
        let route_u = self.nodes[u].route;
        let route_v = self.nodes[v].route;

        self.nodes[u_prev].next = v;
        self.nodes[u_next].prev = v;
        self.nodes[v_prev].next = u;
        self.nodes[v_next].prev = u;

        self.nodes[u].prev = v_prev;
        self.nodes[u].next = v_next;
        self.nodes[v].prev = u_prev;
        self.nodes[v].next = u_next;

        self.nodes[u].route = route_v;
        self.nodes[v].route = route_u;
    }

    /// Updates the preprocessed data of a route.
    fn update_route_data(&mut self, p: &Params, route: usize) {
        let mut my_place = 0;
        let mut my_load = 0.0;
        let mut my_time = 0.0;
        let mut my_reversal_distance = 0.0;
        let mut cumulated_x = 0.0;
        let mut cumulated_y = 0.0;

        let depot = self.routes[route].depot;
        {
            let node = &mut self.nodes[depot];
            node.position = 0;
            node.cumulated_load = 0.0;
            node.cumulated_time = 0.0;
            node.cumulated_reversal_distance = 0.0;
        }

        let mut my_node = depot;
        let mut first_it = true;
        while !self.nodes[my_node].is_depot || first_it {
            my_node = self.nodes[my_node].next;
            my_place += 1;
            let cour = self.nodes[my_node].cour;
            let prev_cour = self.nodes[self.nodes[my_node].prev].cour;
            my_load += p.clients[cour].demand;
            my_time += p.time_cost.get(prev_cour, cour) + p.clients[cour].service_duration;
            my_reversal_distance +=
                p.time_cost.get(cour, prev_cour) - p.time_cost.get(prev_cour, cour);
            {
                let node = &mut self.nodes[my_node];
                node.position = my_place;
                node.cumulated_load = my_load;
                node.cumulated_time = my_time;
                node.cumulated_reversal_distance = my_reversal_distance;
            }
            if !self.nodes[my_node].is_depot {
                cumulated_x += p.clients[cour].coord_x;
                cumulated_y += p.clients[cour].coord_y;
                if first_it {
                    self.routes[route]
                        .sector
                        .initialize(p.clients[cour].polar_angle);
                } else {
                    self.routes[route]
                        .sector
                        .extend(p.clients[cour].polar_angle);
                }
            }
            first_it = false;
        }

        let penalty =
            self.penalty_excess_duration(p, my_time) + self.penalty_excess_load(p, my_load);
        let nb_customers = my_place - 1;
        {
            let r = &mut self.routes[route];
            r.duration = my_time;
            r.load = my_load;
            r.penalty = penalty;
            r.nb_customers = nb_customers;
            r.reversal_distance = my_reversal_distance;
            // Remember "when" this route has been last modified
            // (will be used to filter unnecessary move evaluations)
            r.when_last_modified = self.nb_moves;
        }

        if nb_customers == 0 {
            self.routes[route].polar_angle_barycenter = 1.0e30;
            self.empty_routes.insert(route);
        } else {
            self.routes[route].polar_angle_barycenter = (cumulated_y / nb_customers as f64
                - p.clients[0].coord_y)
                .atan2(cumulated_x / nb_customers as f64 - p.clients[0].coord_x);
            self.empty_routes.remove(&route);
        }
    }

    /// Loads an initial solution into the local search structures.
    pub fn load_individual(&mut self, params: &Params, indiv: &Individual) {
        self.empty_routes.clear();
        self.nb_moves = 0;
        for r in 0..self.nb_vehicles {
            let my_depot = self.depot_node(r);
            let my_depot_fin = self.depot_end_node(r);
            self.nodes[my_depot].prev = my_depot_fin;
            self.nodes[my_depot_fin].next = my_depot;
            if !indiv.chrom_r[r].is_empty() {
                let mut my_client = indiv.chrom_r[r][0];
                self.nodes[my_client].route = r;
                self.nodes[my_client].prev = my_depot;
                self.nodes[my_depot].next = my_client;
                for i in 1..indiv.chrom_r[r].len() {
                    let my_client_pred = my_client;
                    my_client = indiv.chrom_r[r][i];
                    self.nodes[my_client].prev = my_client_pred;
                    self.nodes[my_client_pred].next = my_client;
                    self.nodes[my_client].route = r;
                }
                self.nodes[my_client].next = my_depot_fin;
                self.nodes[my_depot_fin].prev = my_client;
            } else {
                self.nodes[my_depot].next = my_depot_fin;
                self.nodes[my_depot_fin].prev = my_depot;
            }
            self.update_route_data(params, r);
            self.routes[r].when_last_tested_swap_star = -1;
            for i in 1..=self.nb_clients {
                // Initializing memory structures
                let bi_idx = self.bi_index(r, i);
                self.best_insert_client[bi_idx].when_last_calculated = -1;
            }
        }

        for i in 1..=self.nb_clients {
            // Initializing memory structures
            self.nodes[i].when_last_tested_ri = -1;
        }
    }

    /// Exports the LS solution into an individual and computes the penalized cost
    /// according to the original penalty weights from Params.
    pub fn export_individual(&self, params: &Params, indiv: &mut Individual) {
        // Empty routes have a polar angle of 1.e30 and therefore always appear at the end
        let mut route_polar_angles: Vec<(f64, usize)> = (0..self.nb_vehicles)
            .map(|r| (self.routes[r].polar_angle_barycenter, r))
            .collect();
        route_polar_angles.sort_by(|a, b| a.0.total_cmp(&b.0).then(a.1.cmp(&b.1)));

        let mut pos = 0;
        for r in 0..self.nb_vehicles {
            indiv.chrom_r[r].clear();
            let mut node = self.nodes[self.routes[route_polar_angles[r].1].depot].next;
            while !self.nodes[node].is_depot {
                indiv.chrom_t[pos] = self.nodes[node].cour;
                indiv.chrom_r[r].push(self.nodes[node].cour);
                node = self.nodes[node].next;
                pos += 1;
            }
        }

        indiv.evaluate_complete_cost(params);
    }
}
