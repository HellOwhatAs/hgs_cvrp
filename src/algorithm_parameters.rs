//! User-tunable parameters of the HGS algorithm.

use crate::util::format_double;

#[derive(Clone, Copy)]
pub struct AlgorithmParameters {
    /// Granular search parameter, limits the number of moves in the RI local search.
    pub nb_granular: usize,
    /// Minimum population size.
    pub mu: usize,
    /// Number of solutions created before reaching the maximum population size (i.e., generation size).
    pub lambda: usize,
    /// Number of elite individuals.
    pub nb_elite: usize,
    /// Number of closest solutions/individuals considered when calculating diversity contribution.
    pub nb_close: usize,

    /// Number of iterations between penalty updates.
    pub nb_iter_penalty_management: usize,
    /// Reference proportion of feasible individuals, used for the adaptation of the penalty parameters.
    pub target_feasible: f64,
    /// Multiplier used to decrease penalty parameters if there are sufficient feasible individuals.
    pub penalty_decrease: f64,
    /// Multiplier used to increase penalty parameters if there are insufficient feasible individuals.
    pub penalty_increase: f64,

    /// Random seed.
    pub seed: u64,
    /// Number of iterations without improvement until termination (or restart if a time limit is given).
    pub nb_iter: usize,
    /// Number of iterations between traces display during HGS execution.
    pub nb_iter_traces: usize,
    /// CPU time limit until termination in seconds. 0 means inactive.
    pub time_limit: f64,
    /// Use the SWAP* local search or not. Only available when coordinates are provided.
    pub use_swap_star: bool,
}

impl Default for AlgorithmParameters {
    fn default() -> Self {
        Self {
            nb_granular: 20,
            mu: 25,
            lambda: 40,
            nb_elite: 4,
            nb_close: 5,
            nb_iter_penalty_management: 100,
            target_feasible: 0.2,
            penalty_decrease: 0.85,
            penalty_increase: 1.2,
            seed: 0,
            nb_iter: 20_000,
            nb_iter_traces: 500,
            time_limit: 0.0,
            use_swap_star: true,
        }
    }
}

impl AlgorithmParameters {
    /// Prints all parameter values, mirroring the C++ `print_algorithm_parameters`.
    pub fn print(&self) {
        println!("=========== Algorithm Parameters =================");
        println!(
            "---- nbGranular              is set to {}",
            self.nb_granular
        );
        println!("---- mu                      is set to {}", self.mu);
        println!("---- lambda                  is set to {}", self.lambda);
        println!("---- nbElite                 is set to {}", self.nb_elite);
        println!("---- nbClose                 is set to {}", self.nb_close);
        println!(
            "---- nbIterPenaltyManagement is set to {}",
            self.nb_iter_penalty_management
        );
        println!(
            "---- targetFeasible          is set to {}",
            format_double(self.target_feasible)
        );
        println!(
            "---- penaltyDecrease         is set to {}",
            format_double(self.penalty_decrease)
        );
        println!(
            "---- penaltyIncrease         is set to {}",
            format_double(self.penalty_increase)
        );
        println!("---- seed                    is set to {}", self.seed);
        println!("---- nbIter                  is set to {}", self.nb_iter);
        println!(
            "---- nbIterTraces            is set to {}",
            self.nb_iter_traces
        );
        println!(
            "---- timeLimit               is set to {}",
            format_double(self.time_limit)
        );
        println!(
            "---- useSwapStar             is set to {}",
            self.use_swap_star as i32
        );
        println!("==================================================");
    }
}
