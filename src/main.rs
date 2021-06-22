extern crate sysinfo;

use std::thread;
use std::time;

use sysinfo::SystemExt;
use sysinfo::ProcessorExt;
use sysinfo::NetworkExt;
use sysinfo::ComponentExt;

// Add whitespace prepending a value
fn add_whitespace (str_to_format: String, chars_tot: u32) -> String
{
    // Get the length of the passed string and calculate how many spaces to add
    let char_num = str_to_format.as_bytes().len() as u32;
    let space_num = chars_tot - char_num;

    // Create a new string to add everything to
    let mut ret_string = String::new();

    // Add all the needed spaces to that string
    for _i in 0..space_num { ret_string.push(' '); }

    // Add the original string to it
    ret_string.push_str(&str_to_format);

    ret_string
}

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
        let swp_prcnt = get_swp_use(&current_sys);
        let ntwk_dwn = get_ntwk_dwn(&current_sys);
        let ntwk_up = get_ntwk_up(&current_sys);
        let temp = get_temp(&current_sys);

        let prnt_cpu = add_whitespace(format! ("{:.1}",cpu_avg), 5);
        let prnt_ram = add_whitespace(format! ("{:.1}",ram_prcnt), 5);
        let _prnt_swp = add_whitespace(format! ("{:.1}",swp_prcnt), 5); // We're not using swap for now because it's almost always 0, but we'll add it as a cli arg in the future
        let prnt_dwn = add_whitespace(format! ("{:.1}",ntwk_dwn), 5);
        let prnt_up = add_whitespace(format! ("{:.1}",ntwk_up), 5);
        let prnt_tmp = add_whitespace(format! ("{:.1}",temp), 3);

        // Format the message to go on the bar
        let msg = format! ("CPU: {}%|Temp: {}°C|RAM: {}%|Download: {}Kbps|Upload: {}Kbps", prnt_cpu, prnt_tmp, prnt_ram, prnt_dwn, prnt_up);

        // Execute `xsetroot` with the message
        std::process::Command::new("xsetroot").arg("-name").arg(msg).spawn().expect("`xsetroot` has failed!");

        // Wait one second
        thread::sleep(time::Duration::from_millis(1000));
    }
}
