extern crate sysinfo;

use std::thread;
use std::time;

use sysinfo::SystemExt;
use sysinfo::ProcessorExt;

fn get_cpu_use(req_sys: &sysinfo::System) -> f32
{
    let mut cpus: Vec<f32> = Vec::new();
    for core in req_sys.get_processors() { cpus.push(core.get_cpu_usage()); }

    let cpu_tot: f32 = cpus.iter().sum();
    let cpu_avg: f32 = cpu_tot / cpus.len() as f32;

    cpu_avg
}

fn get_ram_use(req_sys: &sysinfo::System) -> f32
{
    (req_sys.get_used_memory() as f32) / (req_sys.get_total_memory() as f32) * 100.
}

fn get_swp_use(req_sys: &sysinfo::System) -> f32
{
    (req_sys.get_used_swap() as f32) / (req_sys.get_total_swap() as f32) * 100.
}

fn main()
{
    println!("Starting dwm-status... ");
    
    let mut current_sys = sysinfo::System::new_all();
    current_sys.refresh_all();

    let cpu_avg = get_cpu_use(&current_sys);
    let ram_prcnt = get_ram_use(&current_sys);
    let swp_prcnt = get_swp_use(&current_sys);

    println! ("CPU: {:.1}%\tRAM: {:.1}%\tSWP: {:.2}%", cpu_avg, ram_prcnt, swp_prcnt);
}
