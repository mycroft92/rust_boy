#!/bin/bash

export GB_ROOT=$PWD
cd codegen
cargo run
cp src/inst_parser.rs ../src/