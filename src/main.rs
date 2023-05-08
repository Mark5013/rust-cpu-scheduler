use std::env;
use std::process;
use crate::helpers::helper::helper_functions;
use crate::algorithms::algorithm::cpu_algorithms;

pub mod algorithms;
pub mod helpers;
pub mod processes;

fn main() {
    // extract args from command line
    let args: Vec<_> = env::args().collect();

    // if more or less than 2 args from command line are received print usage and exit
    if args.len() != 3 {
        println!("Usage: [fifo|sjf|stcf|rr] workload_file");
        process::exit(1); 
    }

    let algorithm = &args[1];
    let workload_file = &args[2];

    // create queue workload later...
    let pq_arrival = crate::helper_functions::read_workload(workload_file);
    crate::helper_functions::show_workload(pq_arrival.clone());

    if algorithm == "fifo" {
        crate::cpu_algorithms::fifo(pq_arrival);
    } else if algorithm == "sjf" {
        crate::cpu_algorithms::sjf(pq_arrival);
    } else if algorithm =="stcf" {
         crate::cpu_algorithms::stcf(pq_arrival);
    } else if algorithm == "rr" {
         crate::cpu_algorithms::rr(pq_arrival);
    } else {
        println!("Error: Unkown algorithm: ");
        println!("Usage: [fifo|sjf|stcf|rr] workload_file");
        process::exit(1); 
    }

    // end
}
