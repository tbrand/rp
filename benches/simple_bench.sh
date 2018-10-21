#!/bin/bash

set -eux

cargo build --release

function init() {
    rm -rf dir && mkdir dir
}

function finalize() {
    rm -rf dir
}

function bench_rp() {
    init
    time target/release/rp -r -m $1 target dir || true
}

function bench_cp() {
    init
    time cp -r target dir || true
}

bench_rp seq
bench_rp fut
bench_rp fut2
bench_cp

finalize
