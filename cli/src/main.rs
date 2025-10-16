// https://stackoverflow.com/a/61417700
#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(missing_docs)]

/*!
An unfussy web server designed for straightforward authoring and scalable performance.

For more information and usage examples see the
[home page](https://github.com/tliron/credence).
*/

mod commands;
mod errors;
mod run;

use run::*;

use {mimalloc::*, std::process::*};

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

/// Main.
pub fn main() -> ExitCode {
    kutil::cli::run::run(run)
}
