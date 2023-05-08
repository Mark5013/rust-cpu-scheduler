

pub mod helper_functions {
    
    use std::fs::File;
    use std::io::{ self, BufRead, BufReader };
    use std::path::Path;
    use std::collections::BinaryHeap;
    use crate::processes::process::ArrivalProcess;
    use crate::processes::process::DurationProcess;
    
    
    pub trait ProcessData {
        type Output;
        fn arrival_data(&self) -> &Self::Output;
        fn duration_data(&self) -> &Self::Output;
        fn first_run_data(&self) -> &Self::Output;
        fn completion_data(&self) -> &Self::Output;
    }
    
    macro_rules! impl_ArrivalData {
        ($out:ty, [$($t:ty),+]) => {
            $(impl ProcessData for $t {
                type Output = $out;
                fn arrival_data(&self) -> &Self::Output {
                    &self.arrival
                }
                fn duration_data(&self) -> &Self::Output {
                    &self.duration
                }
                fn first_run_data(&self) -> &Self::Output {
                    &self.first_run
                }
                fn completion_data(&self) -> &Self::Output {
                    &self.completion
                }
            })*
        }
    }
    
    impl_ArrivalData!{i32, [ArrivalProcess, DurationProcess]}
    
    // arrival Prio queue
    pub type PQueueArrival = BinaryHeap<ArrivalProcess>;
    
    // duration prio queue
    pub type PQueueDuration = BinaryHeap<DurationProcess>;
    
    
    
    
    pub fn read_lines(filename: &String) -> io::Lines<BufReader<File>> {
        // create path for file
        let path = Path::new(filename);
    
        // open file in read only mode, if file path is wrong tell user
        let file = File::open(path).expect("Invalid file path");
        
        // Read the file line by line, the return iterator
        return io::BufReader::new(file).lines();
    }
    
    // convert input file into workload heap and return
    pub fn read_workload(filename: &String) -> PQueueArrival {
        // create workload heap
        let mut workload = PQueueArrival::new();
    
    
        // get lines from file
        let lines = read_lines(filename);
    
        // loop through lines and create processes and push into heap
        for line in lines {
            let unwrapped_line = line.unwrap();
            let nums = unwrapped_line.split_whitespace();
            let mut idx = 0;
            let mut cur_process = ArrivalProcess { arrival: -1, first_run: -1, duration: -1, completion: -1};
    
            for num in nums {
                if idx == 0 {
                    cur_process.arrival = num.parse().unwrap();
                } else {
                    cur_process.duration = num.parse().unwrap();
                }
                idx = (idx + 1) % 2;
            }
            workload.push(cur_process);
        }
    
        workload
    }
    
    // show workload 
    pub fn show_workload(mut workload: PQueueArrival) {
        println!("Workload: ");
    
        while !workload.is_empty() {
            let cur_process = workload.pop().unwrap();
            println!("arrival: {}, duration: {}", cur_process.arrival, cur_process.duration);
        }
            
    }
    
    // shows processes
    pub fn show_processess<T: ProcessData<Output=i32>>(processes: &Vec<T>) {
        println!("Proccesses: ");
    
        for process in processes {
            println!("arrival={}, duration={}, first_run={}, completion={}", process.arrival_data(), process.duration_data(), process.first_run_data(), process.completion_data());
        }
    }
    
    // computes average turnaround
    pub fn avg_turnaround<T: ProcessData<Output=i32>>(processes: &Vec<T>) -> f64 {
        let mut total_jobs = 0;
        let mut total_duration = 0;
    
        for proc in processes {
            total_jobs += 1;
            total_duration += proc.completion_data() - proc.arrival_data();
        }
    
        return total_duration as f64 / total_jobs as f64;
    }
    
    // computes average response
    pub fn avg_response<T: ProcessData<Output=i32>>(processes: &Vec<T>) -> f64 {
        let mut total_jobs = 0;
        let mut total_response = 0;
    
        for proc in processes {
            total_jobs += 1;
            total_response += proc.first_run_data() - proc.arrival_data();
        }
    
        return total_response as f64 / total_jobs as f64;
    }
    
    
    // show workload 
    pub fn d_show_workload(mut workload: PQueueDuration) {
        println!("Workload: ");
    
        while !workload.is_empty() {
            let cur_process = workload.pop().unwrap();
            println!("arrival: {}, duration: {}", cur_process.arrival, cur_process.duration);
        }
            
    }
    

}




