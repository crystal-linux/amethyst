# used across files
exiting = Exiting
packages = { $pkgNum ->
    [one] paquete
    *[other] paquetes
}
missing-deps = Dependencias faltantes
continue = Continuar?

# main
run-as-root = No se permite ejecutar Amethyst como root, ya que puede povocar routuras en el sistema. En su lugar, Amethyst le avisará cuando requiera permisos de superusuario.
following-packages = Los siguientes paquetes fueron encontrados en el AUR:
aur-warning = 
    El AUR es una fuente de paquetes/scripts subidos por usuarios y no siempre es seguro su uso.
    Por favor compruebe el PKGBUILD de cualquier descarga proveniente del AUR antes de instalarlo, ya que algunos PKGBUILDs pueden ser potencialmente maliciosos. 
    Esta advertencia se puede desactivar en el archivo de configuración.
are-you-sure = Estás seguro que quieres continuar?
uninstalling-packages = Desinstalando paquetes: {$packages}
system-upgrade = Realizando una actualización del sistema
removing-orphans = Removiendo paquetes huérfanos
couldnt-find-packages = No se pudieron encontrar los paquetes: {$packages} en repositorios o en el AUR
searching-repos = Buscando repositorios para {$query}
searching-aur = Buscando AUR para {$query}
no-results = No se han encontrado resultados
results = Resultados:
installed-repo-packages = Paquetes de repositorio instalados:
installed-aur-packages = Paquetes de AUR instalados:
installed-packages = Paquetes instalados:
invalid-shell = Intérprete Inválido, {$shell}
zsh-error = Actualmente las terminaciones del intérprete Zsh no están soportadas debido a un bug en el crate clap_completion 

# operations::clean
no-orphans = No se han encontrado paquetes huérfanos
removing-orphans-would = 
    Remover los paquetes huéranos desinstalaría los siguientes paquetes:
    {$packages}
failed-remove-orphans = No se pudieron remover los paquetes huérfanos
success-remove-orphans = Se han removido los paquetes huérfanos exitosamente
clear-pkgbuild-cache = Limpiar el caché PKGBUILD interno de Amethyst?
clear-pacman-cache = También limpiar el caché de paquetes de Pacman?
failed-clear-cache = No se pudo limpiar el caché del paquete, {$error}
success-clear-cache = Se ha limpiado el caché del paquete exitosamente

# operations::install
installing-packages-from-repos = Instalando paquetes {$packages} desde repositorios
error-install = Ha ocurrido un error en la instalación de paquetes: {$error}, abortando

# operations::search
out-of-date = antiguo: desde
installed = instalado

# operations::uninstall
failed-remove-pkgs = Ha ocurrido un error al remover paquetes

# operations::upgrade
failed-upgrade-repo-pkgs = Ha ocurrido un error al actualizar paquetes de repositorio 
success-upgrade-repo-pkgs = Paquetes de repositorio actualizados exitosamente 
couldnt-find-remote-pkg = No se pudo encontrar el paquete remoto para {$pkg}
no-upgrades-aur-package = No hay actualizaciones disponibles para paquetes instalados desde el AUR
scanning-for-pacnew = Escaneando por .pacnew archivos post-actualización

# logging::output
repo-dependencies = Dependencias de repositorio
aur-dependencies = Dependencias de AUR
repo-make-dependencies = Repo make dependencies
aur-make-dependencies = AUR make dependencies
repo-check-dependencies = Repo check dependencies
aur-check-dependencies = AUR check dependencies
version = versión
votes = votos
capital-installed = Instalado

# operations::aur_install::aur_fetch
fetching-pkg-info = Obteniendo información del paquete
couldnt-find-all-pkgs = No se pudieron encontrar todos los paquetes
all-pkgs-found = Se han encontrado todos los paquetes
some-pkgs-already-installed = Algunos paquetes están instalados actualmente. Continuar de todas formas?
do-you-want-to-install = Quieres instalar estos paquetes y dependencias de paquete?

# operations::aur_install::common
downloading-sources = Descargando fuentes
pulling-latest-changes = Extrayendo últimos cambios
cloning-aur-repo = Clonando repositorio aur
down-and-ext-files = Descargando y extrayendo archivos
downloaded = Descargado!
dependency-cycle = Ciclo de dependencia detectado. Abortando instalación.
building-packages = Construyendo paquetes
built = Construido
installing-packages = Instalando paquetes
building-package = Construyendo paquete
build-failed = Construcción fallida!
couldnt-find-pkg-produced = No se pudo encontrar el paquete {$pkg} en los paquetes producidos
review-build-log = Quieres comprobar el build log?

# operations::aur_install::aur_review
select-pkgs-review = Selecciona paquetes para comprobar
do-you-still-want-to-install = Aún quieres instalar esos paquetes?
reviewing = Comprobando
select-file-review = Selecciona un archivo para comprobar
done-reviewing-pkg = Comprobación hecha {$pkg}
review = Comprobar {$pkg}?

# operations::aur_install::aur_download
all-sources-ready = Todas las fuentes están listas.

# interact::theme
no-selections = Sin elecciones

# operations::aur_install::repo_dependency_installation
installing-repo-deps = Instalando dependencias de repositorio

# operations::aur_install::aur_dependency_installation
installing-from-aur = Instalando {$amountOfPkgs} desde el AUR

# operations::aur_install::make_dependency_removal
remove-installed-make-deps = Quieres remover las dependencias make instaladas?
done = Hecho!

# operations::aur_install
aur-rpc-crash = La llamada AUR RPC falló con: {$error}
failed-to-build = Ha ocurrido un error al construir
makepkg-failed = makepkg fallido
unknown-error = Error desconocido

# internal::error
non-zero-exit = Exited with non zero code
build-step-violation = AUR build violated build steps
build-error = Failed to build package
user-cancel = Cancelled by user
makepkg-err = Failed to run makepkg
error-occurred = An error occurred

# internal::detect
sudo-prompt-failed = Solicitud de sudo fallida
scanning-pacnew-files = Escaneando por archivos .pacnew
no-pacnew-found = No se han encontrado archivos .pacnew
pacnew-found = Se han encontrado archivos .pacnew
pacnew-warning =
    Parece que al menos un programa que ha instalado/actualizado ha instalado un archivo de configuración .pacnew.
    Estos se crean cuando ha modificado la configuración de un programa y una actualización de paquete no pudo fusionar automáticamente el nuevo archivo.
    Puedes lidiar con estos archivos ejecutando
run-pacdiff-now = Te gustaría ejecutar pacdiff ahora?
pacdiff-warning =
    Pacdiff usa vimdiff de forma predeterminada para editar archivos para fusionarlos. Puede enfocar los paneles pasando el mouse sobre ellos y presionando el botón izquierdo, y desplazándose hacia arriba y hacia abajo usando la rueda de desplazamiento del mouse (o las teclas de flecha). Para salir de vimdiff, presione la siguiente combinación de teclas: ESC, :qa!, ENTER
    Puede suprimir esta advertencia en el futuro configurando `pacdiff_warn` en "false" en ~/.config/ame/config.toml

# internal::config
config-docs = # Consulte https://getcryst.al/docs/amethyst/config para obtener más información sobre las claves de configuración

# internal::paging
quit = quitar
search = buscar
next-result = siguiente resultado
previous-result = resultado previo

# --help
verbose = Establece el nivel de verbosidad
no-confirm = Completa la operación sin preguntar al usuario
quiet = Hacer que algunos comandos sea menos verbosos (only clean, upgrade, and install están soportados)
sudoloop = Realiza un bucle de sudo en segundo plano para garantizar que no se agote el tiempo de espera durante compilaciones largas
cachedir = Establece un clon de AUR personalizado y un directorio de compilación para la operación especificada
install = Instala o busca un paquete en el AUR o en los repositorios definidos por Pacman
remove = Remueve un paquete instalado previamente
search = Busca paquetes que coincidan con un patrón proporcionado en AUR/repos [alias: -Ss]
query = Consulta paquetes instalados
upgrade = Actualiza paquetes instalados localmente a su última versión (Por defecto)
gencomp = Genera terminaciones de intérprete para intérpretes compatibles (bash, fish, elvish, pwsh)
clean = Remueve todos los paquetes huérfanos 
diff = Ejecuta pacdiff
install-packages = El nombre del paquete(s) a instalar o buscar
install-aur = Opera únicamente en paquetes AUR
install-repo = Opera únicamente en paquetes repo
install-search = Buscar paquetes por un patrón dado en lugar de instalar
install-by = Busca por un campo específico
remove-packages = El nombre del paquete(s) a remover
query-aur = Lista paquetes AUR/extraños [-Qa, -Qm]
query-repo = Lista paquetes repo/nativos [-Qr, -Qn]
query-info = Obtener información acerca de un paquete en específico
query-owns = Obtener información sobre qué paquete posee un archivo
upgrade-repo = Actualiza únicamente paquetes repo/nativos
upgrade-aur = Actualiza únicamente desde el AUR
gencomp-shell = El intérprete para generar terminaciones para (bash, fish, elvish, pwsh, fig)
description = Un AUR helper rápido y eficente
