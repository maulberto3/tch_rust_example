use std::time::Instant;

/*A stress test that hammers the CPU and GPU using PyTorch*/
use clap::Parser;
use rayon::prelude::*;
use tch::Tensor;

//build a cpu load test function
pub fn cpu_load_test() {
    let slice = vec![0; 1_000_000];
    for i in 1..1_000 {
        let t = Tensor::from_slice(&slice).to_device(tch::Device::Cpu);
        println!("{} {:?}", i, t.size())
    }
}


//build a gpu load test function that uses threads via rayon iterator that sends the load to the GPU
pub fn cpu_load_test_rayon() {
    let slice = vec![0; 1_000_000];
    (1..1_000).into_par_iter().for_each(|i| {
        let t = Tensor::from_slice(&slice).to_device(tch::Device::Cpu);
        println!("{} {:?}", i, t.size())
    });
}

//build a gpu load test function
pub fn gpu_load_test() {
    let slice = vec![0; 1_000_000];
    for i in 1..1_000 {
        let t = Tensor::from_slice(&slice).to_device(tch::Device::Cuda(0));
        println!("{} {:?}", i, t.size())
    }
}

//build a gpu load test function that uses threads via rayon iterator that sends the load to the GPU
pub fn gpu_load_test_rayon() {
    let slice = vec![0; 1_000_000];
    (1..1_000).into_par_iter().for_each(|i| {
        let t = Tensor::from_slice(&slice).to_device(tch::Device::Cuda(0));
        println!("{} {:?}", i, t.size())
    });
}

#[derive(Parser)]
#[clap(
    version = "1.0",
    author = "Noah Gift",
    about = "A Stress Test for PyTorch CPU and GPU.  There are three subcommands: cpu, gpu, and tgpu. The tgpu subcommand uses Rayon to send the load to the GPU."
)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    #[clap(version = "1.0", author = "Noah Gift")]
    Cpu {},
    CpuRayon {},
    Gpu {},
    GpuRayon {},
}

//build the main function and import stress:: namespace
fn main() {
    let args = Cli::parse();
    let start = Instant::now();
    match args.command {
        Some(Commands::Cpu {}) => {
            println!("Running CPU Stress Test");
            cpu_load_test();
        }
        Some(Commands::CpuRayon {}) => {
            println!("Running GPU Stress Test");
            cpu_load_test_rayon();
        }
        Some(Commands::Gpu {}) => {
            println!("Running GPU Stress Test with Rayon");
            gpu_load_test();
        }
        Some(Commands::GpuRayon {}) => {
            println!("Running GPU Stress Test with Rayon");
            gpu_load_test_rayon();
        }
        
        None => {
            println!("Please specify a subcommand");
        }
    }
    println!("Duration {:+.4?}", start.elapsed())
}