use argparse::{Cli, CliError, Flag, FlagBuilder, FlagParse};

fn main() -> Result<(), CliError> {
    let args: Vec<String> = std::env::args().collect();

    let cli = Cli {
        program_name: "my cli".to_string(),
        flags: vec![
            FlagBuilder::new("verbose".into())
                .short('v')
                .help("enable verbose logging mode".into())
                .build()?,
            FlagBuilder::new("help".into()).short('h').help("display the help message".into()).build()?,
        ],
    };

    let flagparse = cli.run(&args).unwrap();

    if flagparse.get_flag("help") {
        println!("this is the help message");
        return Ok(());
    }

    Ok(())
}
