FROM ethankhall/rust-cross-build:latest

RUN curl "https://www.archlinux.org/mirrorlist/?country=US&protocol=http&protocol=https&ip_version=4&ip_version=6&use_mirror_status=on" | grep ".edu" > /tmp/mirrorlist && \
    sed -i 's/^#Server/Server/' /tmp/mirrorlist && \
    sudo mv /tmp/mirrorlist /etc/pacman.d/mirrorlist && \
    yaourt -Sy --noconfirm openssh tree
COPY --chown=builder .git inc/

ARG VERSION=0.0.1
WORKDIR /home/builder/inc

RUN tree
RUN git --work-tree=/home/builder/inc/ reset --hard v${VERSION}
RUN find /home/builder/inc -name Cargo.toml -exec sed -i.bck "s/version = \".*\"/version = \"$VERSION\"/g" {} \;
ENV PKG_CONFIG_ALLOW_CROSS 1
RUN /home/builder/.cargo/bin/cargo build --release
RUN /home/builder/.cargo/bin/cargo build --target=x86_64-apple-darwin --release
ENV PKG_CONFIG_PATH /usr/i686-w64-mingw32/lib/pkgconfig
RUN /home/builder/.cargo/bin/cargo build --target=x86_64-pc-windows-gnu --release

RUN mkdir /home/builder/output && \
    cp target/release/inc /home/builder/output/inc-linux-$VERSION && \ 
    cp target/x86_64-apple-darwin/release/inc /home/builder/output/inc-darwin-$VERSION && \ 
    cp target/x86_64-pc-windows-gnu/release/inc.exe /home/builder/output/inc-windows-$VERSION.exe && \ 
    tree /home/builder/output