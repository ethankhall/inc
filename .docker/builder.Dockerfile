FROM archlinux/base

RUN pacman -Syu && pacman -S --noconfirm wget base-devel clang git openssh
RUN mkdir /tmp/pkg && \
    wget -q --directory-prefix=/tmp/pkg https://dl.bintray.com/ethankhall/generic/packages/osxcross-git-0.14-1-x86_64.pkg.tar.xz && \
    wget -q --directory-prefix=/tmp/pkg https://dl.bintray.com/ethankhall/generic/packages/mingw-w64-winpthreads-5.0.3-1-any.pkg.tar.xz && \
    wget -q --directory-prefix=/tmp/pkg https://dl.bintray.com/ethankhall/generic/packages/mingw-w64-headers-5.0.3-1-any.pkg.tar.xz && \
    wget -q --directory-prefix=/tmp/pkg https://dl.bintray.com/ethankhall/generic/packages/mingw-w64-crt-5.0.3-1-any.pkg.tar.xz && \
    wget -q --directory-prefix=/tmp/pkg https://dl.bintray.com/ethankhall/generic/packages/mingw-w64-binutils-2.29-1-x86_64.pkg.tar.xz && \
    wget -q --directory-prefix=/tmp/pkg https://dl.bintray.com/ethankhall/generic/packages/mingw-w64-gcc-7.3.0-1-x86_64.pkg.tar.xz && \
    pacman -U --noconfirm /tmp/pkg/*

RUN curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain nightly -y
COPY cargo-config /root/.cargo/config

RUN /root/.cargo/bin/rustup target add x86_64-apple-darwin && \
    /root/.cargo/bin/rustup target add x86_64-pc-windows-gnu && \
    /root/.cargo/bin/rustup target add x86_64-unknown-linux-musl

ENV PATH=$PATH:/root/.cargo/bin/:/usr/local/osx-ndk-x86/bin:/usr/x86_64-w64-mingw32/bin