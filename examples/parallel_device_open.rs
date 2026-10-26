use std::sync::{Arc, Barrier};
use std::time::{Duration, Instant};

use anyhow::{Context as _, anyhow, ensure};
use clap::Parser;
use orbbec_sdk::{
    Context, LogSeverity,
    device::{Device, DeviceList},
    logger::Logger,
};

#[derive(Parser)]
#[command(about = "Open Orbbec devices concurrently without starting streams")]
struct Args {
    /// Open all enumerated devices.
    #[arg(long, conflicts_with_all = ["index", "serial"])]
    all: bool,

    /// Open the device at this index.
    #[arg(long, conflicts_with_all = ["all", "serial"])]
    index: Option<usize>,

    /// Open the enumerated devices with these serial numbers. Repeat for multiple devices.
    #[arg(long, conflicts_with_all = ["all", "index"])]
    serial: Vec<String>,

    /// Number of concurrent opens for the selected index.
    #[arg(long, default_value_t = 1, requires = "index")]
    copies: usize,

    /// Repeat the concurrent open, closing every device between rounds.
    #[arg(long, default_value_t = 1)]
    iterations: usize,
}

struct OpenedDevice {
    index: usize,
    elapsed: Duration,
    device: Device,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    ensure!(
        args.all || args.index.is_some() || !args.serial.is_empty(),
        "select one of --all, --index or --serial"
    );
    ensure!(args.copies > 0, "--copies must be greater than zero");
    ensure!(
        args.iterations > 0,
        "--iterations must be greater than zero"
    );

    Logger::set_directory(LogSeverity::Off, None)?;
    Logger::set_console(LogSeverity::Info)?;

    let context = Context::new()?;
    let devices = context.query_device_list()?;
    let indices = if args.all {
        ensure!(!devices.is_empty(), "no devices found");
        (0..devices.len()).collect::<Vec<_>>()
    } else if !args.serial.is_empty() {
        let mut indices = Vec::with_capacity(args.serial.len());
        for serial in &args.serial {
            let index = (0..devices.len())
                .find(|&index| {
                    devices
                        .serial_number(index)
                        .is_ok_and(|found| found == *serial)
                })
                .with_context(|| format!("no enumerated device has serial {serial}"))?;
            indices.push(index);
        }
        indices
    } else {
        let index = args.index.expect("validated above");
        ensure!(
            index < devices.len(),
            "device index {index} is outside the enumerated count {}",
            devices.len()
        );
        vec![index; args.copies]
    };

    for iteration in 0..args.iterations {
        if args.iterations > 1 {
            println!("iteration {iteration}");
        }
        open_concurrently(&devices, &indices)?;
    }

    Ok(())
}

fn open_concurrently(devices: &DeviceList<'_>, indices: &[usize]) -> anyhow::Result<()> {
    let start_barrier = Arc::new(Barrier::new(indices.len()));
    let wall_started = Instant::now();
    let opened = std::thread::scope(|scope| {
        let handles = indices
            .iter()
            .copied()
            .enumerate()
            .map(|(worker, index)| {
                let start_barrier = Arc::clone(&start_barrier);
                scope.spawn(move || -> anyhow::Result<OpenedDevice> {
                    start_barrier.wait();
                    let started = Instant::now();
                    let device = devices.get(index).with_context(|| {
                        format!("worker {worker} failed to open device index {index}")
                    })?;
                    Ok(OpenedDevice {
                        index,
                        elapsed: started.elapsed(),
                        device,
                    })
                })
            })
            .collect::<Vec<_>>();

        handles
            .into_iter()
            .enumerate()
            .map(|(worker, handle)| {
                handle
                    .join()
                    .map_err(|_| anyhow!("worker {worker} panicked"))?
            })
            .collect::<anyhow::Result<Vec<_>>>()
    })?;
    let wall_elapsed = wall_started.elapsed();

    for (worker, opened) in opened.iter().enumerate() {
        let info = opened.device.info()?;
        println!(
            "worker {worker}: index={} UID={} serial={} elapsed={:.3}s",
            opened.index,
            info.uid(),
            info.serial_number(),
            opened.elapsed.as_secs_f64()
        );
    }
    println!("total wall time: {:.3}s", wall_elapsed.as_secs_f64());

    Ok(())
}
