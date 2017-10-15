FROM archlinux/base

RUN echo $'[archlinuxfr]\n\
SigLevel = Never\n\
Server = http://repo.archlinux.fr/$arch\n' >> /etc/pacman.conf && \
    pacman -Syu --noconfirm && \
    pacman -S --noconfirm yaourt base-devel git wget customizepkg && \
    useradd -m builder && \
    echo "builder ALL=NOPASSWD: ALL" >> /etc/sudoers && \
    mkdir -p /etc/customizepkg.d/ && \
    echo "replace#global#OSX_VERSION_MIN=10.6#OSX_VERSION_MIN=10.7" > /etc/customizepkg.d/osxcross-git
USER builder
WORKDIR /home/builder
RUN yaourt -S --noconfirm osxcross-git
RUN curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain nightly -y && \
    echo $'[target.x86_64-apple-darwin]\n\
linker = "x86_64-apple-darwin15-cc"\n\
ar = "x86_64-apple-darwin15-ar"\n' > ~/.cargo/config && \
    ~/.cargo/bin/rustup target add x86_64-apple-darwin

ENV PATH=$PATH:~/.cargo/bin/:/usr/local/osx-ndk-x86/bin

COPY --chown=builder . inc

WORKDIR /home/builder/inc

RUN /home/builder/.cargo/bin/cargo build --target=x86_64-apple-darwin --release