use clap::{Parser, Subcommand};

#[derive(Debug, Clone, Parser)]
#[clap(name = "Amethyst", version = env ! ("CARGO_PKG_VERSION"), about = env ! ("CARGO_PKG_DESCRIPTION"))]
pub struct Args {
    #[clap(subcommand)]
    pub subcommand: Option<Operation>,

    /// Sets the level of verbosity
    #[clap(long, short, parse(from_occurrences))]
    pub verbose: usize,

    /// Complete operation without prompting user
    #[clap(long = "noconfirm")]
    pub no_confirm: bool,
}

#[derive(Debug, Clone, Subcommand)]
pub enum Operation {
    /// Installs a package from either the AUR or the PacMan-defined repositories
    #[clap(name = "install", aliases = & ["ins", "in", "i", "-S"])]
    Install(InstallArgs),

    /// Removes a previously installed package
    #[clap(name = "remove", aliases = & ["rm", "r", "-R", "-Rs"])]
    Remove(RemoveArgs),

    /// Searches for the relevant packages in both the AUR and repos
    #[clap(name = "search", aliases = & ["sea", "se", "s", "-Ss"])]
    Search(SearchArgs),

    /// Queries installed packages
    #[clap(name = "query", aliases = & ["ls", "l", "-Q"])]
    Query(QueryArgs),

    /// Upgrades locally installed packages to their latest versions
    #[clap(name = "upgrade", aliases = & ["upg", "up", "u", "-Syu"])]
    Upgrade,

    /// Removes all orphaned packages
    #[clap(name = "clean", aliases = & ["cln", "cl", "-Sc"])]
    Clean,
}

impl Default for Operation {
    fn default() -> Self {
        Self::Upgrade
    }
}

#[derive(Default, Debug, Clone, Parser)]
pub struct InstallArgs {
    /// The name of the package(s) to install
    #[clap(required = true)]
    pub packages: Vec<String>,
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
    #[clap(long, short)]
    pub aur: bool,

    /// Lists repo/native packages
    #[clap(long, short)]
    pub repo: bool,
}
