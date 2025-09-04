dev:
    cargo run

build-win:
    cargo build  --release --target  x86_64-pc-windows-gnu


build:
    cargo build  --release --target x86_64-unknown-linux-musl

release-patch:
    cargo release patch --no-publish --execute

release-minor:
    cargo release minor --no-publish --execute

release-major:
    cargo release major --no-publish --execute