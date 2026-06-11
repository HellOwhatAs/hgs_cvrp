//! HGS-CVRP: A Rust implementation of the Hybrid Genetic Search for the
//! Capacitated Vehicle Routing Problem, with the SWAP* neighborhood.
//!
//! This is a faithful port of the C++ reference implementation by Thibaut Vidal
//! (<https://github.com/vidalt/HGS-CVRP>, MIT license), described in:
//!
//! - Vidal, T., Crainic, T. G., Gendreau, M., Lahrichi, N., Rei, W. (2012).
//!   A hybrid genetic algorithm for multidepot and periodic vehicle routing problems.
//!   Operations Research, 60(3), 611-624.
//! - Vidal, T. (2022). Hybrid genetic search for the CVRP: Open-source implementation
//!   and SWAP* neighborhood. Computers & Operations Research, 140, 105643.
//!
//! # Example
//!
//! ```no_run
//! use hgs_cvrp::{AlgorithmParameters, CvrplibInstance, Genetic, Params};
//!
//! let instance = CvrplibInstance::read("instance.vrp", true).unwrap();
//! let params = Params::new(
//!     &instance.x_coords, &instance.y_coords, instance.dist_mtx,
//!     &instance.service_time, &instance.demands,
//!     instance.vehicle_capacity, instance.duration_limit,
//!     None, instance.is_duration_constraint, true,
//!     AlgorithmParameters::default(),
//! ).unwrap();
//! let mut solver = Genetic::new(params);
//! solver.run();
//! if let Some(best) = solver.population.best_found() {
//!     println!("best cost: {}", best.eval.penalized_cost);
//! }
//! ```

// These stylistic lints are deliberately not followed: the code intentionally
// mirrors the structure of the C++ reference implementation for traceability.
#![allow(clippy::manual_is_multiple_of)]
#![allow(clippy::manual_range_contains)]
#![allow(clippy::manual_rem_euclid)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::should_implement_trait)]

pub mod algorithm_parameters;
pub mod circle_sector;
pub mod cli;
pub mod cvrplib;
pub mod genetic;
pub mod individual;
pub mod local_search;
pub mod matrix;
pub mod params;
pub mod population;
pub mod rng;
pub mod split;
pub mod util;

pub use algorithm_parameters::AlgorithmParameters;
pub use cli::CommandLine;
pub use cvrplib::CvrplibInstance;
pub use genetic::Genetic;
pub use individual::{export_cvrplib_format, EvalIndiv, Individual};
pub use local_search::LocalSearch;
pub use matrix::SquareMatrix;
pub use params::{Client, Params};
pub use population::Population;
pub use split::Split;
pub use util::format_double;
