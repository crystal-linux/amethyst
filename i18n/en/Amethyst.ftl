# used across files
exiting = Exiting
packages = { $pkgNum ->
    [one] package
    *[other] packages
}
missing-deps = Missing dependencies
continue = Continue?

# main
run-as-root = Running Amethyst as root is disallowed as it can lead to system breakage. Instead, Amethyst will prompt you when it needs superuser permissions.
following-packages = The following packages were found in the AUR:
aur-warning = 
    The AUR is a source of user-submitted packages/scripts and isn't always safe to use.
    Please make sure to review the PKGBUILD of anything you download from the AUR before installing it, as some PKGBUILDs may potentially be malicious. 
    This warning can be toggled in the configuration file.
are-you-sure = Are you sure that you want to continue?
uninstalling-packages = Uninstalling packages: {$packages}
system-upgrade = Performing system upgrade
removing-orphans = Removing orphaned packages
couldnt-find-packages = Couldn't find packages: {$packages} in repos or the AUR
searching-repos = Searching repos for {$query}
searching-aur = Searching AUR for {$query}
no-results = No results found
results = Results:
installed-repo-packages = Installed Repo Packages:
installed-aur-packages = Installed AUR Packages:
installed-packages = Installed Packages:
invalid-shell = Invalid shell, {$shell}
zsh-error = Zsh shell completions are currently unsupported due to a bug in the clap_completion crate

# operations::clean
no-orphans = No orphaned packages found
removing-orphans-would = 
    Removing orphans would uninstall the following packages:
    {$packages}
failed-remove-orphans = Failed to remove orphans
success-remove-orphans = Successfully removed orphans
clear-pkgbuild-cache = Clear Amethyst's internal PKGBUILD cache?
clear-pacman-cache = Also clear Pacman's package cache?
failed-clear-cache = Failed to clear package cache, {$error}
success-clear-cache = Successfully cleared package cache

# operations::install
installing-packages-from-repos = Installing packages {$packages} from repos
error-install = An error occured while installing packages: {$error}, aborting

# operations::search
out-of-date = out of date: since
installed = installed

# operations::uninstall
failed-remove-pkgs = Failed to remove packages

# operations::upgrade
failed-upgrade-repo-pkgs = Failed to upgrade repo packages
success-upgrade-repo-pkgs = Successfully upgraded repo packages
couldnt-find-remote-pkg = Could not find the remote package for {$pkg}
no-upgrades-aur-package = No upgrades available for installed AUR packages
scanning-for-pacnew = Scanning for .pacnew files post-upgrade

# logging::output
repo-dependencies = Repo dependencies
aur-dependencies = AUR dependencies
repo-make-dependencies = Repo make dependencies
aur-make-dependencies = AUR make dependencies
repo-check-dependencies = Repo check dependencies
aur-check-dependencies = AUR check dependencies
version = version
votes = votes
capital-installed = Installed

# operations::aur_install::aur_fetch
fetching-pkg-info = Fetching package information
couldnt-find-all-pkgs = Couldn't find all packages
all-pkgs-found = All packages found
some-pkgs-already-installed = Some packages are already installed. Continue anyway?
do-you-want-to-install = Do you want to install these packages and package dependencies?

# operations::aur_install::common
downloading-sources = Downloading sources
pulling-latest-changes = Pulling latest changes
cloning-aur-repo = Cloning aur repository
down-and-ext-files = Downloading and extracting files
downloaded = Downloaded!
dependency-cycle = Dependency cycle detected. Aborting installation.
building-packages = Building packages
built = Built
installing-packages = Installing packages
building-package = Building Package
build-failed = Build failed!
couldnt-find-pkg-produced = Could not find package {$pkg} in produced packages
review-build-log = Do you want to review the build log?

# operations::aur_install::aur_review
select-pkgs-review = Select packages to review
do-you-still-want-to-install = Do you still want to install those packages?
reviewing = Reviewing
select-file-review = Select a file to review
done-reviewing-pkg = Done reviewing {$pkg}
review = Review {$pkg}?

# operations::aur_install::aur_download
all-sources-ready = All sources are ready.

# interact::theme
no-selections = No selections

# operations::aur_install::repo_dependency_installation
installing-repo-deps = Installing repo dependencies

# operations::aur_install::aur_dependency_installation
installing-from-aur = Installing {$amountOfPkgs} from the AUR

# operations::aur_install::make_dependency_removal
remove-installed-make-deps = Do you want to remove the installed make dependencies?
done = Done!

# operations::aur_install
aur-rpc-crash = AUR RPC Call failed with: {$error}
failed-to-build = Failed to build
makepkg-failed = makepkg failed
unknown-error = Unknown error

# internal::error
non-zero-exit = Exited with non zero code
build-step-violation = AUR build violated build steps
build-error = Failed to build package
user-cancel = Cancelled by user
makepkg-err = Failed to run makepkg
error-occurred = An error occurred

# internal::detect
sudo-prompt-failed = Sudo prompt failed
scanning-pacnew-files = Scanning for pacnew files
no-pacnew-found = No .pacnew files found
pacnew-found = pacnew files found
pacnew-warning =
    It appears that at least one program you have installed / upgraded has installed a .pacnew config file.
    These are created when you have modified a program's configuration, and a package upgrade could not automatically merge the new file.
    You can deal with those files by running
run-pacdiff-now = Would you like to run pacdiff now?
pacdiff-warning =
    Pacdiff uses vimdiff by default to edit files for merging. You can focus panes by mousing over them and pressing left click, and scroll up and down using your mouse's scroll wheel (or the arrow keys). To exit vimdiff, press the following key combination: ESC, :qa!, ENTER
    You can suppress this warning in the future by setting `pacdiff_warn` to "false" in ~/.config/ame/config.toml

# internal::config
config-docs = # See https://getcryst.al/docs/amethyst/config for more information on config keys

# internal::paging
quit = quit
search = search
next-result = next result
previous-result = previous result

# --help
verbose = Sets the level of verbosity
no-confirm = Complete operation without prompting user
quiet = Make some commands have less output (only clean, upgrade, and install are supported)
sudoloop = Loops sudo in the background to ensure it doesn't time out during long builds
cachedir = Sets a custom AUR clone and build directory for the specified operation
install = Installs or searches for a package in either the AUR or the Pacman-defined repositories
remove = Removes a previously installed package
search = Searches for packages matching a provided pattern in the AUR/repos [aliases: -Ss]
query = Queries installed packages
upgrade = Upgrades locally installed packages to their latest versions (Default)
gencomp = Generates shell completions for supported shells (bash, fish, elvish, pwsh)
clean = Removes all orphaned packages
diff = Runs pacdiff
install-packages = The name of the package(s) to install or search for
install-aur = Operate only on AUR packages
install-repo = Operate only on repo packages
install-search = Search packages for a given pattern instead of installing
install-by = Searches by a specific field
remove-packages = The name of the package(s) to remove
query-aur = Lists AUR/foreign packages [-Qa, -Qm]
query-repo = Lists repo/native packages [-Qr, -Qn]
query-explicit = Lists explicitly installed packages [-Qe]
query-info = Get information about a specific package
query-owns = Get information about which package owns a file
upgrade-repo = Upgrades only repo/native packages
upgrade-aur = Upgrades only from the AUR
gencomp-shell = The shell to generate completions for (bash, fish, elvish, pwsh, fig)
description = A fast and efficient AUR helper
