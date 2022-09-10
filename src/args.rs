#![allow(clippy::module_name_repetitions)]

use crate::operations::SearchBy;
use clap::{Parser, Subcommand, ValueHint};

static VERSION: &str = concat!(env!("CARGO_PKG_VERSION"), " (Fish)");

#[derive(Debug, Clone, Parser)]
#[clap(bin_name = "ame", name = "Amethyst", version = VERSION, about = env ! ("CARGO_PKG_DESCRIPTION"), infer_subcommands = true)]
pub struct Args {
    #[clap(subcommand)]
    pub subcommand: Option<Operation>,

    /// Sets the level of verbosity
    #[clap(long, short, parse(from_occurrences), global = true)]
    pub verbose: usize,

    /// Complete operation without prompting user
    #[clap(long = "noconfirm", global = true)]
    pub no_confirm: bool,

    /// Make some commands have less output
    #[clap(long, short, global = true)]
    pub quiet: bool,

    /// Loops sudo in the background to ensure it doesn't time out during long builds
    #[clap(long = "sudoloop", global = true)]
    pub sudoloop: bool,

    /// Sets a custom AUR clone and build directory for the specified operation
    #[clap(long, short, global = true, value_hint = ValueHint::DirPath)]
    pub cachedir: Option<String>,
}

#[derive(Debug, Clone, Subcommand)]
pub enum Operation {
    /// Installs or searches for a package in either the AUR or the Pacman-defined repositories
    #[clap(bin_name = "ame", name = "sync", aliases = & [ "-S" ], visible_aliases = & ["install", "i"], short_flag = 'S')]
    Install(InstallArgs),

    /// Removes a previously installed package
    #[clap(bin_name = "ame", name = "remove", visible_aliases = & ["rm"], short_flag = 'R')]
    Remove(RemoveArgs),

    /// Searches for packages matching a provided pattern in the AUR/repos [aliases: -Ss]
    #[clap(bin_name = "ame", name = "search")]
    Search(InstallArgs),

    /// Queries installed packages
    #[clap(bin_name = "ame", name = "query", short_flag = 'Q')]
    Query(QueryArgs),

    /// Upgrades locally installed packages to their latest versions (Default)
    #[clap(bin_name = "ame", name = "upgrade", visible_aliases = & ["-Syu"])]
    Upgrade(UpgradeArgs),

    /// Generates shell completions for supported shells (bash, fish, elvish, pwsh)
    #[clap(bin_name = "ame", name = "gencomp", short_flag = 'G')]
    GenComp(GenCompArgs),

    /// Removes all orphaned packages
    #[clap(bin_name = "ame", name = "clean", short_flag = 'C')]
    Clean,

    /// Runs pacdiff
    #[clap(bin_name = "ame", name = "diff", short_flag = 'd')]
    Diff,
}

impl Default for Operation {
    fn default() -> Self {
        Self::Upgrade(UpgradeArgs::default())
    }
}

#[derive(Default, Debug, Clone, Parser)]
pub struct InstallArgs {
    /// The name of the package(s) to install or search for
    #[clap(required = true)]
    pub packages: Vec<String>,

    /// Operate only on AUR packages
    #[clap(long, short)]
    pub aur: bool,

    /// Operate only on repo packages
    #[clap(long, short)]
    pub repo: bool,

    /// Search packages for a given pattern instead of installing
    #[clap(hidden = true, short = 's')]
    pub search: bool,

    /// Searches by a specific field
    #[clap(long, short)]
    pub by: Option<SearchBy>,
}

#[derive(Default, Debug, Clone, Parser)]
pub struct RemoveArgs {
    /// The name of the package(s) to remove
    #[clap(required = true)]
    pub packages: Vec<String>,
}

#[derive(Default, Debug, Clone, Parser)]
pub struct QueryArgs {
    /// Lists AUR/foreign packages [-Qa, -Qm]
    #[clap(long, short)]
    pub aur: bool,

    /// Lists repo/native packages [-Qr, -Qn]
    #[clap(long, short)]
    pub repo: bool,

    /// Get information about a specific package
    #[clap(long, short)]
    pub info: Option<String>,
}

#[derive(Default, Debug, Clone, Parser)]
pub struct UpgradeArgs {
    /// Upgrades only repo/native packages
    #[clap(long, short)]
    pub repo: bool,

    /// Upgrades only from the AUR
    #[clap(long, short)]
    pub aur: bool,
}

#[derive(Default, Debug, Clone, Parser)]
pub struct GenCompArgs {
    /// The shell to generate completions for (bash, fish, elvish, pwsh, fig)
    #[clap(required = true)]
    pub shell: String,
}
