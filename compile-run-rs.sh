#!/usr/bin/env sh
# simple script to compile *.rs source code via rustc and run the binary
# the script will delete the old binary
# usage: compile-run-rs.sh source-code.rs

fname_ext=$1
fname=${fname_ext%.*}

rm -rf ./$fname
rustc $fname_ext
./$fname