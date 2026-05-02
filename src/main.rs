use larpa::Command;
use std::process::ExitCode;
use stock_trek::{error::result::StockTrekError, verification::verify::verify};

#[derive(Command)]
#[larpa(
    name = "stock-trek",
    version = "1.0",
    homepage = "https://github.com/Stock-Trek/stock-trek",
    license = "MIT",
    repository = "https://github.com/Stock-Trek/stock-trek"
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
            Err(StockTrekError::Verification(e)) => {
                e.errors.iter().for_each(|error| println!("{}", error));
                ExitCode::from(e.exit_code)
            }
            Ok(..) => {
                println!("This code is supported for use with stock-trek.com, happy signalling!");
                ExitCode::SUCCESS
            }
            Err(e) => {
                println!("Error {:?}", e);
                ExitCode::FAILURE
            }
        },
    }
}
