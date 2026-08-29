use std::path::Path;
use std::process::ExitCode;

fn main() -> ExitCode {
    let options = lsr::parse_options(std::env::args().skip(1));

    match lsr::list_names(Path::new("."), options) {
        Ok(names) => {
            for name in names {
                println!("{name}");
            }
            ExitCode::SUCCESS
        }
        Err(err) => {
            eprintln!("lsr: {err}");
            ExitCode::FAILURE
        }
    }
}
