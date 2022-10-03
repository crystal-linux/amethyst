# used across files
exiting = Avslutar
packages =
    { $pkgNum ->
        [ett] paket
       *[annat] paket
    }
missing-deps = Saknade beroenden
continue = Fortsätt?
# main
run-as-root = Att köra Amethyst som root är inte tillåtet eftersom det kan leda till att systemet går sönder. Istället, så kommer Amethyst att fråga dig när det behöver superanvändarbehörigheter.
following-packages = Följande paket hittades i AUR:
aur-warning =
    AUR är en källa för användaruppladdade paket/skript och är inte alltid säkert att använda.
    Se till att granska PKGBUILDs för allt du laddar ner från AUR innan du installerar det, eftersom vissa PKGBUILDs kan vara skadliga. 
    Denna varning kan växlas till av/på i konfigurationsfilen.
are-you-sure = Är du säker att du vill fortsätta?
uninstalling-packages = Avinstallerar paket: { $packages }
system-upgrade = Utför systemuppgradering
removing-orphans = Ta bort föräldralösa paket
couldnt-find-packages = Kunde inte hitta paket: { $packages } i förråd eller på AUR
searching-repos = Söker förråd efter { $query }
searching-aur = Söker AUR efter { $query }
no-results = Inga resultat funna
results = Resultat:
installed-repo-packages = Installerade paket från förråd:
installed-aur-packages = Installerade AUR paket:
installed-packages = Installerade paket:
invalid-shell = Ogiltigt skal, { $shell }
zsh-error = Zsh-skalkompletteringar stöds för närvarande inte på grund av en bugg i clap_completion-lådan
# operations::clean
no-orphans = Inga föräldralösa paket hittades
removing-orphans-would =
    Att ta bort föräldralösa paket skulle avinstallera följande paket:
    { $packages }
failed-remove-orphans = Det gick inte att ta bort föräldralösa paket
success-remove-orphans = Föräldralösa paket har tagits bort
clear-pkgbuild-cache = Rensa Amethysts interna PKGBUILD-cache?
clear-pacman-cache = Rensa också Pacmans paketcache?
failed-clear-cache = Misslyckades med att rensa paketcache, { $error }
success-clear-cache = Framgångsrikt rensade paketcache
# operations::install
installing-packages-from-repos = Installerar paket { $packages } från förråd
error-install = Ett fel inträffade när paket installerades: { $error }, avbryter
# operations::search
out-of-date = inaktuell: sedan
installed = installerades
# operations::uninstall
failed-remove-pkgs = Misslyckades att ta bort paket
# operations::upgrade
failed-upgrade-repo-pkgs = Misslyckades att uppdatera förråd paket
success-upgrade-repo-pkgs = Lyckades att uppgradera förråd paket
couldnt-find-remote-pkg = Kunde inte hitta fjärrpaket för { $pkg }
no-upgrades-aur-package = Inga uppgraderingar tillgängliga för installerade AUR-paket
scanning-for-pacnew = Söker efter .pacnew-filer efter uppgradering
# logging::output
repo-dependencies = Förråd beroenden
aur-dependencies = AUR beroenden
repo-make-dependencies = Förråd byggberoenden
aur-make-dependencies = AUR byggberoenden
version = version
votes = röster
capital-installed = Installerad
# operations::aur_install::aur_fetch
fetching-pkg-info = Hämtar paketinformation
couldnt-find-all-pkgs = Kunde inte hitta alla paket
all-pkgs-found = Alla paket hittades
some-pkgs-already-installed = Vissa paket är redan installerade. Fortsätt ändå?
do-you-want-to-install = Vill du installera dessa paket och paketberoenden?
# operations::aur_install::common
downloading-sources = Laddar ner källkod
pulling-latest-changes = Hämtar de senaste ändringarna
cloning-aur-repo = Klonar aur förråd
down-and-ext-files = Laddar ner och extraherar filer
downloaded = Nedladdat!
dependency-cycle = Beroendecykel upptäckt. Avbryter installationen.
building-packages = Bygger paket
built = Byggd
installing-packages = Installerar paket
building-package = Bygger paket
build-failed = Bygget misslyckades!
couldnt-find-pkg-produced = Det gick inte att hitta paketet { $pkg } i producerade paket
review-build-log = Vill du granska byggloggen?
# operations::aur_install::aur_review
select-pkgs-review = Valda paket att granska
do-you-still-want-to-install = Vill du fortfarande installera dessa paket?
reviewing = Granskar
select-file-review = Välj en fil att granska
done-reviewing-pkg = Klar med granskning { $pkg }
review = Granska { $pkg }?
# operations::aur_install::aur_download
all-sources-ready = Alla källor är redo.
# interact::theme
no-selections = Inga val
# operations::aur_install::repo_dependency_installation
installing-repo-deps = Installerar förråd beroenden
# operations::aur_install::aur_dependency_installation
installing-from-aur = Installerar { $amountOfPkgs } från AUR
# operations::aur_install::make_dependency_removal
remove-installed-make-deps = Vill du ta bort de installerade byggberoendena?
done = Klart!
# operations::aur_install
aur-rpc-crash = AUR RPC-anrop misslyckades med: { $error }
failed-to-build = Misslyckades att bygga
makepkg-failed = makepkg misslyckades
unknown-error = Okänt fel
# internal::error
non-zero-exit = Avslutade med annan kod än noll
build-step-violation = AUR-bygget bröt mot byggstegen
build-error = Misslyckades att bygga paket
user-cancel = Avbruten av användaren
makepkg-err = Misslyckades att köra makepkg
error-occurred = Ett fel inträffade
# internal::detect
sudo-prompt-failed = Sudo prompt misslyckades
scanning-pacnew-files = Letar efter nya pacnew filer
no-pacnew-found = Inga .pacnew filer hittades
pacnew-found = pacnew filer hittades
pacnew-warning =
    Det verkar som om minst ett program du har installerat/uppgraderat har installerat en .pacnew-konfigurationsfil.
    Dessa skapas när du har ändrat ett programs konfiguration, och en paketuppgradering kunde inte automatiskt slå samman den nya filen.
    Du kan hantera dessa filer genom att köra
run-pacdiff-now = Vill du köra pacdiff nu?
pacdiff-warning =
    Pacdiff använder vimdiff som standard för att redigera filer för sammanslagning. Du kan fokusera paneler genom att föra musen över dem och trycka på vänsterklick, och rulla upp och ner med musens rullningshjul (eller piltangenterna). För att avsluta vimdiff, tryck på följande tangentkombination: ESC, :qa!, ENTER
    Du kan få tyst på denna varning i framtiden genom att ställa in `pacdiff_warn` till "false" i ~/.config/ame/config.toml
# internal::config
config-docs = # Se https://getcryst.al/docs/amethyst/config för mer information om konfigurationsnycklar
# internal::paging
quit = avsluta
search = sök
next-result = nästa resultat
previous-result = föregående resultat
# --help
verbose = Ställer in graden av detaljerad information
no-confirm = Slutför operationen utan att uppmana användaren
quiet = Gör att vissa kommandon får mindre utdata (endast rensa, uppgradera och installera stöds)
sudoloop = Slingar sudo i bakgrunden för att säkerställa att det inte tar timeout under långa byggen
cachedir = Ställer in en anpassad AUR-klon och byggkatalog för den angivna operationen
install = Installerar eller söker efter ett paket i antingen AUR eller Pacman-definierade förråd
remove = Tar bort ett tidigare installerat paket
search = Söker efter paket som matchar ett angett mönster i AUR/repos [alias: -Ss]
query = Söker efter installerade paket
upgrade = Uppgraderar lokalt installerade paket till deras senaste versioner (standard)
gencomp = Genererar skalkompletteringar för skal som stöds (bash, fish, elvish, pwsh)
clean = Tar bort alla föräldralösa paket
diff = Kör pacdiff
install-packages = Namnet på paketet/paketen som ska installeras eller sökas efter
install-aur = Operera endast på AUR-paket
install-repo = Operera endast på förrådpaket
install-search = Sök paket efter ett givet mönster istället för att installera
install-by = Söker efter ett specifikt fält
remove-packages = Namnet på paketet/paketen som ska tas bort
query-aur = Listar AUR/främmande paket [-Qa, -Qm]
query-repo = Listar förråd/infödda paket [-Qr, -Qn]
query-info = Få information om ett specifikt paket
upgrade-repo = Uppgraderar endast förråd/infödda-paket
upgrade-aur = Uppgraderar endast från AUR
gencomp-shell = Skal att generera kompletteringar för (bash, fish, elvish, pwsh, fig)
repo-check-dependencies = Repokontroll beroenden
aur-check-dependencies = AUR kontroll beroenden
query-owns = Få information om vilket paket som äger en fil
description = En snabb och effektiv AUR hjälpare
