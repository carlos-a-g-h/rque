#!/bin/bash

cargo init rque
cat _/dep.txt >> rque/Cargo.toml
rm _/dep.txt
cp -va _/* rque/
