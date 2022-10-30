//! Simple argument parsing library created for CSC rust workshop

#[derive(Debug)]
pub enum CliError {
    NoProgramName,
    InvalidFlag(String),
}

impl std::error::Error for CliError {}

impl std::fmt::Display for CliError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
	match self {
	    Self::NoProgramName => write!(f, "Program name not supplied"),
	    Self::InvalidFlag(v) => write!(f, "Invalid flag value: {}", v)
	}
    }
}

/// Main driver for cli parsing
pub struct Cli {
    /// Name of the command line application. This will be used in help messages
    pub program_name: String,
    /// List of flags to parse
    pub flags: Vec<Flag>,
}

/// Description for parseable flag
#[derive(Clone)]
pub struct Flag {
    /// Help message output for this flag
    pub help: String,
    /// Is the flag is required to be passed
    pub required: bool,
    /// Mandatory long flag identifier. For example `--verbose`
    pub long: String,
    /// Optional short flag identifier. For example `-v`
    pub short: Option<char>,
}

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
    pub fn help(&mut self, help: String) -> &mut Self {
	self.help = Some(help);
	self
    }
    pub fn required(&mut self) -> &mut Self {
	self.required = Some(true);
	self
    }
    pub fn short(&mut self, short: char) -> &mut Self {
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


#[derive(Default)]
pub struct FlagParse {
    flags: Vec<Flag>,
    args: Vec<String>,
}

impl Cli {
    pub fn run(&self, args: &Vec<String>) -> Result<(), CliError> {
	let mut flagparse = FlagParse::default();

	let mut arg_it = args.iter();
	arg_it.next().ok_or(CliError::NoProgramName)?;

	for arg in arg_it {

	    // Decide if arg is a long flag, short flag, or a position argument
	    let flag: Option<&Flag> = if arg.starts_with("--") {
		self.flags
		    .iter()
		    .find(|f| f.long == arg[2..].to_string())
	    } else if arg.starts_with("-") {
		self.flags
		    .iter()
		    .find(|f| f.short == arg.chars().nth(1))
	    } else {
		flagparse.args.push(arg.to_owned());
		continue;
	    };

	    let flag: &Flag = if let Some(_flag) = flag {
		_flag
	    } else {
		return Err(CliError::InvalidFlag(arg.to_owned()));
	    };

	    flagparse.flags.push(flag.clone());
	}

	Ok(())
    }
}
