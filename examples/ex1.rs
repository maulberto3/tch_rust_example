use tch::{Device, Tensor};

fn main() {
    // Check if CUDA is available
    if tch::Cuda::is_available() {
        println!("CUDA is available!");
    } else {
        println!("CUDA is not available.");
    }

    // Define a tensor on the CPU
    let cpu_tensor = Tensor::from_slice(&[1.0, 2.0, 3.0, 4.0]).to(Device::Cpu);
    println!("CPU Tensor: {:?}", cpu_tensor);

    // Perform an operation on the CPU tensor
    let cpu_result = &cpu_tensor + 1.0;
    println!("Result on CPU: {:?}", cpu_result);

    // Define a tensor on the GPU (if available)
    if tch::Cuda::is_available() {
        let gpu_tensor = Tensor::from_slice(&[1.0, 2.0, 3.0, 4.0]).to(Device::Cuda(0));
        println!("GPU Tensor: {:?}", gpu_tensor);

        // Perform an operation on the GPU tensor
        let gpu_result = &gpu_tensor + 1.0;
        println!("Result on GPU: {:?}", gpu_result);
    } else {
        println!("Skipping GPU operations since CUDA is not available.");
    }
}
