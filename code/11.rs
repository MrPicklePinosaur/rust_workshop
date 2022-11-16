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
    pub flags: Vec<Flag>,
}

#[derive(Debug)]
pub struct Flag {
    pub help: String,
    pub required: bool,
    pub long: String,
    pub short: Option<char>,
}

impl Flag {}

#[derive(Debug)]
pub struct FlagBuilder {
    pub help: Option<String>,
    pub required: Option<bool>,
    pub long: String,
    pub short: Option<char>,
}

impl FlagBuilder {
    pub fn new(long: String) -> Self {
	FlagBuilder {
	    help: None,
	    required: None,
	    long,
	    short: None,
	}
    }
    pub fn help(mut self, help: String) -> Self {
	self.help = Some(help);
	self
    }
    pub fn required(mut self) -> Self {
	self.required = Some(true);
	self
    }
    pub fn short(mut self, short: char) -> Self {
	self.short = Some(short);
	self
    }
    pub fn build(self) -> Result<Flag, CliError> {
	Ok(Flag {
	    help: self.help.unwrap_or(String::new()),
	    required: self.required.unwrap_or(false),
	    long: self.long,
	    short: self.short,
	})
    }
}

fn main() {
    let flag_1 = FlagBuilder::new("verbose".into())
	.short('v')
	.help("enable verbose logging mode".into())
	.required();

    println!("flag_1: {:?}", flag_1);
 
    let flag_1_built = flag_1.build();

    println!("flag_1_built: {:?}", flag_1_built);

    let flag_2_built = FlagBuilder::new("help".into())
	.short('h')
	.build();

    println!("flag_2_built: {:?}", flag_2_built);
}
