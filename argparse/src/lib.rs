#[derive(Debug)]
pub enum CliError {
    NoProgramName,
    NoArgument,
    InvalidFlag(String),
}

impl std::error::Error for CliError {}

impl std::fmt::Display for CliError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::NoProgramName => write!(f, "Program name not supplied"),
            Self::NoArgument => write!(f, "Argument not supplied"),
            Self::InvalidFlag(v) => write!(f, "Invalid flag: {}", v),
        }
    }
}

pub struct Cli {
    pub program_name: String,
    pub flags: Vec<Flag>,
}

#[derive(Debug, Clone)]
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

#[derive(Default, Debug)]
pub struct FlagParse {
    flags: Vec<Flag>,
    args: Vec<String>,
}

impl FlagParse {

    pub fn get_flag(&self, long: &str) -> bool {
	self.flags.iter().find(|f| f.long.eq(long)).is_some()
    }

    pub fn args(&self) -> &Vec<String> {
	&self.args
    }

}

impl Cli {
    pub fn run(&self, args: &Vec<String>) -> Result<FlagParse, CliError> {
        let mut flagparse = FlagParse::default();

        let mut arg_it = args.iter();

        // skip first item
        arg_it.next().ok_or(CliError::NoProgramName)?;

        for arg in arg_it {
            // Decide if arg is a long flag, short flag, or a position argument
            let flag: Option<&Flag> = if arg.starts_with("--") {
                self.flags.iter().find(|f| f.long == arg[2..].to_string())
            } else if arg.starts_with("-") {
                self.flags.iter().find(|f| f.short == arg.chars().nth(1))
            } else {
                flagparse.args.push(arg.to_owned());
                continue;
            };

	    let flag: &Flag = flag.ok_or(CliError::InvalidFlag(arg.to_owned()))?;

            flagparse.flags.push(flag.clone());
        }

        Ok(flagparse)
    }
}
