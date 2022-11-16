
#[derive(Debug)]
pub enum CliError {
    NoArgument,
}

impl std::error::Error for CliError {}

impl std::fmt::Display for CliError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
	match self {
	    Self::NoArgument => write!(f, "Argument not supplied"),
	}
    }
}

pub struct Cli {
    pub program_name: String,
}

impl Cli {
    pub fn run(&self, args: &Vec<String>) -> Result<(), CliError> {
	let argument: Option<&String> = args.get(1);

	// Convert Option type to Result type
	let arg_value: Result<&String, CliError> = if let Some(arg) = argument {
	    println!("argument is {}", arg);
	    Ok(arg)
	} else {
	    println!("uh oh, we did not get an argument");
	    Err(CliError::NoArgument)
	};

	println!("arg_value is: {:?}", arg_value);

	// Either get the value or TERMINATE function
	let unwrapped_arg_value = match arg_value {
	    Ok(arg) => arg,
	    Err(e) => {
		return Err(e);
	    }
	};

	// do a bunch of other stuff
	println!("unwrapped_arg value is: {}", unwrapped_arg_value);

	Ok(())
    }
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    let my_cli = Cli {
	program_name: "my program".to_string(),
    };
    my_cli.run(&args);
}
