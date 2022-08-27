ARG BASE_IMAGE=docker.io/archlinux:latest
FROM ${BASE_IMAGE} as build_base
RUN pacman -Syu --noconfirm
RUN pacman -S --noconfirm base-devel curl bash
RUN curl https://sh.rustup.rs -sSf | bash -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

FROM build_base as builder
WORKDIR /usr/src
RUN cargo new amethyst
WORKDIR /usr/src/amethyst
COPY Cargo.toml Cargo.lock ./
RUN mkdir target
COPY target/debug/build ./target/debug/build
RUN cargo fetch
COPY src ./src
RUN cargo build --frozen
RUN mkdir /tmp/ame
RUN cp target/debug/ame /tmp/ame/

FROM ${BASE_IMAGE} as runtime
RUN pacman -Syu --noconfirm
RUN pacman -S --noconfirm base-devel zsh wget vim git binutils fakeroot pacman-contrib sudo
RUN useradd -r -d /home/ame -p $(echo "ame" | openssl passwd -1 -stdin) ame -G wheel
RUN echo '%wheel ALL=(ALL) NOPASSWD:ALL' >> /etc/sudoers
RUN mkdir /home/ame
RUN chown ame:ame /home/ame
COPY --from=builder /tmp/ame/ame /usr/bin/
RUN rm -f $(pacdiff -o -f)
USER ame
RUN mkdir -p /home/ame/.local/share
RUN touch /home/ame/.zshrc
ENV AME_LOG=debug,hyper=info,mio=info,want=info
ENTRYPOINT ["zsh"]