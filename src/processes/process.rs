// arrival process 

#[derive(Clone)]
#[derive(Debug)]
pub struct ArrivalProcess {
    pub arrival: i32,
    pub first_run: i32,
    pub duration: i32,
    pub completion: i32,
}

impl std::cmp::Ord for ArrivalProcess {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.arrival != other.arrival {
            other.arrival.cmp(&self.arrival)
        } else {
            other.duration.cmp(&self.duration)
        }
    }
}

impl std::cmp::PartialOrd for ArrivalProcess {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl std::cmp::Eq for ArrivalProcess {}

impl std::cmp::PartialEq for ArrivalProcess {
    fn eq(&self, other: &Self) -> bool {
        self.arrival == other.arrival && self.duration == other.duration
    }
}


// duration proccesses

#[derive(Clone)]
#[derive(Debug)]
pub struct DurationProcess {
    pub arrival: i32,
    pub first_run: i32,
    pub duration: i32,
    pub completion: i32,
}

impl std::cmp::Ord for DurationProcess {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.duration != other.duration {
            other.duration.cmp(&self.duration)
        } else {
            other.arrival.cmp(&self.arrival)
        }
    }
}


impl std::cmp::PartialOrd for DurationProcess {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl std::cmp::Eq for DurationProcess {}

impl std::cmp::PartialEq for DurationProcess {
    fn eq(&self, other: &Self) -> bool {
        self.duration == other.duration && self.arrival == other.arrival
    }
}



