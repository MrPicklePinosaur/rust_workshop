pub struct Cli {
    pub program_name: String,
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    println!("arguments {:?}", args);

    let my_cli = Cli {
	program_name: "my program".to_string(),
    };
}
