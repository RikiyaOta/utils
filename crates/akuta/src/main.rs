use akuta::args::{self, Command};
use akuta::config::{self, Config, Env};
use akuta::error::Error;
use akuta::trash::Trash;
use std::io;
use std::process::ExitCode;
use std::time::SystemTime;

/// Runs the command and returns the exit code to finish with.
///
/// Errors that stop the whole run (a bad command line, an unreadable config
/// file, …) come back as `Err`. Errors on individual paths while trashing do
/// not: like `rm`, `akuta` reports each one, carries on with the remaining
/// paths, and exits with 1 at the end.
fn run() -> Result<ExitCode, Error> {
    let command = args::parse_args(std::env::args().skip(1))?;

    let env = Env::from_process();
    let config = match config::config_file_path(&env) {
        Some(path) => config::load_config(&path)?,
        None => Config::default(),
    };
    let trash = Trash::new(config::resolve_trash_dir(&config, &env)?);

    match command {
        Command::Put(paths) => {
            trash.ensure_dirs()?;
            let mut exit_code = ExitCode::SUCCESS;
            for path in &paths {
                let result =
                    akuta::trash_path(path, &trash, env.home.as_deref(), SystemTime::now());
                if let Err(err) = result {
                    eprintln!("akuta: cannot remove '{}': {err}", path.display());
                    exit_code = ExitCode::FAILURE;
                }
            }
            Ok(exit_code)
        }
        Command::List => {
            for entry in trash.list()? {
                println!("{}", akuta::format_entry(&entry));
            }
            Ok(ExitCode::SUCCESS)
        }
        Command::Restore(name) => {
            let restored = trash.restore(&name, &mut io::stdin().lock(), &mut io::stdout())?;
            println!("restored '{}'", restored.display());
            Ok(ExitCode::SUCCESS)
        }
    }
}

fn main() -> ExitCode {
    match run() {
        Ok(exit_code) => exit_code,
        Err(err) => {
            eprintln!("akuta: {err}");
            ExitCode::FAILURE
        }
    }
}
