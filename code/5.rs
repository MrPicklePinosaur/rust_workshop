pub struct Cli {
    pub program_name: String,
}

impl Cli {
    pub fn run(&self, args: &Vec<String>) {
	let argument: Option<&String> = args.get(1);

	if let Some(arg) = argument {
	    println!("argument is {}", arg);
	} else {
	    println!("uh oh, we did not get an argument");
	}

	// ====== alternatively ======
	match argument {
	    Some(arg) => {
		println!("argument is {}", arg);
	    },
	    None => {
		println!("uh oh, we did not get an argument"); 
	    }
	}
    }
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    let my_cli = Cli {
	program_name: "my program".to_string(),
    };
    my_cli.run(&args);
}
