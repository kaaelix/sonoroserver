use std::env;
use std::process::{Child, Command, ExitCode};
use std::thread;
use std::time::Duration;

fn spawn_child(path: &str) -> Result<Child, std::io::Error> {
    Command::new(path).spawn()
}

fn kill_all(children: &mut [Child]) {
    for child in children {
        let _ = child.kill();
        let _ = child.wait();
    }
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() {
        eprintln!("Usage: serve-all <executable>...");
        return ExitCode::from(1);
    }

    let mut children = Vec::with_capacity(args.len());
    for path in &args {
        match spawn_child(path) {
            Ok(child) => children.push(child),
            Err(err) => {
                eprintln!("failed to spawn '{path}': {err}");
                kill_all(&mut children);
                return ExitCode::from(1);
            }
        }
    }

    loop {
        let mut any_running = false;
        for (idx, child) in children.iter_mut().enumerate() {
            match child.try_wait() {
                Ok(Some(status)) => {
                    if !status.success() {
                        eprintln!("child '{}' exited with {status}, terminating rest", args[idx]);
                        kill_all(&mut children);
                        return ExitCode::from(status.code().unwrap_or(1) as u8);
                    }
                }
                Ok(None) => {
                    any_running = true;
                }
                Err(err) => {
                    eprintln!("wait error for '{}': {err}", args[idx]);
                    kill_all(&mut children);
                    return ExitCode::from(1);
                }
            }
        }
        if !any_running {
            return ExitCode::SUCCESS;
        }
        thread::sleep(Duration::from_millis(100));
    }
}
