use argparse::{Cli, CliError, FlagBuilder};
use rand::seq::SliceRandom;

fn main() -> Result<(), CliError> {
    let args: Vec<String> = std::env::args().collect();

    let cli = Cli {
        program_name: "ur_mom".to_string(),
        flags: vec![
            FlagBuilder::new("father".into())
                .short('f')
                .help("sustitute 'mother' with 'father'".into())
                .build()?,
            FlagBuilder::new("help".into())
                .short('h')
                .help("display the help message".into())
                .build()?,
        ],
    };

    let flagparse = cli.run(&args)?;

    // display help message if flag was passed
    if flagparse.get_flag("help") {
        println!("{}", HELP_MSG);
        return Ok(());
    }

    // first argument is number of jokes to print (if not supplied default to 1)
    let count = flagparse
        .args()
        .get(0)
        .and_then(|c| c.parse::<i32>().ok())
        .unwrap_or(1);

    for _ in 0..count {
        // choose random mom joke
        let mut mom: String = MOM.choose(&mut rand::thread_rng()).unwrap().to_string();

        // perform substitutions
        if flagparse.get_flag("father") {
            mom = mom.replace("mamma", "dadda");
            mom = mom.replace("she", "he");
        }

        println!("{}", mom);
    }

    Ok(())
}

const HELP_MSG: &'static str = r#"
ur_mom - the most advanced program about your mother

USAGE ur_mom [arguments] <number>

ARGUMENTS

  -h, --help     display help message
  -f, --father   substitute mother with father

"#;

const MOM: &'static [&'static str] =  &[
    "Yo mamma is so fat a truck hit her and she said, Hey who threw that rock?",
    "Yo mamma is so fat every time she sits down they add another country to the map",
    "Yo mamma is so fat every time she tries to hide she is hiding that object that she is trying to hide by",
    "Yo mamma is so fat if she buys a fur coat, a species will be extinct.",
    "Yo mamma is so fat people called her a wild hog",
    "Yo mamma is so fat people thought her butt hole was a black hole.",
    "Yo mamma is so fat she can be my bear",
    "Yo mamma is so fat that Weight Watchers said I give up",
    "Yo mamma is so fat that on halloween she says trick or meatloaf",
    "Yo mamma is so fat that she uses the Great Wall of China as a belt",
    "Yo mamma is so fat that the recursive function calculating her mass causes a stack overflow",
    "Yo mamma is so fat were in her right now",
    "Yo mamma is so fat when I killed her in Call of Duty I got a five kill scorestreak.",
    "Yo mamma is so fat when she tried to go to McDonalds she tripped over Wendys and landed on Burger King",
    "Yo mamma is so fat when she was walking on the street her belly button was sticking out everybody started saying, hey look a third nipple!",
    "Yo mamma is so fat you feed on her junk",
    "Yo mamma is so fat, her diet pills say M & M",
    "Yo mamma is so fat, she got arrested at the airport for ten pounds of crack",
    "Yo mamma is so fat, that when she steps on a scale it says to be continued.",
    "Yo mamma so fat I had to take a train and two buses just to get on the her good side.",
    "Yo mamma so fat I took a picture of her last Christmas and its still printing",
    "Yo mamma so fat Mount Everest tried to climb her",
    "Yo mamma so fat her beeper went off and people thought she was backing up.",
    "Yo mamma so fat her belly button got home 30 minutes before she did",
    "Yo mamma so fat her nickname is just DAMN",
    "Yo mamma so fat it aint funny",
    "Yo mamma so fat not even Dora can explore her",
    "Yo mamma so fat she died",
    "Yo mamma so fat she doesn't need the internet; she's already world wide",
    "Yo mamma so fat she don't fit in this joke",
    "Yo mamma so fat she fell in a whirlpool and got stuck",
    "Yo mamma so fat she left the house in high heels and when she came back she had on flipflops",
    "Yo mamma so fat she sat on Wal-Mart and lowered the prices",
    "Yo mamma so fat she sat on a quarter and a booger shot out of George Washington's nose.",
    "Yo mamma so fat she sat on a rainbow and popped Skittles out",
    "Yo mamma so fat she stepped on a Nintendo Gamecube and turned it into a Gameboy",
    "Yo mamma so fat she was diagnosed with flesh eating bacteria and the doctor gave her 87 years to live",
    "Yo mamma so fat she went to japan and Godzilla said DAMN and ran away.",
    "Yo mamma so fat she wore a yellow raincoat and people yelled Taxi!",
    "Yo mamma so fat she's on both sides the family",
    "Yo mamma so fat that she fell over and rocked herself to sleep trying to get up",
    "Yo mamma so fat that when she died she broke the stairway to heaven",
    "Yo mamma so fat that when she died, she ended world hunger.",
    "Yo mamma so fat that when she sits around the house, she sits around the house.",
    "Yo mamma so fat that when she walked by the TV set I missed 3 seasons of Laguna Beach",
    "Yo mamma so fat that when she was born, she gave the hospital stretch marks.",
    "Yo mamma so fat the last time she saw 90210 was on a scale",
    "Yo mamma so fat the only alphabet she knows is her KFC's",
    "Yo mamma so fat when God said let it be light he told your mamma to move",
    "Yo mamma so fat when she burped New Orlean thought Katrina came back to finish the job",
    "Yo mamma so fat when she dances at a concert the whole band skips.",
    "Yo mamma so fat when she gets cut she bleeds gravy",
    "Yo mamma so fat when she goes camping the bears hide their food",
    "Yo mamma so fat when she goes swimming the whales start singing We are Family",
    "Yo mamma so fat when she jogs she leaves pot holes",
    "Yo mamma so fat when she made a YouTube account the entire network crashed.",
    "Yo mamma so fat when she stepped on a scale Buzz Lightyear came out and said infinity and beyond!",
    "Yo mamma so fat when she tossed in her sleep she woke up in another time zone",
    "Yo mamma so fat when she went to McDonalds they offered her a group discount",
    "Yo mamma so fat when she went to her wedding the people sang the song 'here comes the bride so fat and wide'",
    "Yo mamma so fat when she went to the movies she sat by everybody",
    "Yo mamma so fat, I got lost after running around her.",
    "Yo mamma so fat, her stomach gets home 15 minutes before she does.",
    "Yo mamma so fat, that the Twinkies revolve around her.",
    "Yo mamma so fat, when she entered a fat contest, she came in first, second, and third.",
];
