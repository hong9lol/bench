#!/bin/bash

function display_help() {
    echo "Usage: $0 [option] (global_variables)"
    echo
    echo "  -h | help      help message"
    echo
    exit 1
}

if [ $# -eq 0 ]; then
    echo "Error: No arguments provided."
    display_help
fi

cd c
gcc -o main main.c ./src/$1.c
RESULT_C=`./main`
cd ../rust
# if [ $1 == "global_struct" || $1 == "global_variable" ]; then
cargo build --no-default-features --features $1_arc_rwlock --release --quiet
RESULT_RUST_ARC=`./target/release/rust`
cargo build --no-default-features --features $1_refcell --release --quiet
RESULT_RUST_REF=`./target/release/rust`
cargo build --no-default-features --features $1_atomic --release --quiet
RESULT_RUST_ATOMIC=`./target/release/rust`
# else
#     echo "Error: No arguments provided."
#     display_help
# fi
cd ../
echo
echo " [C BASE]  : $RESULT_C"
echo " [R_ARC]   : $RESULT_RUST_ARC"
echo " [R_REF]   : $RESULT_RUST_REF"
echo " [R_ATOMIC]: $RESULT_RUST_ATOMIC"

c=$(echo "$RESULT_C" | sed 's/ms//') 
r_arc=$(echo "$RESULT_RUST_ARC" | sed 's/ms//')
r_ref=$(echo "$RESULT_RUST_REF" | sed 's/ms//')
r_atomic=$(echo "$RESULT_RUST_ATOMIC" | sed 's/ms//')
ratio_arc=$(echo "scale=2; $r_arc / $c" | bc)  
ratio_ref=$(echo "scale=2; $r_ref / $c" | bc)  
ratio_atomic=$(echo "scale=2; $r_atomic / $c" | bc)  

# 결과 출력
echo " [Ratio ARC]   : $ratio_arc"
echo " [Ratio REF]   : $ratio_ref"
echo " [Ratio ATOMIC]: $ratio_atomic"
#taskset -c 1 ./c/main
#taskset -c 1 ./rust/target/release/rust
