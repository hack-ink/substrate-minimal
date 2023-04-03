#!/bin/sh

cargo publish --locked -p subcryptor
cargo publish --locked -p subhasher
cargo publish --locked -p submetadatan
cargo publish --locked -p subrpcer
cargo publish --locked -p subruntimer
cargo publish --locked -p subspector
# substorager depends on subhasher, sleep for 30s
sleep 30
cargo publish --locked -p substorager
cargo publish --locked -p subversioner
