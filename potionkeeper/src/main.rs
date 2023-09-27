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
        #[structopt(short, long, default_value = "0")]
        solver_loglevel: String,

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
        Command::Recommend {
            config,
            solver_loglevel,
        } => recommend::recommend(config, solver_loglevel)?,
        Command::Debug { config } => debug::debug(config)?,
    }

    Ok(())
}
