use cosmic::dialog::file_chooser::open::Dialog;
use cosmic::dialog::file_chooser::FileFilter;
use cosmic::iced::Alignment;
use cosmic::iced::Length;
use cosmic::task;
use cosmic::Action;
use cosmic::ApplicationExt;
use cosmic::Core;
use cosmic::Element;
use cosmic::Task;
use iso9660::ISO9660;
use url::Url;

const HASH_SELECTIONS: [&str; 5] = ["None", "SHA256", "SHA1", "MD5", "BLAKE2b"];
fn is_windows_iso(file: &std::fs::File) -> bool {
    if let Ok(fs) = ISO9660::new(file) {
        return fs.publisher_identifier() == "MICROSOFT CORPORATION";
    }
    false
}

#[derive(Debug, Clone)]
pub enum Message {
    None,
    HashSelected(usize),
    OpenFile,
    SelectedFile(Url),
    SelectedFileSize(String),
    CheckHash,
    Edit(bool),
    Input(String),
    Surface(cosmic::surface::Action),
}

pub struct App {
    core: Core,
    selected_hash: Option<usize>,
    selected_image: Option<String>,
    selected_image_size: Option<String>,
    hash_input: String,
    editing: bool,
    search_id: cosmic::widget::Id,
}

impl cosmic::Application for App {
    type Flags = ();
    type Message = Message;
    type Executor = cosmic::executor::Default;
    const APP_ID: &'static str = "com.system76.Popsicle";

    fn header_start(&self) -> Vec<Element<Self::Message>> {
        vec![cosmic::widget::button::suggested("Cancel").on_press(Message::CheckHash).into()]
    }

    fn header_end(&self) -> Vec<Element<Self::Message>> {
        vec![cosmic::widget::button::suggested("Next").on_press(Message::CheckHash).into()]
    }
    fn update(&mut self, message: Self::Message) -> Task<Action<Self::Message>> {
        match message {
            Message::None => (),
            Message::HashSelected(selection) => self.selected_hash = Some(selection),
            Message::OpenFile => {
                return cosmic::task::future(async move {
                    let filter = FileFilter::new("Iso/Images").glob("*.iso").glob("*.img");
                    let dialog = Dialog::new().title("Open").filter(filter);
                    match dialog.open_file().await {
                        Ok(o) => Message::SelectedFile(o.url().to_owned()),
                        Err(_) => Message::None,
                    }
                })
            }
            Message::SelectedFile(o) => {
                if let Ok(file) = std::fs::File::open(o.path()) {
                    let image_size = file.metadata().ok().map_or(0, |m| m.len());
                    let image_name = if let Some(name) = std::path::Path::new(o.path()).file_name()
                    {
                        Some(name.to_string_lossy().to_string())
                    } else {
                        None
                    };
                    let warning = if is_windows_iso(&file) { "" } else { "" };

                    self.selected_image = image_name;
                    self.selected_image_size = Some(bytesize::ByteSize::b(image_size).to_string());
                }
            }
            Message::CheckHash => (),
            Message::Edit(bool) => (),
            Message::Input(string) => (),
            Message::Surface(surface) => {}
            Message::SelectedFileSize(o) => {}
        }
        task::none()
    }
    fn core(&self) -> &Core {
        &self.core
    }
    fn core_mut(&mut self) -> &mut Core {
        &mut self.core
    }
    fn init(core: Core, flags: Self::Flags) -> (Self, Task<Action<Self::Message>>) {
        let mut nav_model = cosmic::widget::nav_bar::Model::default();
        nav_model.activate_position(0);
        let mut app = App {
            core,
            selected_hash: Some(0),
            selected_image: None,
            selected_image_size: None,
            hash_input: String::new(),
            editing: false,
            search_id: cosmic::widget::Id::unique(),
        };
        app.set_header_title("Popsicle".to_owned());

        (app, Task::none())
    }

    fn view(&self) -> Element<Self::Message> {
        let image_icon = cosmic::widget::Image::new(cosmic::widget::image::Handle::from_path(
            "assets/application-x-cd-image.png",
        ))
        .width(cosmic::iced::Length::Fixed(50.));
        let image_label = cosmic::widget::text::title4("Choose an Image"); // should be bold
        let image_description = cosmic::widget::text::body("Select the .iso or .img that you want to flash. You can also plug your USB drives in now.");
        let image_name = if self.selected_image.is_none() {
            cosmic::widget::text::heading("No image selected")
        } else {
            cosmic::widget::text::heading(self.selected_image.as_ref().unwrap())
        }; //should be bold or when empty Equal "No image selected"
        let image_size = if self.selected_image_size.is_none() {
            cosmic::widget::text::body("")
        } else {
            cosmic::widget::text::body(self.selected_image_size.as_ref().unwrap())
        };
        let image_file_open_button =
            cosmic::widget::button::suggested("Choose Image").on_press(Message::OpenFile);

        //let hash_dropdown = cosmic::widget::dropdown::dropdown(
        //    &HASH_SELECTIONS,
        //    self.selected_hash,
        //    Message::HashSelected,
        //);
        let hash_text_input = cosmic::widget::text_input::TextInput::new("", &self.hash_input)
            .on_input(Message::Input);
        let hash_check_button =
            cosmic::widget::button::suggested("Check").on_press(Message::CheckHash);
        let hash_row = cosmic::widget::row()
            .push(
                cosmic::widget::settings::section().add(cosmic::Element::from(
                    cosmic::widget::settings::item::builder("Hash:").control(
                        cosmic::widget::row()
                            .push(cosmic::widget::dropdown::popup_dropdown(
                                &HASH_SELECTIONS,
                                self.selected_hash,
                                Message::HashSelected,
                                cosmic::iced::window::Id::RESERVED,
                                Message::Surface,
                                |a| {},
                            ))
                            .push(hash_text_input)
                            .push(hash_check_button),
                    ),
                )),
                //hash_check_button
            )
            .align_y(Alignment::Center)
            .width(Length::Fill)
            .height(Length::Fill);
        //let hash_row = row![hash_label, hash_dropdown, hash_text_input, hash_check_button]
        //    .align_y(iced::Alignment::Center)
        //    .width(iced::Length::Fill)
        //    .height(iced::Length::Fill);

        let content = cosmic::widget::container(
            cosmic::widget::column()
                .push(
                    cosmic::widget::row()
                        .push(image_icon)
                        .push(cosmic::widget::column().push(image_label).push(image_description))
                        .height(Length::Fill)
                        .width(Length::Fill),
                )
                .push(image_file_open_button)
                .push(image_name)
                .push(image_size)
                .push(hash_row)
                .width(Length::Fill)
                .align_x(Alignment::Center),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .align_x(Alignment::Center)
        .align_y(Alignment::Center);
        //let z = cosmic::widget::row().push(cosmic::widget::column().push(image_icon)).push(
        //    cosmic::widget::column()
        //        .push(image_label)
        //        .push(image_description)
        //        .push(image_file_open_button)
        //        .push(image_name)
        //        .push(image_size)
        //        .push(hash_row)
        //        .width(iced::Length::Fill)
        //        .height(iced::Length::Fill)
        //        .align_x(iced::Alignment::Center),
        //);
        Element::from(content)
    }
}
