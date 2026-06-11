//! Main procedures of the genetic algorithm, including the OX crossover.

use crate::individual::Individual;
use crate::local_search::LocalSearch;
use crate::params::Params;
use crate::population::Population;
use crate::split::Split;
use crate::util::format_double;

pub struct Genetic {
    pub params: Params,
    split: Split,
    local_search: LocalSearch,
    pub population: Population,
    /// Individual used as scratch space for the crossover result.
    offspring: Individual,
}

impl Genetic {
    pub fn new(mut params: Params) -> Self {
        let split = Split::new(&params);
        let local_search = LocalSearch::new(&params);
        let population = Population::new(&mut params);
        let offspring = Individual::new(&mut params);
        Self {
            params,
            split,
            local_search,
            population,
            offspring,
        }
    }

    /// Runs the genetic algorithm until nb_iter consecutive iterations
    /// without improvement or a time limit.
    pub fn run(&mut self) {
        /* INITIAL POPULATION */
        self.population.generate_population(
            &mut self.params,
            &mut self.split,
            &mut self.local_search,
        );

        let mut nb_iter: usize = 0;
        let mut nb_iter_non_prod: usize = 1;
        if self.params.verbose {
            println!("----- STARTING GENETIC ALGORITHM");
        }
        while nb_iter_non_prod <= self.params.ap.nb_iter
            && (self.params.ap.time_limit == 0.0
                || self.params.start_time.elapsed().as_secs_f64() < self.params.ap.time_limit)
        {
            /* SELECTION AND CROSSOVER */
            let parent1 = self.population.get_binary_tournament(&mut self.params);
            let parent2 = self.population.get_binary_tournament(&mut self.params);
            crossover_ox(
                &mut self.offspring,
                self.population.individual(parent1),
                self.population.individual(parent2),
                &mut self.params,
                &mut self.split,
            );

            /* LOCAL SEARCH */
            let (penalty_capacity, penalty_duration) =
                (self.params.penalty_capacity, self.params.penalty_duration);
            self.local_search.run(
                &mut self.params,
                &mut self.offspring,
                penalty_capacity,
                penalty_duration,
            );
            let mut is_new_best =
                self.population
                    .add_individual(&self.offspring, true, &self.params);
            if !self.offspring.eval.is_feasible && self.params.rng.next() % 2 == 0 {
                // Repair half of the solutions in case of infeasibility
                self.local_search.run(
                    &mut self.params,
                    &mut self.offspring,
                    penalty_capacity * 10.0,
                    penalty_duration * 10.0,
                );
                if self.offspring.eval.is_feasible {
                    is_new_best =
                        self.population
                            .add_individual(&self.offspring, false, &self.params)
                            || is_new_best;
                }
            }

            /* TRACKING THE NUMBER OF ITERATIONS SINCE LAST SOLUTION IMPROVEMENT */
            if is_new_best {
                nb_iter_non_prod = 1;
            } else {
                nb_iter_non_prod += 1;
            }

            /* DIVERSIFICATION, PENALTY MANAGEMENT AND TRACES */
            if nb_iter % self.params.ap.nb_iter_penalty_management == 0 {
                self.population.manage_penalties(&mut self.params);
            }
            if nb_iter % self.params.ap.nb_iter_traces == 0 {
                self.population
                    .print_state(nb_iter, nb_iter_non_prod, &self.params);
            }

            /* FOR TESTS INVOLVING SUCCESSIVE RUNS UNTIL A TIME LIMIT: WE RESET THE ALGORITHM/POPULATION EACH TIME maxIterNonProd IS ATTAINED */
            if self.params.ap.time_limit != 0.0 && nb_iter_non_prod == self.params.ap.nb_iter {
                self.population
                    .restart(&mut self.params, &mut self.split, &mut self.local_search);
                nb_iter_non_prod = 1;
            }

            nb_iter += 1;
        }
        if self.params.verbose {
            println!(
                "----- GENETIC ALGORITHM FINISHED AFTER {} ITERATIONS. TIME SPENT: {}",
                nb_iter,
                format_double(self.params.start_time.elapsed().as_secs_f64())
            );
        }
    }
}

/// OX Crossover: copies a random fragment of parent1 and fills the rest in the
/// order given by parent2, then completes the individual with the Split algorithm.
fn crossover_ox(
    result: &mut Individual,
    parent1: &Individual,
    parent2: &Individual,
    params: &mut Params,
    split: &mut Split,
) {
    let nb_clients = params.nb_clients;

    // Frequency table to track the customers which have been already inserted
    let mut freq_client = vec![false; nb_clients + 1];

    // Picking the beginning and end of the crossover zone
    let start = params.rng.uniform_below(nb_clients as u32) as usize;
    let mut end = params.rng.uniform_below(nb_clients as u32) as usize;

    // Avoid that start and end coincide by accident
    while end == start {
        end = params.rng.uniform_below(nb_clients as u32) as usize;
    }

    // Copy from start to end
    let mut j = start;
    while j % nb_clients != (end + 1) % nb_clients {
        result.chrom_t[j % nb_clients] = parent1.chrom_t[j % nb_clients];
        freq_client[result.chrom_t[j % nb_clients]] = true;
        j += 1;
    }

    // Fill the remaining elements in the order given by the second parent
    for i in 1..=nb_clients {
        let temp = parent2.chrom_t[(end + i) % nb_clients];
        if !freq_client[temp] {
            result.chrom_t[j % nb_clients] = temp;
            j += 1;
        }
    }

    // Complete the individual with the Split algorithm
    split.general_split(params, result, parent1.eval.nb_routes);
}
