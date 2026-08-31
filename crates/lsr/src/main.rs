use lsr::args;
use lsr::error;
use lsr::format;
use std::path::Path;
use std::process::ExitCode;

fn run() -> Result<(), error::Error> {
    let options = args::parse_options(std::env::args().skip(1));
    let entries = lsr::list_entries(Path::new("."), options)?;

    for entry in &entries {
        println!("{}", format::format_entry(entry)?);
    }

    Ok(())
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("lsr: {err}");
            ExitCode::FAILURE
        }
    }
}
