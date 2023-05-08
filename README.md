# rust-cpu-scheduler

# Description and How To Use
This is a program that immitates how a CPU could schedule processes. It has the ability to replicate FIFO, RR, STCF, and SJF. 
It will also display the average response time and average turnaround time after simulating the algorithm. To use it, you can 
download this repo and type "cargo run [algorithm name] [path to workload file]" where the workload file is a txt file formatted 
as [arrival time] [duration] on each line.

# Documentation
## Processes Module
- ArrivalProcess
  - Struct that contains 4 variables
    - arrival: i32
    - first_run: i32
    - duration: i32
    - completion: i32
  - Ordering on this struct is implemented in a way so that it is sorted by arrival time in ascending order, in case of a tie breaker it defaults to the shortest duration time
- DurationProcess
  - Struct that contains 4 variables
    - arrival: i32
    - first_run: i32
    - duration: i32
    - completion: i32
  - Ordering on this struct is implemented in a way so that it is sorted by duration time in ascending order, in case of a tie breaker it defaults to the shortest arrival time
## Helpers Module
- trait ProcessData
  - Trait that contains 4 functions
  - These 4 functions each return one of the variables contained in either ArrivalProcess or DurationProcess
  - Macro on ProcessData
    - Needed so the helper functions can a take a general type T (either ArrivalProcess or DurationProcess) and extract the data from the structs
- type PQueueArrival
  - BinaryHeap that contains ArrivalProccesses
- type PQueueDuration
  - BinaryHeap that contains DurationProcesses
- fn read_lines
  - Takes in a path to a file, opens the file, and returns an iterator that iterates over the lines of the file
  - Input
    - &String
  - Ouput
    - io::Lines<BufReader<File>>
- fn read_workload
  - Takes in a path to a file and returns a PQueueArrival based on the workload file
  - Input
    - &String
  - Output
    - PQueueArrival
- fn show_workload
  - Takes in a PQueueArrival and displays the workload
  - Input
    - PQueueArrival
  - Output
    - void
- fn d_show_workload
  - Takes in a PQueueDuration and displays the workload
  - Input
    - PQueueDuration
  - Output
    - void
- fn show_processes
  - Takes in a vector of Arrival or Duration processes and displays them
  - Input
    - &Vec<T>
  - Output
    - void
- fn avg_turnaround
  - Takes in a vector of Arrival or Duration processes and displays the average turnaround time
  - Input
    - &Vec<T>
  - Output
    - void
- fn avg_response
  - Takes in a vector of Arrival or Duration processes and displays the average response time
  - Input
    - &Vec<T>
  - Output
    - void
## Algorithms Module
-  fn fifo
    - Takes in a PQueueArrival, performs fifo on the workload, and returns a vector of completed ArrivalProcess. Also displays the algorithms average response and turnaround times
    - Input
      - PQueueArrival
    - Output
      - Vec<ArrivalProcess>
- fn rr
  - Takes in a PQueueArrival, performs rr on the workload, and returns a vector of completed ArrivalProcess. Also displays the algorithms average response and turnaround times
  - Input
    - PQueueArrival
  - Output
    - Vec<ArrivalProcess>
- fn sjf
  - Takes in a PQueueArrival, performs sjf on the workload, and returns a vector of completed DurationProcess. Also displays the algorithms average response and turnaround times
  - Input
    - PQueueArrival
  - Output
    - Vec<DurationProcess>
- fn stcf
  - Takes in a PQueueArrival, performs stcf on the workload, and returns a vector of completed DurationProcess. Also displays the algorithms average response and turnaround times
  - Input
    - PQueueArrival
  - Output
    - Vec<DurationProcess>
 ## Main function
 - Reads in the algorithm type and workload file path from the command line
   - If more than 2 arguments are provided, program will output the correct format and then exit
 - Attempts to create a PQueueArrival based on the filepath
   - If file path is invalid, program will notify user and exit
 - Displays the PQueueArrival workload that was created
 - Runs a scheduling algorithm based off the input provided by the user
   - If algorithm is invalid, program will notify user, output the correct format, and then exit


    
