//! Capture complex samples synchronously without an async runtime.
use rtlsdr_nusb::{Complex32, Device, MaybeFuture};
use std::time::Duration;

fn main() -> rtlsdr_nusb::Result<()> {
    for device in Device::list().wait()? {
        println!("{device:?}");
    }
    let mut device = Device::builder().frequency_hz(100_000_000).open().wait()?;
    println!("{:?}", device.info());
    println!(
        "rate={} Hz frequency={} Hz",
        device.actual_sample_rate_hz(),
        device.actual_frequency_hz()
    );
    let mut rx = device.rx_stream()?;
    rx.start().wait()?;
    let mut samples = [Complex32::default(); 4096];
    let count = rx.read(&mut samples, Some(Duration::from_secs(2))).wait()?;
    println!("{count} samples; first={:?}", samples[..count].first());
    println!("{:?}", rx.close().wait()?);
    device.shutdown().wait()
}
