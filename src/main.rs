use clap::Parser;
use sysinfo::Networks;
use std::{thread, time};

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Number of refreshes between information updates
    #[arg(short, long, default_value_t = 10)]
    nrefresh: u32,

    /// Interval between refreshes (ms)
    #[arg(short, long, default_value_t = 100)]
    refreshinterval: u64,

    /// Combine icons
    #[arg(short, long)]
    combineicons: bool,

    /// Enable animated icons
    #[arg(short, long)]
    animatedicons: bool,

    /// Animated icons speed (a parameter in ax+blog10(x) formula used)
    #[arg(long, default_value_t = 0.0000001)]
    speed_multiplier_lin: f64,

    /// Animated icons speed (b parameter in ax+blog10(x) formula used)
    #[arg(long, default_value_t = 0.01)]
    speed_multiplier_log10: f64,

    /// Animated icon 0 (down order)
    #[arg(long, default_value_t = String::from("⡺"))]
    animatedicon_0: String,

    /// Animated icon 1 (down order)
    #[arg(long, default_value_t = String::from("⢵"))]
    animatedicon_1: String,

    /// Animated icon 2 (down order)
    #[arg(long, default_value_t = String::from("⣫"))]
    animatedicon_2: String,

    /// Animated icon 3 (down order)
    #[arg(long, default_value_t = String::from("⢝"))]
    animatedicon_3: String,

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

    // animated icons (in down order)
    let icons = vec![args.animatedicon_0,args.animatedicon_1,args.animatedicon_2,args.animatedicon_3];
    loop {
        if refresh_counter % args.nrefresh == 0{
            let mut sum_up: u64 = 0;
            let mut sum_down: u64 = 0;
            networks.refresh(true);
            for (_, network) in &networks {
                sum_down += network.received();
                sum_up += network.transmitted();
            }
            speed_up = sum_up * 1000/(lastinfoupdatetime.elapsed().as_millis() as u64);
            speed_down = sum_down * 1000/(lastinfoupdatetime.elapsed().as_millis() as u64);
            lastinfoupdatetime = time::Instant::now();
            refresh_counter = 0 // prevent overflow
        }
        let mut disp_up_icon = args.upicon.clone();
        let mut disp_down_icon = args.downicon.clone();
        if args.animatedicons{
            disp_up_icon = icons[icons.len()-1-(up_anim_counter.floor() as usize % icons.len())].clone();
            disp_down_icon = icons[down_anim_counter.floor() as usize % icons.len()].clone();
            if args.combineicons{
            disp_up_icon = format!("{}{}",args.upicon.clone(),disp_up_icon);
            disp_down_icon = format!("{}{}",args.downicon.clone(),disp_down_icon);
            }

            up_anim_counter = (up_anim_counter+args.speed_multiplier_lin*(speed_up as f64)+args.speed_multiplier_log10*((speed_up+1) as f64).log10()) % icons.len() as f64;
            down_anim_counter =  (down_anim_counter+args.speed_multiplier_lin*(speed_down as f64)+args.speed_multiplier_log10*((speed_down+1) as f64).log10()) % icons.len() as f64;
        }

        println!("{} {:>10}  {} {:>10}",disp_up_icon,display_speed_unit(speed_up),disp_down_icon,display_speed_unit(speed_down).to_string());
        thread::sleep(time::Duration::from_millis(args.refreshinterval));
        refresh_counter+=1;
    }
}
