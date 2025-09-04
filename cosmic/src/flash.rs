//! a
use cosmic::cosmic_theme::palette::num::PartialCmp;
use std::collections::HashMap;
use std::os::fd::FromRawFd;
use std::sync::atomic::Ordering;
use std::sync::Arc;

type UDisksOptions = HashMap<&'static str, dbus::arg::Variant<Box<dyn dbus::arg::RefArg>>>;

/// Wrapper type to impl Clone trait, shouldn't ever panic.
#[derive(Debug)]
pub struct File {
    ///
    pub file: std::fs::File,
}
impl Clone for File {
    fn clone(&self) -> Self {
        File { file: self.file.try_clone().unwrap() }
    }
}

///
#[derive(Debug)]
pub struct Flash {
    image: Option<std::fs::File>,
    progress: Arc<Vec<atomic::Atomic<u64>>>,
    finished: Arc<Vec<atomic::Atomic<bool>>>,
}
impl Flash {
    ///
    pub fn new(
        image: std::fs::File,
        progress: Arc<Vec<atomic::Atomic<u64>>>,
        finished: Arc<Vec<atomic::Atomic<bool>>>,
    ) -> Flash {
        Flash { image: Some(image), progress, finished }
    }
    ///
    pub fn write(&mut self, drives: Vec<std::fs::File>) -> popsicle::Task<Progress<'_>> {
        let mut task = popsicle::Task::new(self.image.take().unwrap().into(), false);
        for (i, file) in drives.into_iter().enumerate() {
            let progress = Progress { id: i, flash: self, errors: vec![] };
            task.subscribe(file.into(), (), progress);
        }
        task
    }
}

///
#[derive(Debug)]
pub struct Progress<'a> {
    id: usize,
    flash: &'a Flash,
    errors: Vec<Result<(), crate::Error>>,
}
impl<'a> popsicle::Progress for Progress<'a> {
    type Device = ();

    fn message(&mut self, _: &Self::Device, kind: &str, message: &str) {
        self.errors[self.id] = Err(crate::Error::new_popsicle(kind, message))
    }

    fn finish(&mut self) {
        self.flash.finished[self.id].store(true, Ordering::SeqCst);
    }

    fn set(&mut self, value: u64) {
        self.flash.progress[self.id].store(value, Ordering::SeqCst);
    }
}
///
pub fn is_windows_iso(file: &std::fs::File) -> bool {
    if let Ok(fs) = iso9660::ISO9660::new(file) {
        return fs.publisher_identifier() == "MICROSOFT CORPORATION";
    }
    false
}
///
pub fn refresh_devices(image_size: Option<u64>) -> Option<Box<[Arc<dbus_udisks2::DiskDevice>]>> {
    let udisks = dbus_udisks2::UDisks2::new().unwrap();
    let devices = dbus_udisks2::Disks::new(&udisks).devices;
    let mut devices = devices
        .into_iter()
        .filter(|dick| dick.drive.connection_bus.eq("usb") || dick.drive.connection_bus.eq("sdio"))
        .filter(|d| d.parent.size.neq(&0))
        .filter(|d| d.parent.size >= image_size.unwrap_or(0))
        .map(Arc::new)
        .collect::<Vec<_>>()
        .into_boxed_slice();
    devices.sort_by_key(|d| d.drive.id.clone());
    Some(devices)
}
///
pub fn udisks_unmount(dbus_path: &str) -> Result<(), ()> {
    let connection = dbus::blocking::Connection::new_system().unwrap();

    let dbus_path = ::dbus::strings::Path::new(dbus_path).unwrap();

    let proxy = dbus::blocking::Proxy::new(
        "org.freedesktop.UDisks2",
        dbus_path,
        std::time::Duration::new(25, 0),
        &connection,
    );

    let mut options = UDisksOptions::new();
    options.insert("force", dbus::arg::Variant(Box::new(true)));
    let res: Result<(), _> =
        proxy.method_call("org.freedesktop.UDisks2.Filesystem", "Unmount", (options,));

    if let Err(err) = res {
        if err.name() != Some("org.freedesktop.UDisks2.Error.NotMounted") {
            return Err(());
        }
    }

    Ok(())
}
///
pub fn udisks_open(dbus_path: &str) -> Result<std::fs::File, ()> {
    let connection = dbus::blocking::Connection::new_system().unwrap();

    let dbus_path = ::dbus::strings::Path::new(dbus_path).unwrap();

    let proxy = dbus::blocking::Proxy::new(
        "org.freedesktop.UDisks2",
        &dbus_path,
        std::time::Duration::new(25, 0),
        &connection,
    );
    let mut options = UDisksOptions::new();
    options.insert("flags", dbus::arg::Variant(Box::new(libc::O_SYNC)));
    let res: (dbus::arg::OwnedFd,) =
        proxy.method_call("org.freedesktop.UDisks2.Block", "OpenDevice", ("rw", options)).unwrap();

    Ok(unsafe { std::fs::File::from_raw_fd(res.0.into_fd()) })
}
