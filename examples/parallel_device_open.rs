use std::sync::{Arc, Barrier};
use std::time::{Duration, Instant};

use anyhow::{Context as _, anyhow, ensure};
use clap::Parser;
use orbbec_sdk::{Context, LogSeverity, device::Device, logger::Logger};

#[derive(Parser)]
#[command(about = "Open Orbbec devices concurrently without starting streams")]
struct Args {
    /// Open all enumerated devices.
    #[arg(long, conflicts_with = "index")]
    all: bool,

    /// Open the device at this index.
    #[arg(long, conflicts_with = "all")]
    index: Option<usize>,

    /// Number of concurrent opens for the selected index.
    #[arg(long, default_value_t = 1, requires = "index")]
    copies: usize,
}

struct OpenedDevice {
    index: usize,
    elapsed: Duration,
    device: Device,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    ensure!(
        args.all || args.index.is_some(),
        "select either --all or --index"
    );
    ensure!(args.copies > 0, "--copies must be greater than zero");

    Logger::set_directory(LogSeverity::Off, None)?;
    Logger::set_console(LogSeverity::Info)?;

    let context = Context::new()?;
    let devices = context.query_device_list()?;
    let indices = if args.all {
        ensure!(!devices.is_empty(), "no devices found");
        (0..devices.len()).collect::<Vec<_>>()
    } else {
        let index = args.index.expect("validated above");
        ensure!(
            index < devices.len(),
            "device index {index} is outside the enumerated count {}",
            devices.len()
        );
        vec![index; args.copies]
    };

    let start_barrier = Arc::new(Barrier::new(indices.len()));
    let wall_started = Instant::now();
    let opened = std::thread::scope(|scope| {
        let handles = indices
            .into_iter()
            .enumerate()
            .map(|(worker, index)| {
                let start_barrier = Arc::clone(&start_barrier);
                let devices = &devices;
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
