#!/bin/bash

cargo install --vcs=none rque
cat _/dep.txt >> rque/Cargo.toml
rm _/dep.txt
cp -va _/* rque/
rm -rfv _
