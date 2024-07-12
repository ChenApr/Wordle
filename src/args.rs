use clap::{builder::Str, Parser};

#[derive(Parser, Debug, Default, Clone)]
#[command(name = "Wordle", version = "4.5.8", about = "Setting the parameters of the game", long_about = None)]
#[clap(author = "April")]
pub struct Args {

    ///Specify the answer
    #[arg(short, long)]
    pub word: Option<String>,
    
    ///Random word mode
    #[arg(short, long, conflicts_with = "word")]
    pub random: bool,
    
    ///Difficult modd
    #[arg(short = 'D', long)]
    pub difficult: bool,
    
    ///Print review after each game
    #[arg(short = 't', long)]
    pub stats: bool,

    ///Specify the day
    #[arg(short, long, conflicts_with = "word")]
    pub day: Option<u64>,

    ///Specify the seed
    #[arg(short, long, conflicts_with = "word")]
    pub seed: Option<u64>,

    ///Specify the final-set file path
    #[arg(short = 'f', long = "final-set")]
    pub final_set: Option<String>,

    ///Specify the acceptable file path
    #[arg(short = 'a', long = "acceptable-set")]
    pub acceptable_set: Option<String>,

    ///Read and save this game to specified file path in json
    #[arg(short = 'S', long = "state")]
    pub state: Option<String>,

    ///Specify a config file for this game
    #[arg(short, long)]
    pub config: Option<String>
}
