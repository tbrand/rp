#!/bin/bash

set -eux

cargo build --release

function init() {
    rm -rf dir tar && mkdir dir
    head -c 67108864 </dev/urandom >dir/file0
    head -c 67108864 </dev/urandom >dir/file1
    head -c 67108864 </dev/urandom >dir/file2
    head -c 67108864 </dev/urandom >dir/file3
    head -c 67108864 </dev/urandom >dir/file4
    head -c 67108864 </dev/urandom >dir/file5
    head -c 67108864 </dev/urandom >dir/file6
    head -c 67108864 </dev/urandom >dir/file7
    head -c 67108864 </dev/urandom >dir/file8
    head -c 67108864 </dev/urandom >dir/file9
}

function clean() {
    rm -rf dir tar
}

function rp() {
    mkdir tar
    time target/release/rp -r dir tar --mode=$1 || true
    ls tar/dir
    rm -rf tar
}

init

rp seq
rp fut
rp fut2

clean
