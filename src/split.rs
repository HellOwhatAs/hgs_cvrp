//! Algorithms to decode solutions represented as giant tours into complete CVRP solutions.
//!
//! Direct port of the C++ implementation, itself based on the linear Split of
//! "Vidal, T. (2016). Split algorithm in O(n) for the capacitated vehicle routing problem".

use crate::individual::Individual;
use crate::params::Params;
use crate::util::MY_EPSILON;

#[derive(Clone, Default)]
struct ClientSplit {
    demand: f64,
    service_time: f64,
    d0_x: f64,
    dx_0: f64,
    dnext: f64,
}

/// Simple deque used by the linear Split algorithms.
/// Cursors are signed so that the back can transiently move before the front.
struct TrivialDeque {
    elements: Vec<usize>,
    index_front: i32,
    index_back: i32,
}

impl TrivialDeque {
    fn new(nb_elements: usize, first_node: usize) -> Self {
        let mut elements = vec![0; nb_elements];
        elements[0] = first_node;
        Self {
            elements,
            index_front: 0,
            index_back: 0,
        }
    }

    #[inline]
    fn pop_front(&mut self) {
        self.index_front += 1;
    }

    #[inline]
    fn pop_back(&mut self) {
        self.index_back -= 1;
    }

    #[inline]
    fn push_back(&mut self, i: usize) {
        self.index_back += 1;
        self.elements[self.index_back as usize] = i;
    }

    #[inline]
    fn get_front(&self) -> usize {
        self.elements[self.index_front as usize]
    }

    #[inline]
    fn get_next_front(&self) -> usize {
        self.elements[(self.index_front + 1) as usize]
    }

    #[inline]
    fn get_back(&self) -> usize {
        self.elements[self.index_back as usize]
    }

    fn reset(&mut self, first_node: usize) {
        self.elements[0] = first_node;
        self.index_back = 0;
        self.index_front = 0;
    }

    #[inline]
    fn size(&self) -> i32 {
        self.index_back - self.index_front + 1
    }
}

pub struct Split {
    nb_clients: usize,
    max_vehicles: usize,

    /* Auxiliary data structures to run the Linear Split algorithm */
    cli_split: Vec<ClientSplit>,
    /// Potential vector, flat (nb_vehicles + 1) x (nb_clients + 1).
    potential: Vec<f64>,
    /// Index of the predecessor in an optimal path, same layout as `potential`.
    pred: Vec<usize>,
    /// sum_distance[i] for i > 1 contains the sum of distances: sum_{k=1}^{i-1} d_{k,k+1}.
    sum_distance: Vec<f64>,
    /// sum_load[i] for i >= 1 contains the sum of loads: sum_{k=1}^{i} q_k.
    sum_load: Vec<f64>,
    /// sum_service[i] for i >= 1 contains the sum of service time: sum_{k=1}^{i} s_k.
    sum_service: Vec<f64>,
}

impl Split {
    pub fn new(params: &Params) -> Self {
        let nb_clients = params.nb_clients;
        let nb_vehicles = params.nb_vehicles;
        Self {
            nb_clients,
            max_vehicles: 0,
            cli_split: vec![ClientSplit::default(); nb_clients + 1],
            potential: vec![1.0e30; (nb_vehicles + 1) * (nb_clients + 1)],
            pred: vec![0; (nb_vehicles + 1) * (nb_clients + 1)],
            sum_distance: vec![0.0; nb_clients + 1],
            sum_load: vec![0.0; nb_clients + 1],
            sum_service: vec![0.0; nb_clients + 1],
        }
    }

    #[inline]
    fn idx(&self, k: usize, i: usize) -> usize {
        k * (self.nb_clients + 1) + i
    }

    /// Computes the cost of propagating the label i until j (to be called with i < j only).
    #[inline]
    fn propagate(&self, i: usize, j: usize, k: usize, params: &Params) -> f64 {
        self.potential[self.idx(k, i)] + self.sum_distance[j] - self.sum_distance[i + 1]
            + self.cli_split[i + 1].d0_x
            + self.cli_split[j].dx_0
            + params.penalty_capacity
                * (self.sum_load[j] - self.sum_load[i] - params.vehicle_capacity).max(0.0)
    }

    /// Tests if i dominates j as a predecessor for all nodes x >= j + 1 (assuming i < j).
    #[inline]
    fn dominates(&self, i: usize, j: usize, k: usize, params: &Params) -> bool {
        self.potential[self.idx(k, j)] + self.cli_split[j + 1].d0_x
            > self.potential[self.idx(k, i)] + self.cli_split[i + 1].d0_x + self.sum_distance[j + 1]
                - self.sum_distance[i + 1]
                + params.penalty_capacity * (self.sum_load[j] - self.sum_load[i])
    }

    /// Tests if j dominates i as a predecessor for all nodes x >= j + 1 (assuming i < j).
    #[inline]
    fn dominates_right(&self, i: usize, j: usize, k: usize) -> bool {
        self.potential[self.idx(k, j)] + self.cli_split[j + 1].d0_x
            < self.potential[self.idx(k, i)] + self.cli_split[i + 1].d0_x + self.sum_distance[j + 1]
                - self.sum_distance[i + 1]
                + MY_EPSILON
    }

    /// General Split function: tests the unlimited fleet Split first, and only if it
    /// does not produce a feasible solution, runs the Split algorithm for a limited fleet.
    pub fn general_split(
        &mut self,
        params: &Params,
        indiv: &mut Individual,
        nb_max_vehicles: usize,
    ) {
        // Do not apply Split with fewer vehicles than the trivial (LP) bin packing bound
        self.max_vehicles =
            nb_max_vehicles.max((params.total_demand / params.vehicle_capacity).ceil() as usize);

        // Initialization of the data structures for the linear split algorithms
        for i in 1..=params.nb_clients {
            let client = indiv.chrom_t[i - 1];
            self.cli_split[i].demand = params.clients[client].demand;
            self.cli_split[i].service_time = params.clients[client].service_duration;
            self.cli_split[i].d0_x = params.time_cost.get(0, client);
            self.cli_split[i].dx_0 = params.time_cost.get(client, 0);
            self.cli_split[i].dnext = if i < params.nb_clients {
                params.time_cost.get(client, indiv.chrom_t[i])
            } else {
                -1.0e30
            };
            self.sum_load[i] = self.sum_load[i - 1] + self.cli_split[i].demand;
            self.sum_service[i] = self.sum_service[i - 1] + self.cli_split[i].service_time;
            self.sum_distance[i] = self.sum_distance[i - 1] + self.cli_split[i - 1].dnext;
        }

        // We first try the simple split, and then the Split with limited fleet if not successful
        if !self.split_simple(params, indiv) {
            self.split_lf(params, indiv);
        }

        // Build up the rest of the Individual structure
        indiv.evaluate_complete_cost(params);
    }

    /// Split for unlimited fleet. Returns true if the algorithm reached the beginning of the routes.
    fn split_simple(&mut self, params: &Params, indiv: &mut Individual) -> bool {
        // Reinitialize the potential structure
        let origin = self.idx(0, 0);
        self.potential[origin] = 0.0;
        for i in 1..=params.nb_clients {
            let index = self.idx(0, i);
            self.potential[index] = 1.0e30;
        }

        // MAIN ALGORITHM -- Simple Split using Bellman's algorithm in topological order.
        // This code has been maintained as it is very simple and can be easily adapted to
        // a variety of constraints, whereas the O(n) Split has a more restricted scope.
        if params.is_duration_constraint {
            for i in 0..params.nb_clients {
                let mut load = 0.0;
                let mut distance = 0.0;
                let mut service_duration = 0.0;
                let mut j = i + 1;
                while j <= params.nb_clients && load <= 1.5 * params.vehicle_capacity {
                    load += self.cli_split[j].demand;
                    service_duration += self.cli_split[j].service_time;
                    if j == i + 1 {
                        distance += self.cli_split[j].d0_x;
                    } else {
                        distance += self.cli_split[j - 1].dnext;
                    }
                    let cost = distance
                        + self.cli_split[j].dx_0
                        + params.penalty_capacity * (load - params.vehicle_capacity).max(0.0)
                        + params.penalty_duration
                            * (distance + self.cli_split[j].dx_0 + service_duration
                                - params.duration_limit)
                                .max(0.0);
                    if self.potential[self.idx(0, i)] + cost < self.potential[self.idx(0, j)] {
                        let (from, to) = (self.idx(0, i), self.idx(0, j));
                        self.potential[to] = self.potential[from] + cost;
                        self.pred[to] = i;
                    }
                    j += 1;
                }
            }
        } else {
            let mut queue = TrivialDeque::new(params.nb_clients + 1, 0);
            for i in 1..=params.nb_clients {
                // The front is the best predecessor for i
                let index = self.idx(0, i);
                self.potential[index] = self.propagate(queue.get_front(), i, 0, params);
                self.pred[index] = queue.get_front();

                if i < params.nb_clients {
                    // If i is not dominated by the last of the pile
                    if !self.dominates(queue.get_back(), i, 0, params) {
                        // then i will be inserted, need to remove whoever is dominated by i
                        while queue.size() > 0 && self.dominates_right(queue.get_back(), i, 0) {
                            queue.pop_back();
                        }
                        queue.push_back(i);
                    }
                    // Check iteratively if front is dominated by the next front
                    while queue.size() > 1
                        && self.propagate(queue.get_front(), i + 1, 0, params)
                            > self.propagate(queue.get_next_front(), i + 1, 0, params) - MY_EPSILON
                    {
                        queue.pop_front();
                    }
                }
            }
        }

        if self.potential[self.idx(0, params.nb_clients)] > 1.0e29 {
            panic!("ERROR : no Split solution has been propagated until the last node");
        }

        // Filling the chromR structure
        for k in self.max_vehicles..params.nb_vehicles {
            indiv.chrom_r[k].clear();
        }

        let mut end = params.nb_clients;
        for k in (0..self.max_vehicles).rev() {
            indiv.chrom_r[k].clear();
            let begin = self.pred[self.idx(0, end)];
            for ii in begin..end {
                indiv.chrom_r[k].push(indiv.chrom_t[ii]);
            }
            end = begin;
        }

        // Return OK in case the Split algorithm reached the beginning of the routes
        end == 0
    }

    /// Split for limited fleet. Returns true if the algorithm reached the beginning of the routes.
    fn split_lf(&mut self, params: &Params, indiv: &mut Individual) -> bool {
        // Initialize the potential structure
        let origin = self.idx(0, 0);
        self.potential[origin] = 0.0;
        for k in 0..=self.max_vehicles {
            for i in 1..=params.nb_clients {
                let index = self.idx(k, i);
                self.potential[index] = 1.0e30;
            }
        }

        // MAIN ALGORITHM -- Simple Split using Bellman's algorithm in topological order
        if params.is_duration_constraint {
            for k in 0..self.max_vehicles {
                for i in k..params.nb_clients {
                    // The loop stops as soon as the potential is unreachable (interval property)
                    if self.potential[self.idx(k, i)] >= 1.0e29 {
                        break;
                    }
                    let mut load = 0.0;
                    let mut service_duration = 0.0;
                    let mut distance = 0.0;
                    // Setting a maximum limit on load infeasibility to accelerate the algorithm
                    let mut j = i + 1;
                    while j <= params.nb_clients && load <= 1.5 * params.vehicle_capacity {
                        load += self.cli_split[j].demand;
                        service_duration += self.cli_split[j].service_time;
                        if j == i + 1 {
                            distance += self.cli_split[j].d0_x;
                        } else {
                            distance += self.cli_split[j - 1].dnext;
                        }
                        let cost = distance
                            + self.cli_split[j].dx_0
                            + params.penalty_capacity * (load - params.vehicle_capacity).max(0.0)
                            + params.penalty_duration
                                * (distance + self.cli_split[j].dx_0 + service_duration
                                    - params.duration_limit)
                                    .max(0.0);
                        if self.potential[self.idx(k, i)] + cost
                            < self.potential[self.idx(k + 1, j)]
                        {
                            let (from, to) = (self.idx(k, i), self.idx(k + 1, j));
                            self.potential[to] = self.potential[from] + cost;
                            self.pred[to] = i;
                        }
                        j += 1;
                    }
                }
            }
        } else {
            // Without duration constraints in O(n), from "Vidal, T. (2016).
            // Split algorithm in O(n) for the capacitated vehicle routing problem. C&OR"
            let mut queue = TrivialDeque::new(params.nb_clients + 1, 0);
            for k in 0..self.max_vehicles {
                // In the Split problem there is always one feasible solution with k routes
                // that reaches the index k in the tour
                queue.reset(k);

                // The range of potentials < 1.e29 is always an interval.
                // The size of the queue will stay >= 1 until we reach the end of this interval.
                let mut i = k + 1;
                while i <= params.nb_clients && queue.size() > 0 {
                    // The front is the best predecessor for i
                    let index = self.idx(k + 1, i);
                    self.potential[index] = self.propagate(queue.get_front(), i, k, params);
                    self.pred[index] = queue.get_front();

                    if i < params.nb_clients {
                        // If i is not dominated by the last of the pile
                        if !self.dominates(queue.get_back(), i, k, params) {
                            // then i will be inserted, need to remove whoever it dominates
                            while queue.size() > 0 && self.dominates_right(queue.get_back(), i, k) {
                                queue.pop_back();
                            }
                            queue.push_back(i);
                        }

                        // Check iteratively if front is dominated by the next front
                        while queue.size() > 1
                            && self.propagate(queue.get_front(), i + 1, k, params)
                                > self.propagate(queue.get_next_front(), i + 1, k, params)
                                    - MY_EPSILON
                        {
                            queue.pop_front();
                        }
                    }
                    i += 1;
                }
            }
        }

        if self.potential[self.idx(self.max_vehicles, params.nb_clients)] > 1.0e29 {
            panic!("ERROR : no Split solution has been propagated until the last node");
        }

        // It could be cheaper to use a smaller number of vehicles
        let mut min_cost = self.potential[self.idx(self.max_vehicles, params.nb_clients)];
        let mut nb_routes = self.max_vehicles;
        for k in 1..self.max_vehicles {
            if self.potential[self.idx(k, params.nb_clients)] < min_cost {
                min_cost = self.potential[self.idx(k, params.nb_clients)];
                nb_routes = k;
            }
        }

        // Filling the chromR structure
        for k in nb_routes..params.nb_vehicles {
            indiv.chrom_r[k].clear();
        }

        let mut end = params.nb_clients;
        for k in (0..nb_routes).rev() {
            indiv.chrom_r[k].clear();
            let begin = self.pred[self.idx(k + 1, end)];
            for ii in begin..end {
                indiv.chrom_r[k].push(indiv.chrom_t[ii]);
            }
            end = begin;
        }

        // Return OK in case the Split algorithm reached the beginning of the routes
        end == 0
    }
}
