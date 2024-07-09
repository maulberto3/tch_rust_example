## Download libtorch linux cuda12.1 cxx11 ABI at pytorch.org

https://pytorch.org/

# Then unzip it in a safe location

unzip libtorch-cxx11-abi-shared-with-deps-2.3.1+cu121 -d libtorch

# Run the cargo build with two env vars like this

# Examples
LIBTORCH=~/libtorch/libtorch/ LIBTORCH_BYPASS_VERSION_CHECK=true LD_LIBRARY_PATH=~/libtorch/libtorch/lib:$LD_LIBRARY_PATH cargo run --example ex1

LIBTORCH=~/libtorch/libtorch/ LIBTORCH_BYPASS_VERSION_CHECK=true LD_LIBRARY_PATH=~/libtorch/libtorch/lib:$LD_LIBRARY_PATH cargo run --example ex2 -- cpu
LIBTORCH=~/libtorch/libtorch/ LIBTORCH_BYPASS_VERSION_CHECK=true LD_LIBRARY_PATH=~/libtorch/libtorch/lib:$LD_LIBRARY_PATH cargo run --example ex2 -- cpu-rayon
LIBTORCH=~/libtorch/libtorch/ LIBTORCH_BYPASS_VERSION_CHECK=true LD_LIBRARY_PATH=~/libtorch/libtorch/lib:$LD_LIBRARY_PATH cargo run --example ex2 -- gpu
LIBTORCH=~/libtorch/libtorch/ LIBTORCH_BYPASS_VERSION_CHECK=true LD_LIBRARY_PATH=~/libtorch/libtorch/lib:$LD_LIBRARY_PATH cargo run --example ex2 -- gpu-rayon

# Also needs htop

sudo apt  install htop

# For best experience, use simulataneously

watch -n 1 nvidi-smi
htop

