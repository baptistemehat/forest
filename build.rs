use clap::CommandFactory;
use clap::ValueEnum;
use clap_complete::{generate_to, Shell};
use std::io::Error;

include!("src/cli.rs");

fn main() -> Result<(), Error> {
    let mut cmd = Cli::command();
    for &shell in Shell::value_variants() {
        let path = generate_to(shell, &mut cmd, "forest", "./completions")?;
        println!("cargo:warning=completion file is generated: {path:?}");
    }

    Ok(())
}
