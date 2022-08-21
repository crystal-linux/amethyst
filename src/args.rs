#![allow(clippy::module_name_repetitions)]

use clap::{Parser, Subcommand};

#[derive(Debug, Clone, Parser)]
#[clap(name = "Amethyst", version = env ! ("CARGO_PKG_VERSION"), about = env ! ("CARGO_PKG_DESCRIPTION"), infer_subcommands = true)]
pub struct Args {
    #[clap(subcommand)]
    pub subcommand: Option<Operation>,

    /// Sets the level of verbosity
    #[clap(long, short, parse(from_occurrences), global = true)]
    pub verbose: usize,

    /// Complete operation without prompting user
    #[clap(long = "noconfirm", global = true)]
    pub no_confirm: bool,

    /// Loops sudo in the background to ensure it doesn't time out during long builds
    #[clap(long = "sudoloop", global = true)]
    pub sudoloop: bool,

    /// Sets a custom AUR clone and build directory for the specified operation
    #[clap(long, short, global = true)]
    pub cachedir: Option<String>,
}

#[derive(Debug, Clone, Subcommand)]
pub enum Operation {
    /// Installs a package from either the AUR or the Pacman-defined repositories
    #[clap(name = "install", visible_aliases = & ["-S"])]
    Install(InstallArgs),

    /// Removes a previously installed package
    #[clap(name = "remove", visible_aliases = & ["rm", "-Rs"])]
    Remove(RemoveArgs),

    /// Searches for the relevant packages in both the AUR and repos
    #[clap(name = "search", visible_aliases = & ["-Ss"])]
    Search(SearchArgs),

    /// Queries installed packages
    #[clap(name = "query", visible_aliases = & ["-Q"])]
    Query(QueryArgs),

    /// Gets info about a package
    #[clap(name = "info", visible_aliases = & ["-Qi"])]
    Info(InfoArgs),

    /// Upgrades locally installed packages to their latest versions
    #[clap(name = "upgrade", visible_aliases = & ["-Syu"])]
    Upgrade(UpgradeArgs),

    /// Removes all orphaned packages
    #[clap(name = "clean", visible_aliases = & ["-Sc"])]
    Clean,

    /// Runs pacdiff
    #[clap(name = "diff", visible_aliases = & ["-d"])]
    Diff,
}

impl Default for Operation {
    fn default() -> Self {
        Self::Upgrade(UpgradeArgs::default())
    }
}

#[derive(Default, Debug, Clone, Parser)]
pub struct InstallArgs {
    /// The name of the package(s) to install
    #[clap(required = true)]
    pub packages: Vec<String>,

    /// Installs only from the AUR
    #[clap(long, short)]
    pub aur: bool,

    /// Install the packages from the pacman-defined repositories
    #[clap(long, short)]
    pub repo: bool,
}

#[derive(Default, Debug, Clone, Parser)]
pub struct RemoveArgs {
    /// The name of the package(s) to remove
    #[clap(required = true)]
    pub packages: Vec<String>,
}

#[derive(Default, Debug, Clone, Parser)]
pub struct SearchArgs {
    /// Searches for the relevant packages in both the AUR and repos
    #[clap(long, short)]
    pub aur: bool,

    /// Searches only local repos for the package
    #[clap(long, short)]
    pub repo: bool,

    /// The string the package must match in the search
    #[clap(required = true)]
    pub search: Vec<String>,
}

#[derive(Default, Debug, Clone, Parser)]
pub struct QueryArgs {
    /// Lists AUR/foreign packages
    #[clap(long, short, from_global)]
    pub aur: bool,

    /// Lists repo/native packages
    #[clap(long, short, from_global)]
    pub repo: bool,
}

#[derive(Default, Debug, Clone, Parser)]
pub struct InfoArgs {
    /// The name of the package(s) to get info on
    #[clap(required = true)]
    pub package: String,
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
