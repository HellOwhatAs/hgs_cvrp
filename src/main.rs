use hgs_cvrp::{export_cvrplib_format, CommandLine, CvrplibInstance, Genetic, Params};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if let Err(message) = run(&args) {
        println!("EXCEPTION | {}", message);
    }
}

fn run(args: &[String]) -> Result<(), String> {
    // Reading the arguments of the program
    let command_line = CommandLine::parse(args)?;

    // Print all algorithm parameter values
    if command_line.verbose {
        command_line.ap.print();
    }

    // Reading the data file and initializing some data structures
    if command_line.verbose {
        println!("----- READING INSTANCE: {}", command_line.path_instance);
    }
    let instance = CvrplibInstance::read(
        &command_line.path_instance,
        command_line.is_rounding_integer,
    )?;

    let params = Params::new(
        &instance.x_coords,
        &instance.y_coords,
        instance.dist_mtx,
        &instance.service_time,
        &instance.demands,
        instance.vehicle_capacity,
        instance.duration_limit,
        command_line.nb_veh,
        instance.is_duration_constraint,
        command_line.verbose,
        command_line.ap,
    )?;

    // Running HGS
    let mut solver = Genetic::new(params);
    solver.run();

    // Exporting the best solution
    if let Some(best) = solver.population.best_found() {
        if solver.params.verbose {
            println!(
                "----- WRITING BEST SOLUTION IN : {}",
                command_line.path_solution
            );
        }
        if export_cvrplib_format(best, &command_line.path_solution).is_err() {
            println!("----- IMPOSSIBLE TO OPEN: {}", command_line.path_solution);
        }
        let _ = solver.population.export_search_progress(
            &format!("{}.PG.csv", command_line.path_solution),
            &command_line.path_instance,
            solver.params.ap.seed,
        );
    }

    Ok(())
}
