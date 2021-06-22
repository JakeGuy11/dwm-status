extern crate sysinfo;

use std::thread;
use std::time;

use sysinfo::SystemExt;
use sysinfo::ProcessorExt;
use sysinfo::NetworkExt;
use sysinfo::NetworksExt;

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


fn get_ntwk_dwn(req_sys: &sysinfo::System) -> i32
{
    let mut rcv_tot: Vec<i32> = Vec::new();
    for (interface_name, ntwk) in req_sys.get_networks() { rcv_tot.push(ntwk.get_received() as i32); }

    let ntwk_tot: i32 = rcv_tot.iter().sum();
    let ntwk_processed = (ntwk_tot / 128) as i32;

    ntwk_processed
}

fn get_ntwk_up(req_sys: &sysinfo::System) -> i32
{
    let mut snd_tot: Vec<i32> = Vec::new();
    for (interface_name, ntwk) in req_sys.get_networks() { snd_tot.push(ntwk.get_transmitted() as i32); }

    let ntwk_tot: i32 = snd_tot.iter().sum();
    let ntwk_processed = (ntwk_tot / 128) as i32;

    ntwk_processed
}


fn main()
{
    println!("Starting dwm-status... ");
    
    let mut current_sys = sysinfo::System::new_all();
    
    loop
    {
        current_sys.refresh_all();

        let cpu_avg = get_cpu_use(&current_sys);
        let ram_prcnt = get_ram_use(&current_sys);
        let swp_prcnt = get_swp_use(&current_sys);
        let ntwk_dwn = get_ntwk_dwn(&current_sys);
        let ntwk_up = get_ntwk_up(&current_sys);

        let msg = format! ("CPU: {:.1}%\tRAM: {:.1}%\tSWP: {:.2}%\tDownload: {}Kbps\tUpload: {}Kbps", cpu_avg, ram_prcnt, swp_prcnt, ntwk_dwn, ntwk_up);

        std::process::Command::new("xsetroot").arg("-name").arg(msg).spawn().expect("`xsetroot` has failed!");

        thread::sleep(time::Duration::from_millis(1000));
    }
}
