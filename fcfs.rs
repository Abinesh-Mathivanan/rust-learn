#[derive(Debug)]
struct Process {
    id: u32,
    arrival_time: u32,
    burst_time: u32,
    completion_time: u32,
    turnaround_time: u32,
    waiting_time: u32,
}

impl Process {
    fn new(id: u32, arrival_time: u32, burst_time: u32) -> Self {
        Process {
            id,
            arrival_time,
            burst_time,
            completion_time: 0,
            turnaround_time: 0,
            waiting_time: 0,
        }
    }
}

fn fcfs_scheduling(mut processes: Vec<Process>) -> Vec<Process> {
    processes.sort_by_key(|p| p.arrival_time);
    
    let mut current_time = 0;
    
    for process in processes.iter_mut() {
        if current_time < process.arrival_time {
            current_time = process.arrival_time;
        }
        
        process.completion_time = current_time + process.burst_time;
        process.turnaround_time = process.completion_time - process.arrival_time;
        process.waiting_time = process.turnaround_time - process.burst_time;
        
        current_time = process.completion_time;
    }
    
    processes
}

fn main() {
    let processes = vec![
        Process::new(1, 0, 6),
        Process::new(2, 2, 4),
        Process::new(3, 4, 2),
    ];
    
    let scheduled_processes = fcfs_scheduling(processes);
    
    println!("Process ID | Arrival Time | Burst Time | Completion Time | Turnaround Time | Waiting Time");
    println!("---------------------------------------------------------------------------");
    
    for p in scheduled_processes {
        println!("{:10} | {:12} | {:10} | {:15} | {:15} | {:12}",
            p.id, p.arrival_time, p.burst_time, p.completion_time, p.turnaround_time, p.waiting_time);
    }
}