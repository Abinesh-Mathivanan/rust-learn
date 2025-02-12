struct Process {
    id: u32,
    burst_time: u32,
    arrival_time: u32,
}

fn sjf_scheduling(mut processes: Vec<Process>) -> Vec<Process> {
    processes.sort_by(|a, b| {
        if a.arrival_time == b.arrival_time {
            a.burst_time.cmp(&b.burst_time)
        } else {
            a.arrival_time.cmp(&b.arrival_time)
        }
    });

    let mut current_time = 0;
    let mut result = Vec::new();
    let mut ready_queue = Vec::new();

    while !processes.is_empty() || !ready_queue.is_empty() {
        while let Some(pos) = processes
            .iter()
            .position(|p| p.arrival_time <= current_time)
        {
            ready_queue.push(processes.remove(pos));
        }

        if ready_queue.is_empty() {
            current_time = processes[0].arrival_time;
            continue;
        }

        ready_queue.sort_by_key(|p| p.burst_time);
        let next_process = ready_queue.remove(0);
        current_time += next_process.burst_time;
        result.push(next_process);
    }

    result
}

fn main() {
    let processes = vec![
        Process { id: 1, burst_time: 6, arrival_time: 0 },
        Process { id: 2, burst_time: 2, arrival_time: 1 },
        Process { id: 3, burst_time: 8, arrival_time: 2 },
        Process { id: 4, burst_time: 3, arrival_time: 3 },
    ];

    let scheduled = sjf_scheduling(processes);
    
    for process in scheduled {
        println!("Process {}: burst_time={}, arrival_time={}", 
                process.id, process.burst_time, process.arrival_time);
    }
}