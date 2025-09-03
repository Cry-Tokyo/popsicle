use popsicle_cosmic::{app, i18n};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let requested_languages = i18n_embed::DesktopLanguageRequester::requested_languages();
    //i18n::init(&requested_languages);
    let settings = cosmic::app::Settings::default().size(cosmic::iced::Size::new(500., 300.));

    cosmic::app::run::<app::App>(settings, ())?;

    // write single arg cli to auto fit a iso

    // if let Some(iso_argument) = std::env::args().nth(1) {
    // let path = std::path::PathBuf::from(iso_argument);
    // if path.extension().map_or(false, |ext| {
    // let lower_ext = ext.to_str().unwrap().to_lowercase();
    // lower_ext == "iso" || lower_ext == "img"
    // }) && path.exists()
    // {
    // // send this to updater
    // let _ = app::App::update(&mut self, message);
    // }
    // }
    //
    Ok(())
}
