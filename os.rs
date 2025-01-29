#[derive(Debug)]
enum SystemCall {
    Read(usize, *mut u8, usize),  
    Write(usize, *const u8, usize), 
    Exit(i32),  
}

struct Process {
    pid: u32,
    state: ProcessState,
}

#[derive(Debug)]
enum ProcessState {
    Running,
    Ready,
    Waiting,
    Terminated,
}

struct SystemCallHandler {
    current_process: Option<Process>,
}

impl SystemCallHandler {
    fn new() -> Self {
        SystemCallHandler {
            current_process: None,
        }
    }

    fn handle_syscall(&mut self, syscall: SystemCall) -> Result<i32, &'static str> {
        match syscall {
            SystemCall::Read(fd, buf, count) => {
                println!("Reading {} bytes from fd {}", count, fd);
                Ok(count as i32)
            }
            SystemCall::Write(fd, buf, count) => {
                println!("Writing {} bytes to fd {}", count, fd);
                Ok(count as i32)
            }
            SystemCall::Exit(code) => {
                if let Some(proc) = &mut self.current_process {
                    proc.state = ProcessState::Terminated;
                    println!("Process {} terminated with code {}", proc.pid, code);
                }
                Ok(code)
            }
        }
    }
}

fn main() {
    let mut handler = SystemCallHandler::new();
    
    handler.current_process = Some(Process {
        pid: 1,
        state: ProcessState::Running,
    });

    let buffer: *mut u8 = std::ptr::null_mut();
    
    let calls = vec![
        SystemCall::Read(0, buffer, 128),
        SystemCall::Write(1, buffer, 64),
        SystemCall::Exit(0),
    ];

    for call in calls {
        println!("Executing syscall: {:?}", call);
        match handler.handle_syscall(call) {
            Ok(result) => println!("System call completed with result: {}", result),
            Err(e) => println!("System call failed: {}", e),
        }
    }
}