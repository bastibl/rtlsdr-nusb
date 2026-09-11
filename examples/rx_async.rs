//! Receive using the smol integration; run with `--features smol`.
use rtlsdr_nusb::{Complex32, Device};

fn main() -> rtlsdr_nusb::Result<()> {
    futures_lite::future::block_on(run())
}
async fn run() -> rtlsdr_nusb::Result<()> {
    let mut device = Device::builder().frequency_hz(100_000_000).open().await?;
    println!("{:?}", device.info());
    let mut rx = device.rx_stream()?;
    rx.start().await?;
    let mut samples = [Complex32::default(); 4096];
    let count = rx.read(&mut samples, None).await?;
    println!("{count} samples; first={:?}", samples[..count].first());
    rx.stop().await?;
    device.set_frequency_hz(101_000_000).await?;
    rx.start().await?;
    println!(
        "{} samples after restart",
        rx.read(&mut samples, None).await?
    );
    println!("{:?}", rx.close().await?);
    device.shutdown().await
}
