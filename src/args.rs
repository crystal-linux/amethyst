use clap::{Parser, Subcommand};

#[derive(Debug, Clone, Parser)]
#[clap(name = "Amethyst", version = env ! ("CARGO_PKG_VERSION"), about = env ! ("CARGO_PKG_DESCRIPTION"))]
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
}

#[derive(Debug, Clone, Subcommand)]
pub enum Operation {
    /// Installs a package from either the AUR or the PacMan-defined repositories
    #[clap(name = "install", aliases = & ["ins", "in", "i", "-S"])]
    Install(InstallArgs),

    /// Removes a previously installed package
    #[clap(name = "remove", aliases = & ["rm", "rem", "r", "-R", "-Rs"])]
    Remove(RemoveArgs),

    /// Searches for the relevant packages in both the AUR and repos
    #[clap(name = "search", aliases = & ["sea", "sear", "se", "s", "-Ss"])]
    Search(SearchArgs),

    /// Queries installed packages
    #[clap(name = "query", aliases = & ["q", "qu", "l", "ls", "-Q"])]
    Query(QueryArgs),

    /// Upgrades locally installed packages to their latest versions
    #[clap(name = "upgrade", aliases = & ["upg", "up", "u", "-Syu"])]
    Upgrade(UpgradeArgs),

    /// Removes all orphaned packages
    #[clap(name = "clean", aliases = & ["cln", "cl", "-Sc"])]
    Clean,

    /// Runs pacdiff
    #[clap(name = "diff", aliases = & ["dif", "di", "-d"])]
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
    pub aur: bool,

    /// Install the packages from the pacman-defined repositories
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
pub struct UpgradeArgs {
    /// Upgrades only repo/native packages
    #[clap(long, short)]
    pub repo: bool,

    /// Upgrades only from the AUR
    #[clap(long, short)]
    pub aur: bool,
}
