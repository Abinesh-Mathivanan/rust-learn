struct Process {
    id: i32,
    burst_time: i32,
    arrival_time: i32,
}

fn sbtf_scheduling(mut processes: Vec<Process>) -> Vec<Process> {
    let mut current_time = 0;
    let mut completed = Vec::new();
    processes.sort_by_key(|p| p.arrival_time);

    while !processes.is_empty() {
        let available: Vec<&Process> = processes
            .iter()
            .filter(|p| p.arrival_time <= current_time)
            .collect();

        if available.is_empty() {
            current_time = processes[0].arrival_time;
            continue;
        }

        let shortest = available
            .iter()
            .min_by_key(|p| p.burst_time)
            .unwrap();

        let index = processes
            .iter()
            .position(|p| p.id == shortest.id)
            .unwrap();

        let process = processes.remove(index);
        current_time += process.burst_time;
        completed.push(process);
    }

    completed
}

fn main() {
    let processes = vec![
        Process { id: 1, burst_time: 6, arrival_time: 0 },
        Process { id: 2, burst_time: 2, arrival_time: 1 },
        Process { id: 3, burst_time: 8, arrival_time: 2 },
        Process { id: 4, burst_time: 3, arrival_time: 3 },
    ];

    let scheduled = sbtf_scheduling(processes);
    for process in scheduled {
        println!("Process {}: Burst Time = {}", process.id, process.burst_time);
    }
}