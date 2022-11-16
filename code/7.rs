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

fn main() {
    let my_error = CliError::NoArgument;
    println!("This is the debug formatter: {:?}", my_error);
    println!("This is the display formatter: {}", my_error);
}
