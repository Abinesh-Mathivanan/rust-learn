use std::process::{Command, Child, Stdio};
use std::collections::HashMap;
use std::io::Error;

struct ProcessManager {
    running_processes: HashMap<u32, Child>,
}

impl ProcessManager {
    fn new() -> Self {
        ProcessManager {
            running_processes: HashMap::new(),
        }
    }

    fn spawn_process(&mut self, program: &str, args: &[&str]) -> Result<u32, Error> {
        let child = Command::new(program)
            .args(args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        let pid = child.id();
        self.running_processes.insert(pid, child);
        Ok(pid)
    }

    fn kill_process(&mut self, pid: u32) -> Result<(), Error> {
        if let Some(mut child) = self.running_processes.remove(&pid) {
            child.kill()?;
            child.wait()?;
            Ok(())
        } else {
            Ok(())
        }
    }

    fn list_processes(&self) -> Vec<u32> {
        self.running_processes.keys().cloned().collect()
    }

    fn cleanup(&mut self) {
        let pids: Vec<u32> = self.running_processes.keys().cloned().collect();
        for pid in pids {
            let _ = self.kill_process(pid);
        }
    }
}

fn main() {
    let mut pm = ProcessManager::new();

    match pm.spawn_process("sleep", &["10"]) {
        Ok(pid) => println!("Started process with PID: {}", pid),
        Err(e) => eprintln!("Failed to start process: {}", e),
    }

    println!("Running processes: {:?}", pm.list_processes());
    pm.cleanup();
}