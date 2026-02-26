use orbbec_sdk::{Context, LogSeverity, logger::Logger};

fn main() -> anyhow::Result<()> {
    // Don't create a ./Log directory in the current working directory
    Logger::set_directory(LogSeverity::Off, None)?;
    Logger::set_console(LogSeverity::Info)?;

    let context = Context::new()?;
    let devices = context.query_device_list()?;

    for (i, device) in devices.into_iter().enumerate() {
        let device = device?;
        let device_info = device.info()?;

        print!("Device {i}: ");
        println!("{device_info:#?}\n");
    }

    Ok(())
}
