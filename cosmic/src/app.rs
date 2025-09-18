//! a
use crate::fl;
use cosmic::ApplicationExt;
use futures::SinkExt;
use futures::StreamExt;
use std::collections::HashMap;
use std::fmt::Debug;
use std::ops::Deref;
use std::path::PathBuf;
use std::sync::atomic::Ordering;
use std::sync::{Arc, Mutex};

const HASH_SELECTIONS: [&str; 6] = ["None", "SHA512", "SHA256", "SHA1", "MD5", "BLAKE2b"];
/// The app model the holds context, state, and core logic.
pub struct App {
    core: cosmic::Core,
    state: AppState,
    context: AppContext,
}
impl Debug for App {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("App")
            .field("core", &String::from("cosmic::Core"))
            .field("state", &self.state)
            .field("context", &self.context)
            .finish()
    }
}
impl cosmic::Application for App {
    type Flags = ();
    type Message = Message;
    type Executor = cosmic::executor::Default;
    const APP_ID: &'static str = "com.system76.Popsicle";
    fn subscription(&self) -> cosmic::iced::Subscription<Self::Message> {
        match self.context {
            AppContext::SelectDrives { .. } => {
                cosmic::iced::time::every(cosmic::iced::time::Duration::from_secs(3))
                    .map(|_| Message::RefreshDevices)
            }
            AppContext::Progress => cosmic::iced::Subscription::run(test).map(|_| Message::Done),
            _ => cosmic::iced::Subscription::none(),
        }
    }
    fn header_start(&self) -> Vec<cosmic::Element<'_, Self::Message>> {
        let mut header = Vec::new();
        match self.context {
            AppContext::ChooseAnImg { .. } => {
                let mut next_button = cosmic::widget::button::suggested(fl!("next"));
                header.push(
                    cosmic::widget::button::suggested(fl!("cancel"))
                        .on_press(Message::Close)
                        .into(),
                );
                if self.state.image.is_some() {
                    next_button =
                        next_button.on_press(Message::SwitchContext(AppContext::SelectDrives {
                            drive_selected: false,
                        }));
                }
                header.push(next_button.into());
                header
            }
            AppContext::SelectDrives { drive_selected } => {
                let mut next_button = cosmic::widget::button::suggested(fl!("next"));
                header.push(
                    cosmic::widget::button::suggested("Back")
                        .on_press(Message::SwitchContext(AppContext::ChooseAnImg {
                            generating_checksum: false,
                            identcal_hash: None,
                        }))
                        .into(),
                );
                if drive_selected {
                    next_button = next_button.on_press(Message::StartFlash);
                }
                header.push(next_button.into());
                header
            }
            AppContext::Progress => {
                header.push(
                    cosmic::widget::button::suggested(fl!("cancel"))
                        .on_press(Message::SwitchContext(AppContext::ChooseAnImg {
                            generating_checksum: false,
                            identcal_hash: None,
                        }))
                        .into(),
                );
                header
            }
            AppContext::Success => {
                header.push(
                    cosmic::widget::button::suggested(fl!("flash-again"))
                        .on_press(Message::SwitchContext(AppContext::ChooseAnImg {
                            generating_checksum: false,
                            identcal_hash: None,
                        }))
                        .into(),
                );
                header.push(
                    cosmic::widget::button::suggested(fl!("done")).on_press(Message::Close).into(),
                );
                header
            }
        }
    }
    fn update(&mut self, message: Self::Message) -> cosmic::app::Task<Self::Message> {
        match message {
            Message::Close => {
                return cosmic::Task::done(cosmic::Action::Cosmic(cosmic::app::Action::Close));
            }
            Message::SwitchContext(context) => {
                self.context = context;
                match context {
                    AppContext::ChooseAnImg { .. } => {}
                    AppContext::SelectDrives { .. } => {
                        return cosmic::app::Task::done(cosmic::Action::App(
                            Message::RefreshDevices,
                        ));
                    }
                    AppContext::Progress => {
                        return cosmic::app::Task::done(cosmic::Action::App(Message::Flash));
                    }

                    AppContext::Success => {}
                }
            }
            Message::OpenFile => {
                return cosmic::task::future(async move {
                    let filter = cosmic::dialog::file_chooser::FileFilter::new("Iso/Images")
                        .glob("*.iso")
                        .glob("*.img");
                    let dialog = cosmic::dialog::file_chooser::open::Dialog::new()
                        .title("Open")
                        .filter(filter);
                    Message::SelectedFile(dialog.open_file().await.unwrap().url().to_owned())
                });
            }
            Message::SelectedFile(url) => {
                if let Ok(file) = std::fs::File::open(url.path()) {
                    let image_size = file.metadata().ok().map_or(0, |m| m.len());
                    let image_name = std::path::Path::new(url.path())
                        .file_name()
                        .map(|name| name.to_string_lossy().to_string());
                    self.state.image_name = image_name;
                    self.state.image = Some(PathBuf::from(url.path()));
                    self.state.image_size = Some(image_size);
                }
            }
            Message::Input(text) => {
                self.state.hash_input = text;
            }
            Message::HashSelected(selection) => {
                self.state.selected_hash = Some(selection);
            }
            Message::CheckHash => {
                let (hash, path) =
                    (self.state.selected_hash.unwrap(), self.state.image.clone().unwrap());
                if let Some(hash) = self.state.hashed.get(&(path.clone(), hash)) {
                    self.state.hash = Some(hash.clone());
                    return cosmic::task::message(Message::Done);
                }
                if let AppContext::ChooseAnImg { identcal_hash, .. } = self.context {
                    self.context =
                        AppContext::ChooseAnImg { generating_checksum: true, identcal_hash }
                }
                return cosmic::task::future(async move {
                    Message::GeneratedHash(crate::hash::generate_hash(hash, path))
                });
            }
            Message::GeneratedHash(hash) => {
                self.state.hash = hash.clone();
                self.state.hashed.insert(
                    (self.state.image.as_ref().unwrap().clone(), self.state.selected_hash.unwrap()),
                    hash.unwrap(),
                );
                if self.state.hash.is_some() {
                    match self.state.hash.as_ref().unwrap() == &self.state.hash_input {
                        true => {
                            if let AppContext::ChooseAnImg { generating_checksum, .. } =
                                self.context
                            {
                                self.context = AppContext::ChooseAnImg {
                                    generating_checksum,
                                    identcal_hash: Some(true),
                                }
                            }
                        }
                        false => {
                            if let AppContext::ChooseAnImg { generating_checksum, .. } =
                                self.context
                            {
                                self.context = AppContext::ChooseAnImg {
                                    generating_checksum,
                                    identcal_hash: Some(false),
                                }
                            }
                        }
                    }
                }
                if let AppContext::ChooseAnImg { identcal_hash, .. } = self.context {
                    self.context =
                        AppContext::ChooseAnImg { generating_checksum: false, identcal_hash }
                }
            }
            Message::SelectedAllDrives => {
                self.state.all_drives = !self.state.all_drives;
                self.state.drives.values_mut().map(|b| *b = self.state.all_drives).for_each(drop);
                self.context = AppContext::SelectDrives {
                    drive_selected: !self.state.drives.values().all(|f| f.eq(&false)),
                };
            }
            Message::RefreshDevices => {
                if let Some(devices) = crate::flash::refresh_devices(self.state.image_size) {
                    let device_paths = devices
                        .iter()
                        .map(|d| {
                            (
                                {
                                    if d.drive.vendor.is_empty() {
                                        format!("{}", d.drive.model,)
                                    } else {
                                        format!("{} {}", d.drive.vendor, d.drive.model,)
                                    }
                                },
                                d.parent.preferred_device.display().to_string(),
                                bytesize::ByteSize::b(d.parent.size).display().iec().to_string(),
                            )
                        })
                        .collect::<Vec<_>>();
                    if self.state.drives_paths.as_ref().is_none_or(|f| f != &device_paths) {
                        self.state.drives_paths = Some(device_paths);
                        self.state.available_devices = Some(devices);
                        self.state.drives = self
                            .state
                            .available_devices
                            .as_ref()
                            .unwrap()
                            .iter()
                            .enumerate()
                            .map(|(i, _)| (i, false))
                            .collect();
                    }
                }
            }
            Message::SelectDrive(i) => {
                if let Some(b) = self.state.drives.get_mut(&i) {
                    *b = !b.deref()
                };
                self.context = AppContext::SelectDrives {
                    drive_selected: !self.state.drives.values().all(|f| f.eq(&false)),
                };
            }
            Message::StartFlash => {
                self.state.drives_selected = Some(
                    self.state
                        .drives
                        .iter()
                        .filter_map(|(i, b)| {
                            if *b {
                                Some(self.state.available_devices.as_ref().unwrap()[*i].clone())
                            } else {
                                None
                            }
                        })
                        .collect(),
                );
                let n = self.state.drives_selected.as_ref().unwrap().len();
                self.state.flash_finished =
                    Arc::new((0..n).map(|_| atomic::Atomic::new(false)).collect::<_>());
                self.state.flash_progress =
                    Arc::new((0..n).map(|_| atomic::Atomic::new(0)).collect::<_>());
                self.state.previous = Arc::new(Mutex::new((0..n).map(|_| [0; 7]).collect::<_>()));

                return cosmic::app::Task::done(cosmic::Action::App(Message::SwitchContext(
                    AppContext::Progress,
                )));
            }
            Message::Flash => {
                let mut flash = crate::flash::Flash::new(
                    std::fs::File::open(self.state.image.as_ref().unwrap()).unwrap(),
                    self.state.flash_progress.clone(),
                    self.state.flash_finished.clone(),
                );
                let drives = self
                    .state
                    .drives_selected
                    .as_ref()
                    .unwrap()
                    .iter()
                    .map(|p| {
                        let _ = crate::flash::udisks_unmount(&p.parent.path);
                        for partition in &p.partitions {
                            let _ = crate::flash::udisks_unmount(&partition.path);
                        }
                        crate::flash::udisks_open(&p.parent.path).unwrap()
                    })
                    .collect();
                return cosmic::task::future(async move {
                    let task = flash.write(drives);
                    let mut buf = [0u8; 64 * 1024];
                    match futures::executor::block_on(task.process(&mut buf)) {
                        Ok(_) => {
                            tracing::info!("Flash completed");
                        }
                        Err(e) => {
                            tracing::error!("{}", e)
                        }
                    }
                    Message::Done
                });
            }
            Message::Flashing(mut sender) => {
                futures::executor::block_on(sender.send(Event::Flash(
                    crate::flash::Flash::new(
                        std::fs::File::open(self.state.image.as_ref().unwrap()).unwrap(),
                        self.state.flash_progress.clone(),
                        self.state.flash_finished.clone(),
                    ),
                    self.state.drives_selected.as_ref().unwrap().clone(),
                )));
            }
            Message::Done => {
                tracing::info!("Future returned");
            }
            Message::Failed => {}
        }
        cosmic::app::Task::none()
    }
    fn core(&self) -> &cosmic::Core {
        &self.core
    }
    fn core_mut(&mut self) -> &mut cosmic::Core {
        &mut self.core
    }
    fn init(core: cosmic::Core, _flags: Self::Flags) -> (Self, cosmic::app::Task<Self::Message>) {
        let mut nav_model = cosmic::widget::nav_bar::Model::default();
        nav_model.activate_position(0);
        let mut app = App {
            core,
            state: AppState { selected_hash: Some(0), ..Default::default() },
            context: AppContext::default(),
        };
        let msg = app.set_window_title(String::from("Popsicle"));
        app.set_header_title("Popsicle".to_owned());
        (app, msg)
    }
    fn view(&self) -> cosmic::Element<'_, Self::Message> {
        let view = match self.context {
            AppContext::ChooseAnImg { generating_checksum, identcal_hash } => {
                self.choose_an_img_view(generating_checksum, identcal_hash)
            }
            AppContext::SelectDrives { .. } => self.select_drives_view(),
            AppContext::Progress => self.progress_view(),
            AppContext::Success => self.success_view(),
        };
        cosmic::Element::new(
            cosmic::widget::container(view)
                .height(cosmic::iced::Length::Fill)
                .width(cosmic::iced::Length::Fill)
                .align_x(cosmic::iced::Alignment::Center)
                .align_y(cosmic::iced::Alignment::Center),
        )
    }
}
impl App {
    fn choose_an_img_view(
        &self,
        generating_checksum: bool,
        identcal_hash: Option<bool>,
    ) -> cosmic::Element<'_, Message> {
        let image_icon = cosmic::widget::Image::new(cosmic::widget::image::Handle::from_path(
            "assets/application-x-cd-image.png",
        ))
        .width(cosmic::iced::Length::Fixed(50.));
        let image_title = cosmic::widget::text::title4(fl!("image-view-title")); // should be bold
        let image_description = cosmic::widget::text::body(fl!("image-view-description"));
        let image_top = cosmic::widget::column()
            .push(image_title)
            .push(image_description)
            .align_x(cosmic::iced::Left)
            .width(cosmic::iced::Length::Fill)
            .height(cosmic::iced::Length::Fill);

        let gen_chksum = cosmic::widget::column()
            .push(cosmic::widget::text("Spinner goes here"))
            .push(cosmic::widget::text::heading(fl!("generating-checksum")))
            .align_x(cosmic::iced::Center);
        let (image_name, image_size) =
            if let (Some(name), Some(size)) = (&self.state.image_name, &self.state.image_size) {
                (
                    cosmic::widget::text::heading(name),
                    cosmic::widget::text::body(bytesize::ByteSize::b(*size).to_string()),
                )
            } else {
                (
                    cosmic::widget::text::heading(fl!("no-image-selected")),
                    cosmic::widget::text::body(""),
                )
            }; //should be bold or when empty Equal "No image selected";
        let mut image_center = cosmic::widget::column();
        let image_file_open_button = cosmic::widget::button::suggested(fl!("choose-image-button"))
            .on_press(Message::OpenFile);
        match generating_checksum {
            true => image_center = image_center.push(gen_chksum),
            false => {
                image_center =
                    image_center.push(image_file_open_button).push(image_name).push(image_size)
            }
        }
        image_center = image_center
            .align_x(cosmic::iced::Alignment::Center)
            .width(cosmic::iced::Length::Fill)
            .height(cosmic::iced::Length::Fill);
        let hash_dropdown = cosmic::widget::dropdown::dropdown(
            &HASH_SELECTIONS,
            self.state.selected_hash,
            Message::HashSelected,
        );
        let mut hash_text_input = cosmic::widget::text_input("", &self.state.hash_input);
        let mut hash_check_button = cosmic::widget::button::suggested(fl!("check-label"));
        if self.state.image.is_some() {
            hash_check_button = hash_check_button.on_press(Message::CheckHash);
            hash_text_input = hash_text_input.on_input(Message::Input).style(match identcal_hash {
                Some(true) => crate::style::green_style(),
                Some(false) => crate::style::red_style(),
                None => cosmic::theme::TextInput::Default,
            });
        }
        let image_bottom = cosmic::widget::settings::section().add(cosmic::Element::from(
            cosmic::widget::settings::item::flex_item_row(vec![cosmic::Element::new(
                //"Hash:",
                cosmic::widget::row()
                    .push(cosmic::widget::text::body(fl!("hash-label")))
                    .push(hash_dropdown)
                    .push(hash_text_input)
                    .push(hash_check_button)
                    .align_y(cosmic::iced::Alignment::Center),
            )])
            .width(cosmic::iced::Length::Fill),
        ));
        let row = cosmic::widget::row()
            .push(image_icon)
            .push(
                cosmic::widget::container(
                    cosmic::widget::column()
                        .push(image_top)
                        .push(image_center)
                        .push(image_bottom)
                        .height(cosmic::iced::Length::Fill)
                        .width(cosmic::iced::Length::Fill),
                )
                .height(cosmic::iced::Length::Fill)
                .width(cosmic::iced::Length::Fill),
            )
            .height(cosmic::iced::Length::Fill)
            .width(cosmic::iced::Length::Fill);
        row.into()
    }
    fn select_drives_view(&self) -> cosmic::Element<'_, Message> {
        let drive_icon = cosmic::widget::Image::new(cosmic::widget::image::Handle::from_path(
            "assets/drive-removable-media-usb.png",
        ))
        .width(cosmic::iced::Length::Fixed(50.));
        let drive_label = cosmic::widget::text::title4(fl!("devices-view-title")); // should be bold
        let drive_description = cosmic::widget::text::body(fl!("devices-view-description"));

        let mut drive_buf = vec![];
        if let Some(drives) = self.state.drives_paths.as_ref() {
            for (i, (name, path, size)) in drives.iter().enumerate() {
                let select = cosmic::widget::mouse_area(
                    cosmic::widget::container(
                        cosmic::widget::row()
                            .push(
                                cosmic::widget::checkbox(
                                    "",
                                    *self.state.drives.get(&i).unwrap_or(&false),
                                )
                                .on_toggle(move |_| Message::SelectDrive(i)),
                            )
                            .push(
                                cosmic::widget::column()
                                    .push(cosmic::widget::text::heading(format!(
                                        "{} ({})",
                                        name, path
                                    )))
                                    .push(cosmic::widget::text(size.clone())),
                            )
                            .width(cosmic::iced::Length::Fill)
                            .align_y(cosmic::iced::Alignment::Center),
                    )
                    .width(cosmic::iced::Length::Fill),
                )
                .on_press(Message::SelectDrive(i));

                drive_buf.push(cosmic::Element::new(select));
            }
        }
        let drive_select = cosmic::widget::settings::section::section().extend(drive_buf);
        let drive_column = cosmic::widget::column()
            .push(drive_label)
            .push(drive_description)
            .push(
                cosmic::widget::mouse_area(
                    cosmic::widget::container(
                        cosmic::widget::checkbox(fl!("select-all"), self.state.all_drives)
                            .on_toggle(|_| Message::SelectedAllDrives),
                    )
                    .width(cosmic::iced::Length::Fill),
                )
                .on_press(Message::SelectedAllDrives),
            )
            .push(
                cosmic::widget::scrollable(drive_select)
                    .width(cosmic::iced::Length::Fill)
                    .height(cosmic::iced::Length::Fill),
            );
        let drive_row = cosmic::widget::row()
            .push(drive_icon)
            .push(drive_column)
            .height(cosmic::iced::Length::Fill)
            .width(cosmic::iced::Length::Fill);
        drive_row.into()
    }
    fn progress_view(&self) -> cosmic::Element<'_, Message> {
        let drive_icon = cosmic::widget::Image::new(cosmic::widget::image::Handle::from_path(
            "assets/drive-removable-media-usb.png",
        ))
        .width(cosmic::iced::Length::Fixed(50.));
        let flash_label = cosmic::widget::text::title4(fl!("flash-view-title")); // should be bold
        let flash_description = cosmic::widget::text::body(fl!("flash-view-description"));

        let mut flash_buf = vec![];
        let mut prev = self.state.previous.lock().unwrap();
        if let Some(drives) = self.state.drives_selected.as_ref() {
            for (i, drive) in drives.iter().enumerate() {
                let progress_label = match self.state.flash_finished[i].load(Ordering::SeqCst) {
                    true => cosmic::widget::text::body("Complete"),
                    false => cosmic::widget::text::body(format!(
                        "{}/s",
                        bytesize::ByteSize::b({
                            prev[i][1] = prev[i][2];
                            prev[i][2] = prev[i][3];
                            prev[i][3] = prev[i][4];
                            prev[i][4] = prev[i][5];
                            prev[i][5] = prev[i][6];
                            prev[i][6] =
                                self.state.flash_progress[i].load(Ordering::SeqCst) - prev[i][0];
                            prev[i][0] = self.state.flash_progress[i].load(Ordering::SeqCst);
                            prev[i].iter().skip(1).sum::<u64>() / 3
                        })
                        .to_string()
                    )),
                };
                let progress_name = cosmic::widget::text::heading(format!(
                    "{} ({})",
                    self.state.drives_paths.as_ref().unwrap()[i].0.clone(),
                    self.state.drives_paths.as_ref().unwrap()[i].1.clone()
                ));
                let progress_column = cosmic::widget::column()
                    .push(
                        cosmic::iced::widget::progress_bar(0.0..=100., {
                            let f = self.state.flash_progress[i].load(Ordering::SeqCst) as f32
                                / self.state.image_size.unwrap() as f32;
                            f * 100.0
                        })
                        .height(cosmic::iced::Length::Fixed(2.0)),
                    )
                    .push(progress_label)
                    .align_x(cosmic::iced::Alignment::Center);
                let row = cosmic::widget::row()
                    .push(progress_name)
                    .push(progress_column)
                    .align_y(cosmic::iced::Alignment::Center);
                flash_buf.push(cosmic::Element::new(row));
            }
        }
        let flash_column = cosmic::widget::column()
            .push(flash_label)
            .push(flash_description)
            .height(cosmic::iced::Length::Fill)
            .width(cosmic::iced::Length::Fill)
            .extend(flash_buf);
        let flash_row = cosmic::widget::row().push(drive_icon).push(flash_column);
        flash_row.into()
    }
    fn success_view(&self) -> cosmic::Element<'_, Message> {
        let complete_icon = cosmic::widget::icon(cosmic::widget::icon::from_path(PathBuf::from(
            "assets/process-completed-symbolic.svg",
        )))
        .size(50);
        let complete_label = cosmic::widget::text::title4(fl!("flashing-completed")); // should be bold
        let complete_description = cosmic::widget::text::body(format!(
            "{} devices successfully flashed",
            self.state.drives_selected.as_ref().unwrap().len()
        ));
        let complete_row = cosmic::widget::row()
            .push(complete_icon)
            .push(cosmic::widget::column().push(complete_label).push(complete_description))
            .height(cosmic::iced::Length::Fill)
            .width(cosmic::iced::Length::Fill);
        complete_row.into()
    }
}
/// Messages emitted by the app.
#[derive(Debug, Clone)]
pub enum Message {
    /// Emitted when closing the app.
    Close,
    /// Emitted to switch between contexts.
    SwitchContext(AppContext),
    /// Emitted when the button is pressed.
    OpenFile,
    /// Emitted when typed into a text input.
    Input(String),
    /// Emitted when choosing a hashing algorithm.
    HashSelected(usize),
    /// Emitted when
    GeneratedHash(Option<String>),
    /// Emitted when the check box is toggled.
    SelectedAllDrives,
    /// Emitted when the drive check box is toggled.
    SelectDrive(usize),
    /// Emitted when
    SelectedFile(url::Url),
    /// Emitted when
    RefreshDevices,
    /// Emitted when the button is pressed.
    CheckHash,
    ///
    Flash,
    ///
    Flashing(cosmic::iced::futures::channel::mpsc::Sender<Event>),
    ///
    StartFlash,
    ///
    Done,
    ///
    Failed,
}
/// The context of the app which will be displayed.
#[derive(Clone, Copy, Debug)]
pub enum AppContext {
    /// a
    ChooseAnImg {
        /// a
        generating_checksum: bool,
        /// a
        identcal_hash: Option<bool>,
    },
    /// a
    SelectDrives {
        /// a
        drive_selected: bool,
    },
    /// a
    Progress,
    /// a
    Success,
}
impl Default for AppContext {
    fn default() -> Self {
        AppContext::ChooseAnImg { generating_checksum: false, identcal_hash: None }
    }
}

/// The app state the store all data that drives logic.
#[derive(Debug, Default)]
pub struct AppState {
    // needed image selection, label for name, size
    image: Option<PathBuf>,
    image_name: Option<String>,
    image_size: Option<u64>,
    // needed for hash input string
    hash_input: String,
    hash: Option<String>,
    selected_hash: Option<usize>,
    all_drives: bool,
    hashed: HashMap<(PathBuf, usize), String>,
    // list of devices paths and sizes
    drives_paths: Option<Vec<(String, String, String)>>,
    drives: HashMap<usize, bool>,
    //drives choosen to be flashed
    drives_selected: Option<Vec<Arc<dbus_udisks2::DiskDevice>>>, //Option<Vec<String>>,
    test: Arc<atomic::Atomic<bool>>,
    //
    previous: Arc<Mutex<Vec<[u64; 7]>>>,
    flash_progress: Arc<Vec<atomic::Atomic<u64>>>,
    flash_finished: Arc<Vec<atomic::Atomic<bool>>>,
    available_devices: Option<Box<[Arc<dbus_udisks2::DiskDevice>]>>,
}

enum Event {
    Flash(crate::flash::Flash, Vec<Arc<dbus_udisks2::DiskDevice>>),
    T1(Arc<atomic::Atomic<bool>>),
    T2(Arc<atomic::Atomic<bool>>),
}
fn test() -> impl cosmic::iced::futures::Stream<Item = Message> {
    cosmic::iced::stream::channel(100, |mut output| async move {
        let (sender, mut receiver) = cosmic::iced::futures::channel::mpsc::channel(100);
        output.send(Message::Flashing(sender)).await;
        loop {
            let input = receiver.select_next_some().await;
            match input {
                Event::T1(b) => {
                    b.store(true, Ordering::SeqCst);
                }
                Event::T2(b) => {
                    b.store(false, Ordering::SeqCst);
                }
                _ => {}
            }
        }
    })
}
fn flad() -> impl cosmic::iced::futures::Stream<Item = Message> {
    cosmic::iced::stream::channel(100, |mut output| async move {
        let (sender, mut receiver) = cosmic::iced::futures::channel::mpsc::channel(100);
        output.send(Message::Flashing(sender)).await;
        loop {
            let input = receiver.select_next_some().await;
            match input {
                Event::Flash(mut f, d) => {
                    let drives = d
                        .iter()
                        .map(|p| {
                            let _ = crate::flash::udisks_unmount(&p.parent.path);
                            for partition in &p.partitions {
                                let _ = crate::flash::udisks_unmount(&partition.path);
                            }
                            crate::flash::udisks_open(&p.parent.path).unwrap()
                        })
                        .collect();
                    let task = f.write(drives);
                    let mut buf = [0u8; 64 * 1024];
                    match futures::executor::block_on(task.process(&mut buf)) {
                        Ok(_) => {
                            tracing::info!("Flash completed");
                            output.send(Message::Done).await;
                        }
                        Err(e) => {
                            tracing::error!("{}", e);
                            output.send(Message::Failed).await;
                        }
                    }
                }
            }
        }
    })
}
