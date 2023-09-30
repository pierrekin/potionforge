use structopt::StructOpt;

mod debug;
mod printer;
mod recommend;

#[derive(StructOpt)]
struct Opt {
    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(StructOpt)]
enum Command {
    Recommend {
        #[structopt(short, long, default_value = "recommend.yml")]
        config: String,
    },
    InitRecommend {
        #[structopt(short, long, default_value = "recommend.yml")]
        config: String,
    },
    Debug {
        #[structopt(short, long, default_value = "debug.yml")]
        config: String,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Opt::from_args();

    match opt.cmd {
        Command::InitRecommend { config } => recommend::init_recommend(config)?,
        Command::Recommend { config } => recommend::recommend(config)?,
        Command::Debug { config } => debug::debug(config)?,
    }

    Ok(())
}
