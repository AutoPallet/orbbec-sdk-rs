use orbbec_sdk::Context;

fn main() {
    let context = Context::new().unwrap();
    let devices = context.query_device_list().unwrap();

    for (i, device) in devices.into_iter().enumerate() {
        let device = device.unwrap();
        let device_info = device.info().unwrap();

        print!("Device {i}: ");
        println!("{device_info:#?}\n");
    }
}
