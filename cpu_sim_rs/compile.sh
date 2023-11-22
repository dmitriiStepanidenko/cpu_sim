#!/bin/sh
wasm-pack build --out-dir ./../src/lib/pkg --target web --release
wasm-opt -Oz ./../src/lib/pkg/cpu_sim_rs_bg.wasm -o ./../src/lib/pkg/cpu_sim_rs_bg.wasm
