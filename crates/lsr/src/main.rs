use std::path::Path;
use std::process::ExitCode;

fn main() -> ExitCode {
    let options = lsr::parse_options(std::env::args().skip(1));

    match lsr::list_entries(Path::new("."), options) {
        Ok(entries) => {
            for entry in &entries {
                println!("{}", lsr::format_entry(entry));
            }
            ExitCode::SUCCESS
        }
        Err(err) => {
            eprintln!("lsr: {err}");
            ExitCode::FAILURE
        }
    }
}
