use std::path::Path;
use std::process::ExitCode;
use teru::args;
use teru::error;
use teru::format;

fn run() -> Result<(), error::Error> {
    let options = args::parse_options(std::env::args().skip(1));
    let entries = teru::list_entries(Path::new("."), options)?;

    for entry in &entries {
        println!("{}", format::format_entry(entry)?);
    }

    Ok(())
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("teru: {err}");
            ExitCode::FAILURE
        }
    }
}
