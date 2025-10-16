use super::commands::*;

use {clap::*, kutil::cli::log::*, problemo::*, tokio::runtime::*};

/// Run.
pub fn run() -> Result<(), Problem> {
    let cli = Root::parse();

    if cli.journald {
        initialize_tracing_journald(cli.verbose + 2)?;
    } else if !cli.quiet {
        initialize_tracing(cli.verbose + 2, cli.log_path.as_ref())?;
    }

    match &cli.subcommand {
        None => {
            let tokio = Builder::new_multi_thread().enable_all().build()?;
            tokio.block_on(cli.start())?;
        }

        Some(SubCommand::Version(version)) => version.run::<Root>(),
        Some(SubCommand::Completion(completion)) => completion.run::<Root>(),
        Some(SubCommand::Manual(manual)) => manual.run::<Root>()?,
    }

    Ok(())
}
