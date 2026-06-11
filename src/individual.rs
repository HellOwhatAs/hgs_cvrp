//! Representation of an individual solution in the genetic algorithm.

use crate::params::Params;
use crate::util::{format_double, MY_EPSILON};

#[derive(Clone, Default)]
pub struct EvalIndiv {
    /// Penalized cost of the solution.
    pub penalized_cost: f64,
    /// Number of routes.
    pub nb_routes: usize,
    /// Total distance.
    pub distance: f64,
    /// Sum of excess load in all routes.
    pub capacity_excess: f64,
    /// Sum of excess duration in all routes.
    pub duration_excess: f64,
    /// Feasibility status of the individual.
    pub is_feasible: bool,
}

#[derive(Clone)]
pub struct Individual {
    /// Solution cost parameters.
    pub eval: EvalIndiv,
    /// Giant tour representing the individual.
    pub chrom_t: Vec<usize>,
    /// For each vehicle, the associated sequence of deliveries (complete solution).
    pub chrom_r: Vec<Vec<usize>>,
    /// For each node, the successor in the solution (can be the depot 0).
    pub successors: Vec<usize>,
    /// For each node, the predecessor in the solution (can be the depot 0).
    pub predecessors: Vec<usize>,
}

impl Individual {
    /// Creates a random individual containing only a giant tour with a shuffled visit order.
    pub fn new(params: &mut Params) -> Self {
        let mut chrom_t: Vec<usize> = (1..=params.nb_clients).collect();
        params.rng.shuffle(&mut chrom_t);
        Self {
            eval: EvalIndiv {
                penalized_cost: 1.0e30,
                ..EvalIndiv::default()
            },
            chrom_t,
            chrom_r: vec![Vec::new(); params.nb_vehicles],
            successors: vec![0; params.nb_clients + 1],
            predecessors: vec![0; params.nb_clients + 1],
        }
    }

    /// Measures cost and feasibility of the individual from the information of chrom_r.
    pub fn evaluate_complete_cost(&mut self, params: &Params) {
        self.eval = EvalIndiv::default();
        for route in &self.chrom_r {
            if route.is_empty() {
                continue;
            }
            let mut distance = params.time_cost.get(0, route[0]);
            let mut load = params.clients[route[0]].demand;
            let mut service = params.clients[route[0]].service_duration;
            self.predecessors[route[0]] = 0;
            for i in 1..route.len() {
                distance += params.time_cost.get(route[i - 1], route[i]);
                load += params.clients[route[i]].demand;
                service += params.clients[route[i]].service_duration;
                self.predecessors[route[i]] = route[i - 1];
                self.successors[route[i - 1]] = route[i];
            }
            self.successors[route[route.len() - 1]] = 0;
            distance += params.time_cost.get(route[route.len() - 1], 0);
            self.eval.distance += distance;
            self.eval.nb_routes += 1;
            if load > params.vehicle_capacity {
                self.eval.capacity_excess += load - params.vehicle_capacity;
            }
            if distance + service > params.duration_limit {
                self.eval.duration_excess += distance + service - params.duration_limit;
            }
        }

        self.eval.penalized_cost = self.eval.distance
            + self.eval.capacity_excess * params.penalty_capacity
            + self.eval.duration_excess * params.penalty_duration;
        self.eval.is_feasible =
            self.eval.capacity_excess < MY_EPSILON && self.eval.duration_excess < MY_EPSILON;
    }
}

/// Exports an individual to a file in CVRPLib format.
pub fn export_cvrplib_format(indiv: &Individual, path: &str) -> std::io::Result<()> {
    let mut output = String::new();
    for (k, route) in indiv.chrom_r.iter().enumerate() {
        if !route.is_empty() {
            // Route IDs start at 1 in the file format
            output.push_str(&format!("Route #{}:", k + 1));
            for &i in route {
                output.push_str(&format!(" {}", i));
            }
            output.push('\n');
        }
    }
    output.push_str(&format!(
        "Cost {}\n",
        format_double(indiv.eval.penalized_cost)
    ));
    std::fs::write(path, output)
}
