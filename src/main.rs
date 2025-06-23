use clap::Parser;
use sysinfo::Networks;
use std::{thread, time};

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Number of refreshes between information updates
    #[arg(short, long, default_value_t = 1)]
    nrefresh: u32,

    /// Interval between refreshes (ms)
    #[arg(short, long, default_value_t = 1000)]
    refreshinterval: u64,

    /// Combine icons
    #[arg(short, long)]
    combineicons: bool,

    /// Enable animated icons
    #[arg(short, long)]
    animatedicons: bool,

    /// Animated icons speed (a parameter in ax+blog10(x) formula used) (for high speeds)
    #[arg(long, default_value_t = 0.0000000000000001)]
    speed_multiplier_lin: f64,

    /// Animated icons speed (b parameter in ax+blog10(x) formula used) (for low speeds)
    #[arg(long, default_value_t = 0.0000000001)]
    speed_multiplier_log10: f64,

    /// Animated icon list (down order) ⡺⢵⣫⢝
    #[arg(long, default_value_t = String::from("▁▃▄▅▆▇█▇▆▅▄▃▁"))]
    animatediconlist: String,

    /// Size of bins (for the dynamic icons) in bytes
    #[arg(short, long, default_value_t = 2048)]
    binsize: u128,

    /// Static up icon
    #[arg(short, long, default_value_t = String::from("↑"))]
    upicon: String,

    /// Static down icon
    #[arg(short, long, default_value_t = String::from("↓"))]
    downicon: String,
}

fn display_speed_unit(speed_in_bytes_per_second: u64) -> String{
    let speed_in_bits_per_second = speed_in_bytes_per_second*8;
    if speed_in_bits_per_second < 1000{ // b/s
        return format!("{} b/s",speed_in_bits_per_second);
    }
    if 1000 <= speed_in_bits_per_second && speed_in_bits_per_second < 1000_000{ // kb/s
        return format!("{:.1} kb/s",speed_in_bits_per_second as f64 / 1000.);
    }
    if 1000_000 <= speed_in_bits_per_second && speed_in_bits_per_second < 1000_000_000 { // Mb/s
        return format!("{:.1} Mb/s",speed_in_bits_per_second as f64 / 1000_000.);
    }
    if 1000_000_000 <= speed_in_bits_per_second { // Gb/s
        return format!("{:.1} Gb/s",speed_in_bits_per_second as f64 / 1000_000_000.);
    }
    return String::from("error")
}

fn main() {
    let args = Args::parse();
    let mut networks = Networks::new_with_refreshed_list();
    let mut lastinfoupdatetime = time::Instant::now();
    let mut speed_up: u64 = 0;
    let mut speed_down: u64 = 0;
    let mut refresh_counter = args.nrefresh-1;
    let mut up_anim_counter: f64 = 0.0;
    let mut down_anim_counter: f64 = 0.0;
    let mut lastframetime = time::Instant::now();
    let mut lastdelay = time::Duration::from_millis(args.refreshinterval);
    // animated icons (in down order) args.animatediconlist
    let icons: Vec<char> = args.animatediconlist.chars().collect();
    loop {
        if refresh_counter % args.nrefresh == 0{
            let mut sum_up: u64 = 0;
            let mut sum_down: u64 = 0;
            networks.refresh(true);
            for (_, network) in &networks {
                sum_down += network.received();
                sum_up += network.transmitted();
            }
            if lastinfoupdatetime.elapsed().as_millis() != 0{
                speed_up = sum_up * 1000/(lastinfoupdatetime.elapsed().as_millis() as u64);
                speed_down = sum_down * 1000/(lastinfoupdatetime.elapsed().as_millis() as u64);
            }
            lastinfoupdatetime = time::Instant::now();
            refresh_counter = 0 // prevent overflow
        }
        let mut disp_up_icon = args.upicon.clone();
        let mut disp_down_icon = args.downicon.clone();
        let mut up_add_amount: f64 = 1.;
        let mut down_add_amount: f64 = 1.;
        if args.animatedicons{
            disp_up_icon = String::from(icons[icons.len()-1-(up_anim_counter.floor() as usize % icons.len())].clone());
            disp_down_icon = String::from(icons[down_anim_counter.floor() as usize % icons.len()].clone());
            if args.combineicons{
            disp_up_icon = format!("{}{}",args.upicon.clone(),disp_up_icon);
            disp_down_icon = format!("{}{}",args.downicon.clone(),disp_down_icon);
            }
            up_add_amount = (args.speed_multiplier_lin*(speed_up as f64)+args.speed_multiplier_log10*((speed_up+1) as f64).log10())*(lastframetime.elapsed().as_nanos() as f64);
            down_add_amount = (args.speed_multiplier_lin*(speed_down as f64)+args.speed_multiplier_log10*((speed_down+1) as f64).log10())*(lastframetime.elapsed().as_nanos() as f64);
            up_anim_counter = (up_anim_counter+up_add_amount) % icons.len() as f64;
            down_anim_counter =  (down_anim_counter+down_add_amount) % icons.len() as f64;
        }
        println!("{} {:>10}  {} {:>10}",disp_up_icon,display_speed_unit(speed_up),disp_down_icon,display_speed_unit(speed_down).to_string());
        lastframetime = time::Instant::now();
        if up_add_amount > 1.0 || down_add_amount > 1.0 {
            lastdelay = time::Duration::from_nanos((lastdelay.as_nanos() as f64/up_add_amount.max(down_add_amount)) as u64);
        }
        if up_add_amount < 0.1 && down_add_amount < 0.1 {
            lastdelay = time::Duration::from_millis(args.refreshinterval);
        }
        thread::sleep(lastdelay);
        refresh_counter+=1;
    }
}
