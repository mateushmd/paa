#!/usr/bin/env bash

if [ $# -ne 1 ]; then
    echo "Error: expected exactly one parameter"
    exit 1
fi

printf "pub mod %s;\n" "$1" >> src/problems/mod.rs

printf "pub use problems::%s;\n" "$1" >> src/lib.rs

printf "use crate::input;\n\npub fn solve() {\n\n}\n" > "src/problems/$1.rs"
