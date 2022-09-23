# used across files
exiting = Exiting
packages = { $pkgNum ->
    [one] package
    *[other] packages
}
missing-deps = Fehlende Packetabhängigkeiten
continue = Fortführen?
# main
run-as-root = Amethyst als root auszuführen ist nicht erlaubt und kann zu kaputten systemen führen. Amethyst wird sie statdessen nach root rechten fragen wenn diese nötig sind.
following-packages = Folgende Pakete wurden in der AUR gefunden:
aur-warning = 
    Die AUR ist eine Quelle an Paketen die von den benutzern hochgeladen werden, welche nicht immer sicher sind.
    Es wird empfohlen die PKGBUILD der Pakete zu überprüfen bevor sie diese Pakete installieren, da manche PKGBUILDs gefährlich sein können. 
    Diese Warnung kann in der Konfigurationsdatei abgeschaltet werden.
are-you-sure = Sind sie sich sicher, dass sie fortfahren möchten?
uninstalling-packages = Folgende Pakete werden deinstalliert: {$packages}
system-upgrade = Führe systemupgrade aus
removing-orphans = Deinstalliere nicht benötigte Pakete
couldnt-find-packages = Folgende pakete wurden nicht in den Paketquellen oder der AUR gefunden: {$packages}
searching-repos = Suche Paketquellen nach {$query}
searching-aur = Suche AUR nach {$query}
no-results = Keine Ergebnisse gefunden
results = Ergebnisse:
installed-repo-packages = Pakete installiert von den Paketquellen:
installed-aur-packages = Pakete installiert aus der AUR:
installed-packages = Installierte Pakete:
invalid-shell = Nicht unterstützte Shell: {$shell}
zsh-error = Zsh shell Vervollständigung werden zurzeit wegen einem fehler in der clap_completion crate nicht unterstützt
# operations::clean
no-orphans = Keine ungebrauchten Pakete gefunden
removing-orphans-would = 
    Das entfernen von Ungebrauchten Paketen würde folgende Pakete entfernen:
    {$packages}
failed-remove-orphans = Ungebrauchte Pakete konnten nicht entfernt werden
success-remove-orphans = Ungebrauchte Pakete wurden erfolgreich entfernt
clear-pkgbuild-cache = Amethysts internen PKGBUILD Cache leeren?
clear-pacman-cache = Pacmans Paketcache auch leeren?
failed-clear-cache = Paketcache konnte nicht geleert werden, {$error}
success-clear-cache = Paketcache wurde erfolgreich geleert
# operations::install
installing-packages-from-repos = Pakete {$packages} werden von den Paketquellen installiert
error-install = Ein Fehler geschah wärend dem installieren von Paketen: {$error}, breche die installation ab
# operations::search
out-of-date = out of date: since
installed = installiert
# operations::uninstall
failed-remove-pkgs = Pakete konnten nicht deinstalliert werden
# operations::upgrade
failed-upgrade-repo-pkgs = Pakete von Paketquellen konnten nicht aktualisiert werden
success-upgrade-repo-pkgs = Pakete von Paketquellen wurden erfolgreich aktualisiert
couldnt-find-remote-pkg = Remotepaket für {$pkg} konnte nicht gefunden werden
no-upgrades-aur-package = Keine Aktualisierungen für AUR Pakete gefunden
scanning-for-pacnew = Scanne für .pacnew dateien nach Aktualisierung
# logging::output
repo-dependencies = Paketabhängigkeiten aus Paketquellen
aur-dependencies = Paketabhängigkeiten aus AUR
repo-make-dependencies = Paketabhängigkeiten zum machen der Pakete aus Paketquellen 
aur-make-dependencies = Paketabhängigkeiten zum machen der Pakete aus der AUR
version = Version
votes = Stimmen
capital-installed = Installiert
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
aur-rpc-crash = AUR RPC Call failed with
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
scanning-pacnew-files = Scanning for pacnew files"
no-pacnew-found = No .pacnew files found
pacnew-found = pacnew files found
pacnew-warning =
    It appears that at least one program you have installed / upgraded has installed a .pacnew config file.
    These are created when you have modified a program's configuration, and a package upgrade could not automatically merge the new file.
    You can deal with those files by running
run-pacdiff-now = Would you like to run pacdiff now?
pacdiff-warning =
    Pacdiff uses vimdiff by default to edit files for merging. You can focus panes by mousing over them and pressing left click, and scroll up and down using your mouse's scroll wheel (or the arrow keys). To exit vimdiff, press the following key combination: ESC, :qa!, ENTER
    You can surpress this warning in the future by setting `pacdiff_warn` to "false" in ~/.config/ame/config.toml
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
query-info = Get information about a specific package
upgrade-repo = Upgrades only repo/native packages
upgrade-aur = Upgrades only from the AUR
gencomp-shell = The shell to generate completions for (bash, fish, elvish, pwsh, fig)
review = {$pkg} überprüfen?
