use std::env;
use std::fs;
use std::io;
use std::process::ExitCode;

fn main() -> ExitCode {
    for path in env::args().skip(1) {
        if let Err(err) = fs::remove_dir_all(&path)
            && err.kind() != io::ErrorKind::NotFound
        {
            eprintln!("failed to delete '{path}': {err}");
            return ExitCode::from(1);
        }
    }
    ExitCode::SUCCESS
}
