installed = zainstalowane
# operations::uninstall
failed-remove-pkgs = Nie udało się usunąć pakietów
# operations::upgrade
failed-upgrade-repo-pkgs = Nie udało się zaktualizować pakietów z repozytoriów
success-upgrade-repo-pkgs = Pomyślnie zaktualizowano pakiety z repozytoriów
couldnt-find-remote-pkg = Nie można znaleźć pakietu zdalnego dla { $pkg }
no-upgrades-aur-package = Brak dostępnych aktualizacji dla zainstalowanych pakietów AUR
scanning-for-pacnew = Skanowanie w poszukiwaniu plików .pacnew po aktualizacji
# logging::output
repo-dependencies = Zależności repozytorium
aur-dependencies = Zależności AUR
repo-make-dependencies = Zależności do kompilowania pakietów z repozytoriów
missing-deps = Brakujące zależności
continue = Kontynuować?
# main
run-as-root = Uruchamianie Amethyst jako root jest niedozwolone, ponieważ może prowadzić do awarii systemu. Zamiast tego Amethyst wyświetli komunikat, gdy będzie potrzebować uprawnień administratora.
following-packages = Następujące pakiety zostały znalezione w AUR:
are-you-sure = Jesteś pewien że chcesz kontynuować?
uninstalling-packages = Odinstalowywanie następujących pakietów: { $packages }
system-upgrade = Aktualizowanie wszystkich zainstalowanych pakietów
removing-orphans = Usuwanie niepotrzebnych pakietów
couldnt-find-packages = Nie odnaleziono : { $packages } w repozytoriach lub AUR
searching-repos = Szukanie { $query } w repozytoriach
searching-aur = Szukanie { $query } w AUR
results = Wyniki:
no-results = Niczego nie znaleziono
installed-aur-packages = Zainstalowane pakiety AUR:
installed-repo-packages = Zainstalowane pakiety z repozytoriów:
installed-packages = Zainstalowane pakiety:
# operations::clean
no-orphans = Nie znaleziono niepotrzebnych pakietów
failed-remove-orphans = Nie udało się usunąć niepotrzebnych pakietów
removing-orphans-would =
    Usuwanie niepotrzebnych pakietów usunie:
    { $packages }
clear-pacman-cache = Wyczyścić także pamięć podręczną pakietów Pacmana?
success-clear-cache = Pomyślnie wyczyszczono pamięć podręczną pakietów
success-remove-orphans = Pomyślnie usunięto niepotrzebne pakiety
error-install = Wystąpił błąd podczas instalacji pakietów: { $error }, przerywanie
failed-clear-cache = Nie udało się wyczyścić pamięci podręcznej pakietów,  { $error }
version = wersja
votes = głosy
capital-installed = Zainstalowane
aur-make-dependencies = Zależności do kompilowania pakietów AUR
aur-warning = AUR jest źródłem pakietów/skryptów przesłanych przez użytkowników i nie zawsze jest bezpieczny w użyciu.
clear-pkgbuild-cache = Wyczyścić wewnętrzną pamięć podręczną PKGBUILD?
# operations::install
installing-packages-from-repos = Instalowanie { $packages } z repozytoriów
zsh-error = Uzupełnienia powłoki Zsh są obecnie nieobsługiwane z powodu błędu w clap_completion
