# used across files
exiting = Esco
packages = { $pkgNum ->
    [one] pacchetto
    *[other] pacchetti
}
missing-deps = Dipendenze mancanti
continue = Continuare?

# main
run-as-root = Non è consentito usare Amethyst come super utente poiché potrebbe danneggiare il sistema. Amethyst chiederà l'autenticazione quando necessario.
following-packages = I seguenti pacchetti non sono stati trovati in AUR:
aur-warning = 
    L'AUR è una fonte di pacchetti creati da utenti e non sempre è sicura.
    È consigliato esaminare il PKGBUILD di qualsiasi pacchetto AUR prima di installarlo poiché alcuni potrebbero essere dannosi. 
    Questo avviso può essere disattivato nel file di configurazione.
are-you-sure = Continuare?
uninstalling-packages = Disinstallazione dei pacchetti: {$packages}
system-upgrade = Aggiornamento del sistema
removing-orphans = Rimozione dei pacchetti orfani
couldnt-find-packages = Impossibile trovare i pacchetti: {$packages} nei repository o in AUR
searching-repos = Ricerca di {$query} nei repository
searching-aur = Ricerca di {$query} in AUR
no-results = Nessun risultato
results = Risultati:
installed-repo-packages = Pacchetti repository installati:
installed-aur-packages = Pacchetti AUR installati:
installed-packages = Pacchetti installati:
invalid-shell = La shell {$shell} non è supportata
zsh-error = L'autocompletamento Zsh non è supportato a causa di un bug nel crate clap_completion

# operations::clean
no-orphans = Nessun pacchetto orfano trovato
removing-orphans-would = 
    I seguenti pacchetti orfani saranno rimossi:
    {$packages}
failed-remove-orphans = Impossibile rimuovere i pacchetti orfani
success-remove-orphans = I pacchetti orfani sono stati rimossi
clear-pkgbuild-cache = Pulire la cache interna dei PKGBUILD di Amethyst?
clear-pacman-cache = Pulire anche la cache dei pacchetti di Pacman?
failed-clear-cache = Impossibile pulire la cache dei pacchetti: {$error}
success-clear-cache = La cache dei pacchetti è stata pulita

# operations::install
installing-packages-from-repos = Installazione di {$packages} dai repository
error-install = Si è verificato un errore durante l'installazione: {$error}, interrompo

# operations::search
out-of-date = obsoleto: dal
installed = installato

# operations::uninstall
failed-remove-pkgs = Impossibile installare i pacchetti

# operations::upgrade
failed-upgrade-repo-pkgs = Impossibile aggiornare i pacchetti repository
success-upgrade-repo-pkgs = I pacchetti repository sono stati aggiornati
couldnt-find-remote-pkg = Impossibile trovare il pacchetto {$pkg}
no-upgrades-aur-package = Nessun aggiornamento disponibile per i pacchetti AUR
scanning-for-pacnew = Ricerca di file .pacnew post-aggiornamento

# logging::output
repo-dependencies = Dipendenze repository
aur-dependencies = Dipendenze AUR
repo-make-dependencies = Dipendenze repository per la compilazione
aur-make-dependencies = Dipendenze AUR per la compilazione
version = versione
votes = voti
capital-installed = Installato

# operations::aur_install::aur_fetch
fetching-pkg-info = Raccoglimento delle informazioni dei pacchetti
couldnt-find-all-pkgs = Alcuni pacchetti non sono stati trovati
all-pkgs-found = Tutti i pacchetti sono stati trovati
some-pkgs-already-installed = Alcuni pacchetti sono già installati. Continuare comunque?
do-you-want-to-install = Vuoi installare questi pacchetti e dipendenze?

# operations::aur_install::common
downloading-sources = Scaricamento delle sorgenti
pulling-latest-changes = Recupero dei cambiamenti più recenti
cloning-aur-repo = Clonazione del repository AUR
down-and-ext-files = Scaricamento e estrazione dei file
downloaded = Scaricato!
dependency-cycle = È stato rilevato un ciclo di dipendenze. L'installazione è stata interrotta.
building-packages = Compilazione dei pacchetti
built = Compilato
installing-packages = Installazione dei pacchetti
building-package = Compilazione del pacchetti
build-failed = La compilazione non è riuscita!
couldnt-find-pkg-produced = Impossibile trovare {$pkg} nei pacchetti compilati
review-build-log = Vuoi esaminare il registro di compilazione?

# operations::aur_install::aur_review
select-pkgs-review = Seleziona pacchetti da esaminare
do-you-still-want-to-install = Vuoi ancora installare i pacchetti?
reviewing = Esaminazione
select-file-review = Selezionare un file da esaminare
done-reviewing-pkg = Esaminazione di {$pkg} conclusa
review = Esaminare {$pkg}?

# operations::aur_install::aur_download
all-sources-ready = Le sorgenti sono pronte.

# interact::theme
no-selections = La selezione è vuota

# operations::aur_install::repo_dependency_installation
installing-repo-deps = Installazione delle dipendenze repository

# operations::aur_install::aur_dependency_installation
installing-from-aur = Installazione di {$amountOfPkgs} pacchetti AUR

# operations::aur_install::make_dependency_removal
remove-installed-make-deps = Rimuovere le dipendenze di compilazione?
done = Fatto!

# operations::aur_install
aur-rpc-crash = La chiamata RCP a AUR non è riuscita: {$error}
failed-to-build = Errore di compilazione
makepkg-failed = Errore di makepkg
unknown-error = Errore sconosciuto

# internal::error
non-zero-exit = Codice di uscita diverso da zero
build-step-violation = La compilazione AUR ha violato le fasi di compilazione
build-error = Errore nella compilazione del pacchetto
user-cancel = Annullato dall'utente
makepkg-err = Impossibile eseguire makepkg
error-occurred = Si è verificato un errore

# internal::detect
sudo-prompt-failed = Errore nella richiesta di sudo
scanning-pacnew-files = Ricerca di file pacnew
no-pacnew-found = Nessun file .pacnew trovato
pacnew-found = file pacnew trovati
pacnew-warning =
    Uno o più programmi installati / aggiornati hanno installato un file di configurazione .pacnew.
    I file .pacnew vengono creati quando la configurazione di un programma è stata modificata e non è possibile unire automaticamente il file di configurazione aggiornato.
    Puoi gestire questi file con
run-pacdiff-now = Eseguire pacdiff adesso?
pacdiff-warning =
    Pacdiff usa vimdiff di default per modificare e unire i file. Puoi focalizzare i pannelli puntandoli e con un click sinistro e scorrere con la rotella del mouse (o le frecce). Per uscire da vimdiff, usare la seguente combinazione di tasti: ESC, :qa!, INVIO
    Puoi disattivare questo avviso impostando `pacdiff_warn` a "false" in ~/.config/ame/config.toml

# internal::config
config-docs = # Visitare https://getcryst.al/docs/amethyst/config Per ulteriori informazioni sulla configurazione di amethyst

# internal::paging
quit = esci
search = cerca
next-result = risultato successivo
previous-result = risultato precedente

# --help
verbose = Mostra più informazioni
no-confirm = Completa l'operazione senza chiedere conferme
quiet = Mostra meno informazioni (solo per clean, upgrade e install)
sudoloop = Mantiene sudo in esecuzione per evitare che scada durante lunghe compilazioni
cachedir = Imposta un percorso di clonazione e compilazione AUR alternativo per l'operazione specificata
install = Installa o cerca un pacchetto in AUR o nei repository di Pacman
remove = Rimuove un pacchetto installato
search = Cerca in AUR e nei repository pacchetti che corrispondono a un termine di ricerca [alias: -Ss]
query = Elenca i pacchetti installati
upgrade = Aggiorna i pacchetti installati alla versione più recente (predefinito)
gencomp = Genera completamenti per le shell supportate (bash, fish, elvish, pwsh)
clean = Rimuove tutti i pacchetti orfani
diff = Esegue pacdiff
install-packages = Il nome dei pacchetti da installare o cercare
install-aur = Opera solo sui pacchetti AUR
install-repo = Opera solo sui pacchetti repository
install-search = Cerca pacchetti che corrispondono a un termine di ricerca anziché installarli
install-by = Cerca in base a un campo specifico
remove-packages = Il nome dei pacchetti da rimuovere
query-aur = Elenca i pacchetti AUR [-Qa, -Qm]
query-repo = Elenca i pacchetti repository [-Qr, -Qn]
query-info = Ottieni informazioni su un pacchetto
upgrade-repo = Aggiorna solo i pacchetti repository
upgrade-aur = Aggiorna solo i pacchetti aur
gencomp-shell = La shell per cui generare i completamenti (bash, fish, elvish, pwsh, fig)
