#![allow(clippy::module_name_repetitions)]

use crate::operations::SearchBy;
use clap::{Parser, Subcommand, ValueHint};

#[derive(Debug, Clone, Parser)]
#[clap(bin_name = "ame", name = "Amethyst", version = env ! ("CARGO_PKG_VERSION"), about = env ! ("CARGO_PKG_DESCRIPTION"), infer_subcommands = true, allow_external_subcommands = true, allow_hyphen_values = true)]
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
    #[clap(long, short, global = true, value_hint = ValueHint::DirPath)]
    pub cachedir: Option<String>,
}

#[derive(Debug, Clone, Subcommand)]
pub enum Operation {
    /// Installs a package from either the AUR or the Pacman-defined repositories
    #[clap(bin_name = "ame", name = "install", visible_aliases = & ["-S"], aliases = & ["-Sa", "-Sr"])]
    Install(InstallArgs),

    /// Removes a previously installed package
    #[clap(bin_name = "ame", name = "remove", visible_aliases = & ["rm", "-Rs"])]
    Remove(RemoveArgs),

    /// Searches for packages matching a regex-supported pattern in the AUR and/or the repos
    #[clap(bin_name = "ame", name = "search", visible_aliases = & ["-Ss"], aliases = & ["-Ssa", "-Ssr"])]
    Search(SearchArgs),

    /// Queries installed packages
    #[clap(bin_name = "ame", name = "query", visible_aliases = & ["-Q"], aliases = & ["-Qa", "-Qr", "-Qm", "-Qn"])]
    Query(QueryArgs),

    /// Gets info about a package
    #[clap(bin_name = "ame", name = "info", visible_aliases = & ["-Qi"])]
    Info(InfoArgs),

    /// Upgrades locally installed packages to their latest versions (Default)
    #[clap(bin_name = "ame", name = "upgrade", visible_aliases = & ["-Syu"])]
    Upgrade(UpgradeArgs),

    /// Generates shell completions for supported shells (bash, fish, elvish, pwsh)
    #[clap(bin_name = "ame", name = "gencomp", visible_aliases = & ["-g"])]
    GenComp(GenCompArgs),

    /// Removes all orphaned packages
    #[clap(bin_name = "ame", name = "clean", visible_aliases = & ["-Sc"])]
    Clean,

    /// Runs pacdiff
    #[clap(bin_name = "ame", name = "diff", visible_aliases = & ["-d"])]
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

    /// Installs only from the AUR [-Sa]
    #[clap(long, short)]
    pub aur: bool,

    /// Install the packages only from the pacman-defined repositories [-Sr]
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
    /// Searches for the relevant packages in both the AUR and repos [-Ssa]
    #[clap(long, short)]
    pub aur: bool,

    /// Searches only pacman repos for the package [-Ssr]
    #[clap(long, short)]
    pub repo: bool,

    /// The string the package must match in the search
    #[clap(required = true)]
    pub search: String,

    /// Searches by a specific field
    #[clap(long, short)]
    pub by: Option<SearchBy>,
}

#[derive(Default, Debug, Clone, Parser)]
pub struct QueryArgs {
    /// Lists AUR/foreign packages [-Qa, -Qm]
    #[clap(long, short)]
    pub aur: bool,

    /// Lists repo/native packages [-Qr, -Qn]
    #[clap(long, short)]
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

#[derive(Default, Debug, Clone, Parser)]
pub struct GenCompArgs {
    /// The shell to generate completions for (bash, fish, elvish, pwsh)
    #[clap(required = true)]
    pub shell: String,
}
