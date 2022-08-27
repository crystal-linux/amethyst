FROM ghcr.io/crystal-linux/crystal:latest

RUN  pacman -S --needed --noconfirm cargo openssl git binutils fakeroot pacman-contrib vim expac less
RUN  useradd -m -G wheel vscode
RUN  bash -c "echo \"vscode ALL=(ALL) NOPASSWD: ALL\" >> /etc/sudoers"

ENV  USER=vscode
