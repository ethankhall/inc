FROM ethankhall/rust-cross-build:latest

COPY --chown=builder . inc

WORKDIR /home/builder/inc
ENV PKG_CONFIG_ALLOW_CROSS 1
RUN /home/builder/.cargo/bin/cargo build --release
RUN /home/builder/.cargo/bin/cargo build --target=x86_64-apple-darwin --release
ENV PKG_CONFIG_PATH /usr/i686-w64-mingw32/lib/pkgconfig
RUN /home/builder/.cargo/bin/cargo build --target=x86_64-pc-windows-gnu --release
