mod app;
mod i18n;

use cosmic::app::Settings;
use cosmic::iced::Size;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let settings = Settings::default().size(Size::new(500., 300.));

    cosmic::app::run::<app::App>(settings, ())?;

    Ok(())
}
