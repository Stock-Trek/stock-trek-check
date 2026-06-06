use larpa::Command;
use std::process::ExitCode;
use stock_trek_check::verify::verify;

#[derive(Command)]
#[larpa(
    name = "stock-trek-check",
    version = "0.1.1",
    homepage = "https://github.com/Stock-Trek/stock-trek-check",
    license = "MIT",
    repository = "https://github.com/Stock-Trek/stock-trek-check"
)]
struct Cli {
    #[larpa(subcommand)]
    command: Commands,
}

#[derive(Command)]
enum Commands {
    Verify {
        #[larpa(name = ["-f", "--file"])]
        file: String,
    },
}

fn main() -> ExitCode {
    let cli = Cli::from_args();
    match cli.command {
        Commands::Verify { file } => match verify(file) {
            Err(e) => {
                e.errors.iter().for_each(|error| println!("{}", error));
                ExitCode::from(e.exit_code)
            }
            Ok(..) => {
                println!("This code is supported for use with stock-trek.com, happy signalling!");
                ExitCode::SUCCESS
            }
        },
    }
}
