pub struct Cli {
    pub program_name: String,
}

enum MyResult<T, E> {
    Ok(T),
    Err(E),
}

impl Cli {
    pub fn run(&self, args: &Vec<String>) -> Result<(), Box<dyn std::error::Error>> {
        let argument: Option<&String> = args.get(1);

        let arg_value = if let Some(arg) = argument {
            println!("argument is {}", arg);
	    arg
        } else {
            println!("uh oh, we did not get an argument");
	    // What should we put in the ... ?
	    // return Err(...)
	    "error"
        };

	// do something with arg_value
	println!("the value of arg_value is {}", arg_value);

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
