//! Device enumeration and selection without opening hardware.
use crate::{Error, Result};
use nusb::MaybeFuture;

/// USB identity collected during enumeration.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeviceDescriptor {
    /// Index among currently visible devices with the selected USB IDs.
    pub index: usize,
    /// USB vendor ID.
    pub vid: u16,
    /// USB product ID.
    pub pid: u16,
    /// Device serial string, when the OS/browser supplies it.
    pub serial: Option<String>,
    /// Manufacturer string, when available.
    pub manufacturer: Option<String>,
    /// Product string, when available.
    pub product: Option<String>,
}
impl DeviceDescriptor {
    pub(crate) fn from_nusb(index: usize, info: &nusb::DeviceInfo) -> Self {
        Self {
            index,
            vid: info.vendor_id(),
            pid: info.product_id(),
            serial: info.serial_number().map(str::to_owned),
            manufacturer: info.manufacturer_string().map(str::to_owned),
            product: info.product_string().map(str::to_owned),
        }
    }
    pub(crate) fn blog_v4(&self) -> bool {
        self.manufacturer.as_deref() == Some("RTLSDRBlog")
            && self.product.as_deref() == Some("Blog V4")
    }
}

#[derive(Clone, Debug, Default)]
pub(crate) struct Selector {
    pub index: Option<usize>,
    pub serial: Option<String>,
    pub usb_id: Option<(u16, u16)>,
}
impl Selector {
    pub(crate) fn matches_id(&self, vid: u16, pid: u16) -> bool {
        self.usb_id
            .map_or(vid == 0x0bda && matches!(pid, 0x2832 | 0x2838), |pair| {
                pair == (vid, pid)
            })
    }
    fn matches(&self, index: usize, info: &nusb::DeviceInfo) -> bool {
        self.index.is_none_or(|i| i == index)
            && self
                .serial
                .as_deref()
                .is_none_or(|s| info.serial_number() == Some(s))
    }
}
pub(crate) fn list() -> impl MaybeFuture<Output = Result<Vec<DeviceDescriptor>>> {
    nusb::list_devices().map(|devices| {
        Ok(devices?
            .filter(|d| Selector::default().matches_id(d.vendor_id(), d.product_id()))
            .enumerate()
            .map(|(i, d)| DeviceDescriptor::from_nusb(i, &d))
            .collect())
    })
}
pub(crate) fn select(
    selector: Selector,
) -> impl MaybeFuture<Output = Result<(DeviceDescriptor, nusb::DeviceInfo)>> {
    nusb::list_devices().map(move |devices| {
        devices?
            .filter(|d| selector.matches_id(d.vendor_id(), d.product_id()))
            .enumerate()
            .find(|(i, d)| selector.matches(*i, d))
            .map(|(i, d)| (DeviceDescriptor::from_nusb(i, &d), d))
            .ok_or(Error::DeviceNotFound)
    })
}
#[cfg(target_arch = "wasm32")]
pub(crate) async fn request_permission(selector: &Selector) -> Result<()> {
    let pairs = selector
        .usb_id
        .map_or_else(|| vec![(0x0bda, 0x2832), (0x0bda, 0x2838)], |p| vec![p]);
    let selectors: Vec<_> = pairs
        .into_iter()
        .map(|(vid, pid)| {
            let mut filter = nusb::DeviceSelector::all().with_vid_pid(vid, pid);
            if let Some(serial) = &selector.serial {
                filter = filter.with_serial_number(serial.clone());
            }
            filter
        })
        .collect();
    nusb::request_device(&selectors)
        .await?
        .ok_or(Error::DeviceNotFound)?;
    Ok(())
}
