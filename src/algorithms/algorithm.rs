

pub mod cpu_algorithms {

    use crate::processes::process::ArrivalProcess;
    use crate::processes::process::DurationProcess;

    
    pub fn fifo(mut processes: crate::helper_functions::PQueueArrival) -> Vec<ArrivalProcess>{
        let mut complete: Vec<ArrivalProcess> = Vec::new();
        let mut cur_time = 0;
    
        // loop until heap is empty
        while !processes.is_empty() {
            // pop off earliest process
            let mut cur_process = processes.pop().unwrap();
    
            // check if it has arrived yet, if not skip to time and push it back onto heap
            if cur_process.arrival <= cur_time {
                cur_process.first_run = cur_time;
                cur_process.completion = cur_time + cur_process.duration;
                cur_time = cur_process.completion;
    
                complete.push(cur_process);
            } else {
                cur_time = cur_process.arrival;
                processes.push(cur_process);
            }
        }
    
        crate::helper_functions::show_processess(&complete);
        println!("avg FIFO turnaround: {}, avg FIFO response: {}", crate::helper_functions::avg_turnaround(&complete), crate::helper_functions::avg_response(&complete));
    
        return complete;
    }
    
    pub fn rr(mut processes:  crate::helper_functions::PQueueArrival) -> Vec<ArrivalProcess> {
        let mut complete: Vec<ArrivalProcess> = Vec::new();
        let mut cur_time = 0;
    
        while !processes.is_empty() {
            let mut cur: Vec<ArrivalProcess> = Vec::new();
            let mut i = 0;
    
            while !processes.is_empty() {
                let mut cur_process = processes.pop().unwrap();
    
                if cur_process.arrival < cur_time || cur_process.arrival == 0 {
    
                    if cur_process.first_run == -1 {
                        cur_process.first_run = cur_time;
                    }
    
                    cur_process.duration -= 1;
                    cur_time += 1;
                    i += 1;
    
                    if cur_process.duration == 0 {
                        cur_process.completion = cur_time;
                        complete.push(cur_process);
                    } else {
                        cur.push(cur_process);
                    }
                }
            }
    
            for proc in cur {
                processes.push(proc);
            }
    
            if i == 0 {
                cur_time += 1;
            }
        }
    
        crate::helper_functions::show_processess(&complete);
        println!("avg RR turnaround: {}, avg RR response: {}", crate::helper_functions::avg_turnaround(&complete), crate::helper_functions::avg_response(&complete));
    
        return complete;
    }
    
    pub fn sjf (mut processes: crate::helper_functions::PQueueArrival) -> Vec<DurationProcess> {
        let mut complete: Vec<DurationProcess> = Vec::new();
        let mut workload: crate::helper_functions::PQueueDuration = crate::helper_functions::PQueueDuration::new();
        let mut cur_time = 0;
    
        while !processes.is_empty() {
            let old_proc = processes.pop().unwrap();
            let new_proc = DurationProcess { arrival: old_proc.arrival, first_run: old_proc.first_run, duration: old_proc.duration, completion: old_proc.completion};
            workload.push(new_proc);    
        }
    
        crate::helper_functions::d_show_workload(workload.clone());
    
        while !workload.is_empty() {
            let mut cur: Vec<DurationProcess> = Vec::new();
            let mut i = 0;
    
            while !workload.is_empty() {
                let mut cur_proc = workload.pop().unwrap();
    
                if cur_proc.arrival <= cur_time {
                    cur_proc.first_run = cur_time;
                    cur_proc.completion = cur_time + cur_proc.duration;
                    cur_time = cur_proc.completion;
                    
                    i += 1;
                    complete.push(cur_proc);
                } else {
                    cur.push(cur_proc);
                }
            }
    
            for proc in cur {
                workload.push(proc);
            }
    
            if i == 0 {
                cur_time += 1;
            }
    
            
        }
    
    
        crate::helper_functions::show_processess(&complete);
        println!("avg SJF turnaround: {}, avg SJF response: {}", crate::helper_functions::avg_turnaround(&complete), crate::helper_functions::avg_response(&complete));
        return complete;
    }
    
    pub fn stcf(mut processes: crate::helper_functions::PQueueArrival) -> Vec<DurationProcess> {
        let mut complete: Vec<DurationProcess> = Vec::new();
        let mut workload: crate::helper_functions::PQueueDuration = crate::helper_functions::PQueueDuration::new();
        let mut cur_time = 0;
    
        while !processes.is_empty() {
            let old_proc = processes.pop().unwrap();
            let new_proc = DurationProcess {arrival: old_proc.arrival, first_run: old_proc.first_run, duration: old_proc.duration, completion: old_proc.completion};
            workload.push(new_proc);
        }
    
        while !workload.is_empty() {
            let mut cur: Vec<DurationProcess> = Vec::new();
            let mut i = 0;
            let mut flag = false;
    
            while !workload.is_empty() {
                let mut cur_proc = workload.pop().unwrap();
    
                if cur_proc.arrival <= cur_time && !flag {
                    if cur_proc.first_run == -1 {
                        cur_proc.first_run = cur_time;
                    }
                    flag = true;
    
                    cur_proc.duration -= 1;
                    cur_time += 1;
                    i += 1;
    
                    if cur_proc.duration == 0 {
                        cur_proc.completion = cur_time;
                        complete.push(cur_proc);
                    } else {
                        cur.push(cur_proc);
                    }
    
                } else {
                    cur.push(cur_proc);
                }
            } 
    
            for proc in cur {
                workload.push(proc);
            }
    
            if i == 0 {
                cur_time += 1;
            }
        }
    
        crate::helper_functions::show_processess(&complete);
        println!("avg STCF turnaround: {}, avg STCF response: {}", crate::helper_functions::avg_turnaround(&complete), crate::helper_functions::avg_response(&complete));
        return complete;
    }
    

}


