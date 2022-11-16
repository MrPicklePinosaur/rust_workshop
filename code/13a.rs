#[derive(Debug)]
pub enum CliError {
    NoProgramName,
    NoArgument,
}

impl std::error::Error for CliError {}

impl std::fmt::Display for CliError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::NoProgramName => write!(f, "Program name not supplied"),
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

#[derive(Default, Debug)]
pub struct FlagParse {
    flags: Vec<Flag>,
    args: Vec<String>,
}

impl Cli {
    pub fn run(&self, args: &Vec<String>) -> Result<FlagParse, CliError> {
        let mut flagparse = FlagParse::default();

        let mut arg_it = args.iter();

        // skip first item
        arg_it.next().ok_or(CliError::NoProgramName)?;

        for arg in arg_it {
            println!("argument {}", arg);
        }

        Ok(flagparse)
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let cli = Cli {
        program_name: "my cli".to_string(),
        flags: vec![
            FlagBuilder::new("verbose".into())
                .short('v')
                .help("enable verbose logging mode".into())
                .required()
                .build()
		.unwrap(),
            FlagBuilder::new("help".into()).short('h').build().unwrap(),
        ],
    };

    let flagparse = cli.run(&args).unwrap();
    println!("{:?}", flagparse);
}
