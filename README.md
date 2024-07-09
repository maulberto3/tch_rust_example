# Pytorch through Rust (tch) example

This example shows how to use Pytorch through Rust by leveraging the `tch` crate:
    - This repo shows two examples
    - Example 1: Tests tch with and without cuda
    - Example 2: Tests tch with and without rayon and cuda
        - This example should show that rayon is good for parallelizing data and processors effectiveness

## Setup

### Download libtorch linux cuda12.1 cxx11 ABI at pytorch.org

Be sure use the same pytorch version as the one tch needs
If it's diferent, use the LIBTORCH_BYPASS_VERSION_CHECK env var

https://pytorch.org/

### Then unzip it in a safe location

`unzip libtorch-cxx11-abi-shared-with-deps-2.3.1+cu121 -d libtorch`

## Run the examples

### Example 1
`LIBTORCH=~/libtorch/libtorch/ LIBTORCH_BYPASS_VERSION_CHECK=true LD_LIBRARY_PATH=~/libtorch/libtorch/lib:$LD_LIBRARY_PATH cargo run --example ex1`

### Example 1
First, hit `htop` to see real time core processor usage. If not available, install with `sudo apt install htop`

Then, hit `watch -n 2 nvidia-smi` to watch for cuda usage as well

So, hit them in different processes/terminals. Then, 

`LIBTORCH=~/libtorch/libtorch/ LIBTORCH_BYPASS_VERSION_CHECK=true LD_LIBRARY_PATH=~/libtorch/libtorch/lib:$LD_LIBRARY_PATH cargo run --example ex2 -- cpu`

`LIBTORCH=~/libtorch/libtorch/ LIBTORCH_BYPASS_VERSION_CHECK=true LD_LIBRARY_PATH=~/libtorch/libtorch/lib:$LD_LIBRARY_PATH cargo run --example ex2 -- cpu-rayon`

`LIBTORCH=~/libtorch/libtorch/ LIBTORCH_BYPASS_VERSION_CHECK=true LD_LIBRARY_PATH=~/libtorch/libtorch/lib:$LD_LIBRARY_PATH cargo run --example ex2 -- gpu`

`LIBTORCH=~/libtorch/libtorch/ LIBTORCH_BYPASS_VERSION_CHECK=true LD_LIBRARY_PATH=~/libtorch/libtorch/lib:$LD_LIBRARY_PATH cargo run --example ex2 -- gpu-rayon`