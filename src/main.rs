mod cli;
mod task;

use anyhow::anyhow;
use cli::{Action::*, CommandLineArgs};
use std::path::PathBuf;
use structopt::StructOpt;
use task::Task;

fn main() -> anyhow::Result<()> {
    // Get tha command-line arguments.
    let CommandLineArgs {
        action,
        journal_file,
    } = CommandLineArgs::from_args();

    // Unpack the journal file.
    let journal_file = journal_file
        .or_else(find_default_journal_file)
        .ok_or(anyhow!("Fail to fild journal file"))?;

    // Perform the action.
    match action {
        Add { text } => task::add_task(journal_file, Task::new(text)),
        List => task::list_tasks(journal_file),
        Done { position } => task::complete_task(journal_file, position),
    }?;
    Ok(())
}

fn find_default_journal_file() -> Option<PathBuf> {
    home::home_dir().map(|mut path| {
        path.push(".rusty-journal.json");
        path
    })
}
