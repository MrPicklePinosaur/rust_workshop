pub struct Cli {
    pub program_name: String,
}

impl Cli {
    pub fn run(&self, args: &Vec<String>) {
	println!("[{}] {:?}", self.program_name, args);
    }
}

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    let my_cli = Cli {
	program_name: "my program".to_string(),
    };
    my_cli.run(&args);
}
