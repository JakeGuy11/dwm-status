extern crate sysinfo;

use std::thread;
use std::time;

use sysinfo::SystemExt;
use sysinfo::ProcessorExt;
use sysinfo::NetworkExt;
use sysinfo::ComponentExt;

// Get the average core usage
fn get_cpu_use(req_sys: &sysinfo::System) -> f32
{
    // Put all of the core loads into a vector
    let mut cpus: Vec<f32> = Vec::new();
    for core in req_sys.get_processors() { cpus.push(core.get_cpu_usage()); }

    // Get the average load
    let cpu_tot: f32 = cpus.iter().sum();
    let cpu_avg: f32 = cpu_tot / cpus.len() as f32;

    cpu_avg
}

// Divide the used RAM by the total RAM
fn get_ram_use(req_sys: &sysinfo::System) -> f32
{
    (req_sys.get_used_memory() as f32) / (req_sys.get_total_memory() as f32) * 100.
}

// Divide the used swap by the total swap
fn get_swp_use(req_sys: &sysinfo::System) -> f32
{
    (req_sys.get_used_swap() as f32) / (req_sys.get_total_swap() as f32) * 100.
}

// Get the total network (down) usage
fn get_ntwk_dwn(req_sys: &sysinfo::System) -> i32
{
    // Get the total bytes recieved by every network interface
    let mut rcv_tot: Vec<i32> = Vec::new();
    for (_interface_name, ntwk) in req_sys.get_networks() { rcv_tot.push(ntwk.get_received() as i32); }

    // Total them and convert the bytes to KB
    let ntwk_tot: i32 = rcv_tot.iter().sum();
    let ntwk_processed = (ntwk_tot / 128) as i32;

    ntwk_processed
}

// Get the total network (up) usage
fn get_ntwk_up(req_sys: &sysinfo::System) -> i32
{
    // Get the total bytes sent by every network interface
    let mut snd_tot: Vec<i32> = Vec::new();
    for (_interface_name, ntwk) in req_sys.get_networks() { snd_tot.push(ntwk.get_transmitted() as i32); }

    // Total them and convert the bytes to KB
    let ntwk_tot: i32 = snd_tot.iter().sum();
    let ntwk_processed = (ntwk_tot / 128) as i32;

    ntwk_processed
}

// Get the temperature of the CPU
fn get_temp(req_sys: &sysinfo::System) -> i32
{
    // For every component, if it's the CPU, put its temperature in variable to return
    let mut wanted_temp: f32 = -1.;
    for comp in req_sys.get_components() { if comp.get_label() == "CPU" { wanted_temp = comp.get_temperature(); } }
    
    wanted_temp as i32
}

// Main function
fn main()
{
    // Define a system that we will check
    let mut current_sys = sysinfo::System::new_all();
    
    loop
    {
        // Refresh the system
        current_sys.refresh_all();

        // Call each function to get all the values we need
        let cpu_avg = get_cpu_use(&current_sys);
        let ram_prcnt = get_ram_use(&current_sys);
        let _swp_prcnt = get_swp_use(&current_sys); // We're not using swap for now because it's almost always 0, but we'll add it as a cli arg in the future
        let ntwk_dwn = get_ntwk_dwn(&current_sys);
        let ntwk_up = get_ntwk_up(&current_sys);
        let temp = get_temp(&current_sys);

        // Format the message to go on the bar
        // TODO: format the entries properly so the size doesn't change
        let msg = format! ("CPU: {:.1}%\t\tTemp: {}°C\t\tRAM: {:.1}%\t\tDownload: {}Kbps\t\tUpload: {}Kbps", cpu_avg, temp, ram_prcnt, ntwk_dwn, ntwk_up);

        // Execute `xsetroot` with the message
        std::process::Command::new("xsetroot").arg("-name").arg(msg).spawn().expect("`xsetroot` has failed!");

        // Wait one second
        thread::sleep(time::Duration::from_millis(1000));
    }
}
