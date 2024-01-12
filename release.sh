#!/bin/bash

git status -s | grep ^ && exit 1

cargo install cargo-release cargo-get set-cargo-version

cargo-release release "${RELEASE_TYPE=patch}" -x --no-confirm || exit 1

cargo release version patch -x --no-confirm
VERSION=$(cargo get package.version)
set-cargo-version ./Cargo.toml "${VERSION}"-SNAPSHOT
git add ./Cargo.toml Cargo.lock
git commit -m "prepare for further development"
git push