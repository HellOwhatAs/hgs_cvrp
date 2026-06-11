//! Reader for problem instances following the CVRPLIB conventions.

use crate::matrix::SquareMatrix;

pub struct CvrplibInstance {
    pub x_coords: Vec<f64>,
    pub y_coords: Vec<f64>,
    pub dist_mtx: SquareMatrix,
    pub service_time: Vec<f64>,
    pub demands: Vec<f64>,
    /// Route duration limit.
    pub duration_limit: f64,
    /// Capacity limit.
    pub vehicle_capacity: f64,
    /// Indicates if the problem includes duration constraints.
    pub is_duration_constraint: bool,
    /// Number of clients (excluding the depot).
    pub nb_clients: usize,
}

fn next_token<'a>(tokens: &mut impl Iterator<Item = &'a str>) -> Result<&'a str, String> {
    tokens
        .next()
        .ok_or_else(|| "Unexpected end of input file".to_string())
}

fn parse<T: std::str::FromStr>(token: &str) -> Result<T, String> {
    token
        .parse()
        .map_err(|_| format!("Could not parse value: {}", token))
}

impl CvrplibInstance {
    pub fn read(path: &str, is_rounding_integer: bool) -> Result<CvrplibInstance, String> {
        let content = std::fs::read_to_string(path)
            .map_err(|_| format!("Impossible to open instance file: {}", path))?;

        // The reference implementation skips the first three lines (NAME, COMMENT, TYPE)
        let mut remainder = &content[..];
        for _ in 0..3 {
            match remainder.find('\n') {
                Some(position) => remainder = &remainder[position + 1..],
                None => {
                    remainder = "";
                    break;
                }
            }
        }
        let mut tokens = remainder.split_whitespace();

        // Header section: keyword, separator (":"), value
        let mut nb_clients: Option<usize> = None;
        let mut vehicle_capacity = 1.0e30;
        let mut duration_limit = 1.0e30;
        let mut is_duration_constraint = false;
        let mut service_time_data = 0.0;
        loop {
            let token = next_token(&mut tokens)?;
            match token {
                "NODE_COORD_SECTION" => break,
                "DIMENSION" => {
                    next_token(&mut tokens)?;
                    let dimension: usize = parse(next_token(&mut tokens)?)?;
                    // Need to subtract the depot from the number of nodes
                    nb_clients = Some(dimension.saturating_sub(1));
                }
                "EDGE_WEIGHT_TYPE" => {
                    next_token(&mut tokens)?;
                    next_token(&mut tokens)?;
                }
                "CAPACITY" => {
                    next_token(&mut tokens)?;
                    vehicle_capacity = parse(next_token(&mut tokens)?)?;
                }
                "DISTANCE" => {
                    next_token(&mut tokens)?;
                    duration_limit = parse(next_token(&mut tokens)?)?;
                    is_duration_constraint = true;
                }
                "SERVICE_TIME" => {
                    next_token(&mut tokens)?;
                    service_time_data = parse(next_token(&mut tokens)?)?;
                }
                other => return Err(format!("Unexpected data in input file: {}", other)),
            }
        }
        let nb_clients = match nb_clients {
            Some(n) if n > 0 => n,
            _ => return Err("Number of nodes is undefined".to_string()),
        };
        if vehicle_capacity == 1.0e30 {
            return Err("Vehicle capacity is undefined".to_string());
        }

        // Reading node coordinates: the depot is node 1 in the file (index 0 here),
        // customers are nodes 2, 3, ... (indices 1, 2, ...)
        let mut x_coords = vec![0.0; nb_clients + 1];
        let mut y_coords = vec![0.0; nb_clients + 1];
        for i in 0..=nb_clients {
            let node_number: i64 = parse(next_token(&mut tokens)?)?;
            x_coords[i] = parse(next_token(&mut tokens)?)?;
            y_coords[i] = parse(next_token(&mut tokens)?)?;
            if node_number != i as i64 + 1 {
                return Err("The node numbering is not in order.".to_string());
            }
        }

        // Reading demand information
        let token = next_token(&mut tokens)?;
        if token != "DEMAND_SECTION" {
            return Err(format!("Unexpected data in input file: {}", token));
        }
        let mut demands = vec![0.0; nb_clients + 1];
        let mut service_time = vec![0.0; nb_clients + 1];
        for i in 0..=nb_clients {
            next_token(&mut tokens)?; // node number (not checked, as in the C++ version)
            demands[i] = parse(next_token(&mut tokens)?)?;
            service_time[i] = if i == 0 { 0.0 } else { service_time_data };
        }

        // Calculating the 2D Euclidean distance matrix
        let mut dist_mtx = SquareMatrix::new(nb_clients + 1, 0.0);
        for i in 0..=nb_clients {
            for j in 0..=nb_clients {
                let dx: f64 = x_coords[i] - x_coords[j];
                let dy: f64 = y_coords[i] - y_coords[j];
                let mut distance = (dx * dx + dy * dy).sqrt();
                if is_rounding_integer {
                    distance = distance.round();
                }
                dist_mtx.set(i, j, distance);
            }
        }

        // Reading depot information (the depot is represented as node 1 in all current instances)
        let token = next_token(&mut tokens)?;
        let depot_index = next_token(&mut tokens)?;
        next_token(&mut tokens)?; // "-1" end marker
        let eof_marker = next_token(&mut tokens)?;
        if token != "DEPOT_SECTION" {
            return Err(format!("Unexpected data in input file: {}", token));
        }
        if depot_index != "1" {
            return Err(format!("Expected depot index 1 instead of {}", depot_index));
        }
        if eof_marker != "EOF" {
            return Err(format!("Unexpected data in input file: {}", eof_marker));
        }

        Ok(CvrplibInstance {
            x_coords,
            y_coords,
            dist_mtx,
            service_time,
            demands,
            duration_limit,
            vehicle_capacity,
            is_duration_constraint,
            nb_clients,
        })
    }
}
