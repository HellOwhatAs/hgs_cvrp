//! Command line parsing for the `hgs` executable.

use crate::algorithm_parameters::AlgorithmParameters;

pub struct CommandLine {
    pub ap: AlgorithmParameters,
    /// Number of vehicles. `None` lets the algorithm compute a reasonable bound.
    pub nb_veh: Option<usize>,
    /// Instance path.
    pub path_instance: String,
    /// Solution path.
    pub path_solution: String,
    pub verbose: bool,
    pub is_rounding_integer: bool,
}

impl CommandLine {
    /// Reads the command line (including the program name in `args[0]`)
    /// and extracts the possible options.
    pub fn parse(args: &[String]) -> Result<CommandLine, String> {
        if args.len() % 2 != 1 || args.len() > 35 || args.len() < 3 {
            println!(
                "----- NUMBER OF COMMANDLINE ARGUMENTS IS INCORRECT: {}",
                args.len()
            );
            display_help();
            return Err("Incorrect line of command".to_string());
        }

        let mut command_line = CommandLine {
            ap: AlgorithmParameters::default(),
            nb_veh: None,
            path_instance: args[1].clone(),
            path_solution: args[2].clone(),
            verbose: true,
            is_rounding_integer: true,
        };

        let mut i = 3;
        while i < args.len() {
            let option = args[i].as_str();
            let value = args[i + 1].as_str();
            let result: Result<(), String> = match option {
                "-t" => parse_into(value, &mut command_line.ap.time_limit),
                "-it" => parse_into(value, &mut command_line.ap.nb_iter),
                "-seed" => parse_into(value, &mut command_line.ap.seed),
                "-veh" => {
                    let veh: usize = parse(value)?;
                    command_line.nb_veh = Some(veh);
                    Ok(())
                }
                "-round" => parse_bool_into(value, &mut command_line.is_rounding_integer),
                "-log" => parse_bool_into(value, &mut command_line.verbose),
                "-nbGranular" => parse_into(value, &mut command_line.ap.nb_granular),
                "-mu" => parse_into(value, &mut command_line.ap.mu),
                "-lambda" => parse_into(value, &mut command_line.ap.lambda),
                "-nbElite" => parse_into(value, &mut command_line.ap.nb_elite),
                "-nbClose" => parse_into(value, &mut command_line.ap.nb_close),
                "-nbIterPenaltyManagement" => {
                    parse_into(value, &mut command_line.ap.nb_iter_penalty_management)
                }
                "-nbIterTraces" => parse_into(value, &mut command_line.ap.nb_iter_traces),
                "-targetFeasible" => parse_into(value, &mut command_line.ap.target_feasible),
                "-penaltyIncrease" => parse_into(value, &mut command_line.ap.penalty_increase),
                "-penaltyDecrease" => parse_into(value, &mut command_line.ap.penalty_decrease),
                _ => {
                    println!("----- ARGUMENT NOT RECOGNIZED: {}", option);
                    display_help();
                    return Err("Incorrect line of command".to_string());
                }
            };
            result?;
            i += 2;
        }

        Ok(command_line)
    }
}

fn parse<T: std::str::FromStr>(value: &str) -> Result<T, String> {
    value
        .parse()
        .map_err(|_| format!("Could not parse argument value: {}", value))
}

fn parse_into<T: std::str::FromStr>(value: &str, target: &mut T) -> Result<(), String> {
    *target = parse(value)?;
    Ok(())
}

fn parse_bool_into(value: &str, target: &mut bool) -> Result<(), String> {
    let as_int: i64 = parse(value)?;
    *target = as_int != 0;
    Ok(())
}

/// Prints information about how to use the code.
pub fn display_help() {
    println!();
    println!("-------------------------------------------------- HGS-CVRP algorithm (2020) ---------------------------------------------------");
    println!("Call with: ./hgs instancePath solPath [-it nbIter] [-t myCPUtime] [-seed mySeed] [-veh nbVehicles] [-log verbose]               ");
    println!("[-it <int>] sets a maximum number of iterations without improvement. Defaults to 20,000                                         ");
    println!("[-t <double>] sets a time limit in seconds. If this parameter is set the code will be run iteratively until the time limit      ");
    println!("[-seed <int>] sets a fixed seed. Defaults to 0                                                                                  ");
    println!("[-veh <int>] sets a prescribed fleet size. Otherwise a reasonable UB on the the fleet size is calculated                        ");
    println!("[-round <bool>] rounding the distance to the nearest integer or not. It can be 0 (not rounding) or 1 (rounding). Defaults to 1. ");
    println!("[-log <bool>] sets the verbose level of the algorithm log. It can be 0 or 1. Defaults to 1.                                     ");
    println!();
    println!("Additional Arguments:                                                                                                           ");
    println!("[-nbIterTraces <int>] Number of iterations between traces display during HGS execution. Defaults to 500                         ");
    println!("[-nbGranular <int>] Granular search parameter, limits the number of moves in the RI local search. Defaults to 20                ");
    println!("[-mu <int>] Minimum population size. Defaults to 25                                                                             ");
    println!("[-lambda <int>] Number of solutions created before reaching the maximum population size (i.e., generation size). Defaults to 40 ");
    println!("[-nbElite <int>] Number of elite individuals. Defaults to 5                                                                     ");
    println!("[-nbClose <int>] Number of closest solutions/individuals considered when calculating diversity contribution. Defaults to 4      ");
    println!("[-nbIterPenaltyManagement <int>] Number of iterations between penalty updates. Defaults to 100                                  ");
    println!("[-targetFeasible <double>] target ratio of feasible individuals between penalty updates. Defaults to 0.2                        ");
    println!("[-penaltyIncrease <double>] penalty increase if insufficient feasible individuals between penalty updates. Defaults to 1.2      ");
    println!("[-penaltyDecrease <double>] penalty decrease if sufficient feasible individuals between penalty updates. Defaults to 0.85       ");
    println!("--------------------------------------------------------------------------------------------------------------------------------");
    println!();
}
