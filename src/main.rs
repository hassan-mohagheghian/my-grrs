use anyhow::{self, Context};
use clap::Parser;
use my_grrs::find_matches;

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();
    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file {:?}", &args.path))?;
    find_matches(&content, &args.pattern, &mut std::io::stdout())?;
    Ok(())
}
