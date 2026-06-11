//! Main data structures for the method: instance data, penalties and RNG state.

use std::time::Instant;

use crate::algorithm_parameters::AlgorithmParameters;
use crate::circle_sector::CircleSector;
use crate::matrix::SquareMatrix;
use crate::rng::MinstdRand;
use crate::util::PI;

#[derive(Clone, Default)]
pub struct Client {
    /// Coordinate X.
    pub coord_x: f64,
    /// Coordinate Y.
    pub coord_y: f64,
    /// Service duration.
    pub service_duration: f64,
    /// Demand.
    pub demand: f64,
    /// Polar angle of the client around the depot, measured in degrees and truncated for convenience.
    pub polar_angle: i32,
}

/// Stores the problem data along with the mutable search state (adaptive penalties, RNG).
///
/// Like in the C++ implementation, the penalties and the RNG live here so that all the
/// components share a single source of truth; in Rust this means a `&mut Params` is
/// threaded through the calls that update them.
pub struct Params {
    /* PARAMETERS OF THE GENETIC ALGORITHM */
    pub verbose: bool,
    pub ap: AlgorithmParameters,

    /* ADAPTIVE PENALTY COEFFICIENTS */
    pub penalty_capacity: f64,
    pub penalty_duration: f64,

    /* START TIME OF THE ALGORITHM (wall clock, the C++ version uses CPU clock) */
    pub start_time: Instant,

    /* RANDOM NUMBER GENERATOR */
    pub rng: MinstdRand,

    /* DATA OF THE PROBLEM INSTANCE */
    pub is_duration_constraint: bool,
    pub nb_clients: usize,
    pub nb_vehicles: usize,
    pub duration_limit: f64,
    pub vehicle_capacity: f64,
    pub total_demand: f64,
    pub max_demand: f64,
    pub max_dist: f64,
    pub clients: Vec<Client>,
    /// Distance matrix.
    pub time_cost: SquareMatrix,
    /// Neighborhood restrictions: for each client, list of nearby customers.
    pub correlated_vertices: Vec<Vec<usize>>,
    pub are_coordinates_provided: bool,
}

impl Params {
    /// Builds the parameters from a given data set.
    ///
    /// `nb_veh` set to `None` triggers a default fleet size initialization
    /// (the C++ version uses INT_MAX for the same purpose).
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        x_coords: &[f64],
        y_coords: &[f64],
        dist_mtx: SquareMatrix,
        service_time: &[f64],
        demands: &[f64],
        vehicle_capacity: f64,
        duration_limit: f64,
        nb_veh: Option<usize>,
        is_duration_constraint: bool,
        verbose: bool,
        ap: AlgorithmParameters,
    ) -> Result<Params, String> {
        // This marks the starting time of the algorithm
        let start_time = Instant::now();

        let nb_clients = demands.len() - 1; // Need to subtract the depot from the number of nodes
        let rng = MinstdRand::new(ap.seed);

        // Check if valid coordinates are provided
        let are_coordinates_provided =
            demands.len() == x_coords.len() && demands.len() == y_coords.len();

        let mut clients = vec![Client::default(); nb_clients + 1];
        let mut total_demand = 0.0;
        let mut max_demand = 0.0;
        for i in 0..=nb_clients {
            // If use_swap_star is false, x_coords and y_coords may be empty.
            if ap.use_swap_star && are_coordinates_provided {
                clients[i].coord_x = x_coords[i];
                clients[i].coord_y = y_coords[i];
                clients[i].polar_angle = CircleSector::positive_mod(
                    (32768.0
                        * (clients[i].coord_y - y_coords[0])
                            .atan2(clients[i].coord_x - x_coords[0])
                        / PI) as i32,
                );
            }
            clients[i].service_duration = service_time[i];
            clients[i].demand = demands[i];
            if clients[i].demand > max_demand {
                max_demand = clients[i].demand;
            }
            total_demand += clients[i].demand;
        }

        if verbose && ap.use_swap_star && !are_coordinates_provided {
            println!("----- NO COORDINATES HAVE BEEN PROVIDED, SWAP* NEIGHBORHOOD WILL BE DEACTIVATED BY DEFAULT");
        }

        // Default initialization if the number of vehicles has not been provided by the user
        let nb_vehicles = match nb_veh {
            None => {
                // Safety margin: 30% + 3 more vehicles than the trivial bin packing LB
                let default_veh = (1.3 * total_demand / vehicle_capacity).ceil() as usize + 3;
                if verbose {
                    println!(
                        "----- FLEET SIZE WAS NOT SPECIFIED: DEFAULT INITIALIZATION TO {} VEHICLES",
                        default_veh
                    );
                }
                default_veh
            }
            Some(n) => {
                if verbose {
                    println!("----- FLEET SIZE SPECIFIED: SET TO {} VEHICLES", n);
                }
                n
            }
        };

        // Calculation of the maximum distance
        let mut max_dist = 0.0;
        for i in 0..=nb_clients {
            for j in 0..=nb_clients {
                if dist_mtx.get(i, j) > max_dist {
                    max_dist = dist_mtx.get(i, j);
                }
            }
        }

        // Calculation of the correlated vertices for each customer (for the granular restriction)
        let mut set_correlated_vertices: Vec<Vec<usize>> = vec![Vec::new(); nb_clients + 1];
        let mut order_proximity: Vec<(f64, usize)> = Vec::new();
        for i in 1..=nb_clients {
            order_proximity.clear();
            for j in 1..=nb_clients {
                if i != j {
                    order_proximity.push((dist_mtx.get(i, j), j));
                }
            }
            order_proximity.sort_by(|a, b| a.0.total_cmp(&b.0).then(a.1.cmp(&b.1)));

            for &(_, j) in order_proximity
                .iter()
                .take(ap.nb_granular.min(nb_clients - 1))
            {
                // If i is correlated with j, then j should be correlated with i
                set_correlated_vertices[i].push(j);
                set_correlated_vertices[j].push(i);
            }
        }

        // Filling the vector of correlated vertices (sorted unique values, as the C++ std::set)
        let mut correlated_vertices: Vec<Vec<usize>> = vec![Vec::new(); nb_clients + 1];
        for i in 1..=nb_clients {
            let mut vertices = std::mem::take(&mut set_correlated_vertices[i]);
            vertices.sort_unstable();
            vertices.dedup();
            correlated_vertices[i] = vertices;
        }

        // Safeguards to avoid possible numerical instability in case of instances
        // containing arbitrarily small or large numerical values
        if max_dist < 0.1 || max_dist > 100000.0 {
            return Err("The distances are of very small or large scale. This could impact numerical stability. Please rescale the dataset and run again.".to_string());
        }
        if max_demand < 0.1 || max_demand > 100000.0 {
            return Err("The demand quantities are of very small or large scale. This could impact numerical stability. Please rescale the dataset and run again.".to_string());
        }
        if (nb_vehicles as f64) < (total_demand / vehicle_capacity).ceil() {
            return Err(
                "Fleet size is insufficient to service the considered clients.".to_string(),
            );
        }

        // A reasonable scale for the initial values of the penalties
        let penalty_duration = 1.0;
        let penalty_capacity = (max_dist / max_demand).clamp(0.1, 1000.0);

        if verbose {
            println!(
                "----- INSTANCE SUCCESSFULLY LOADED WITH {} CLIENTS AND {} VEHICLES",
                nb_clients, nb_vehicles
            );
        }

        Ok(Params {
            verbose,
            ap,
            penalty_capacity,
            penalty_duration,
            start_time,
            rng,
            is_duration_constraint,
            nb_clients,
            nb_vehicles,
            duration_limit,
            vehicle_capacity,
            total_demand,
            max_demand,
            max_dist,
            clients,
            time_cost: dist_mtx,
            correlated_vertices,
            are_coordinates_provided,
        })
    }
}
