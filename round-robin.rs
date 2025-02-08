use std::collections::VecDeque;

struct Process {
    id: usize,
    burst_time: u32,
    remaining_time: u32,
}

struct RoundRobin {
    queue: VecDeque<Process>,
    time_quantum: u32,
    current_time: u32,
}

impl RoundRobin {
    fn new(time_quantum: u32) -> Self {
        RoundRobin {
            queue: VecDeque::new(),
            time_quantum,
            current_time: 0,
        }
    }

    fn add_process(&mut self, id: usize, burst_time: u32) {
        self.queue.push_back(Process {
            id,
            burst_time,
            remaining_time: burst_time,
        });
    }

    fn execute(&mut self) -> Vec<(usize, u32)> {
        let mut completion_times = Vec::new();

        while !self.queue.is_empty() {
            if let Some(mut process) = self.queue.pop_front() {
                let execution_time = std::cmp::min(self.time_quantum, process.remaining_time);
                self.current_time += execution_time;
                process.remaining_time -= execution_time;

                if process.remaining_time > 0 {
                    self.queue.push_back(process);
                } else {
                    completion_times.push((process.id, self.current_time));
                }
            }
        }
        completion_times
    }
}