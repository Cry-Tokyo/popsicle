//! a
use cosmic::prelude::ColorExt;
/// a
pub fn red_style() -> cosmic::theme::TextInput {
    cosmic::theme::TextInput::Custom {
        active: Box::new(|t| {
            let palette = t.cosmic();
            let container = t.current_container();
            let background: cosmic::iced::Color = container.component.base.into();
            let corner = palette.corner_radii;

            let label_color = palette.palette.neutral_9;
            cosmic::widget::text_input::Appearance {
                background: background.into(),
                border_radius: corner.radius_s.into(),
                border_width: 2.,
                border_offset: None,
                border_color: container.component.divider.into(),
                label_color: label_color.into(),
                placeholder_color: {
                    let color: cosmic::iced::Color = container.on.into();
                    color.blend_alpha(background, 0.7)
                },
                selected_text_color: palette.on_accent_color().into(),
                icon_color: None,
                text_color: Some(cosmic::iced::Color::from_rgb8(255, 0, 0)),
                selected_fill: palette.accent_color().into(),
            }
        }),
        error: Box::new(|t| {
            let palette = t.cosmic();
            let container = t.current_container();
            let background: cosmic::iced::Color = container.component.base.into();
            let corner = palette.corner_radii;

            let label_color = palette.palette.neutral_9;
            cosmic::widget::text_input::Appearance {
                background: background.into(),
                border_radius: corner.radius_s.into(),
                border_width: 2.,
                border_offset: Some(2.),
                border_color: cosmic::iced::Color::from(palette.destructive_color()),
                icon_color: None,
                text_color: Some(cosmic::iced::Color::from_rgb8(255, 0, 0)),
                placeholder_color: {
                    let color: cosmic::iced::Color = container.on.into();
                    color.blend_alpha(background, 0.7)
                },
                selected_text_color: palette.on_accent_color().into(),
                selected_fill: palette.accent_color().into(),
                label_color: label_color.into(),
            }
        }),
        hovered: Box::new(|t| {
            let palette = t.cosmic();
            let container = t.current_container();
            let background: cosmic::iced::Color = container.component.base.into();
            let corner = palette.corner_radii;

            let label_color = palette.palette.neutral_9;
            cosmic::widget::text_input::Appearance {
                background: background.into(),
                border_radius: corner.radius_s.into(),
                border_width: 2.,
                border_offset: None,
                border_color: palette.accent.base.into(),
                icon_color: None,
                text_color: Some(cosmic::iced::Color::from_rgb8(255, 0, 0)),
                placeholder_color: {
                    let color: cosmic::iced::Color = container.on.into();
                    color.blend_alpha(background, 0.7)
                },
                selected_text_color: palette.on_accent_color().into(),
                selected_fill: palette.accent_color().into(),
                label_color: label_color.into(),
            }
        }),
        focused: Box::new(|t| {
            let palette = t.cosmic();
            let container = t.current_container();
            let background: cosmic::iced::Color = container.component.base.into();
            let corner = palette.corner_radii;

            let label_color = palette.palette.neutral_9;
            cosmic::widget::text_input::Appearance {
                background: background.into(),
                border_radius: corner.radius_s.into(),
                border_width: 2.,
                border_offset: Some(2.),
                border_color: palette.accent.base.into(),
                icon_color: None,
                text_color: Some(cosmic::iced::Color::from_rgb8(255, 0, 0)),
                placeholder_color: {
                    let color: cosmic::iced::Color = container.on.into();
                    color.blend_alpha(background, 0.7)
                },
                selected_text_color: palette.on_accent_color().into(),
                selected_fill: palette.accent_color().into(),
                label_color: label_color.into(),
            }
        }),
        disabled: Box::new(|t| {
            let palette = t.cosmic();
            let container = t.current_container();
            let background: cosmic::iced::Color = container.component.base.into();
            let corner = palette.corner_radii;

            let label_color = palette.palette.neutral_9;
            cosmic::widget::text_input::Appearance {
                background: background.into(),
                border_radius: corner.radius_s.into(),
                border_width: 2.,
                border_offset: None,
                border_color: container.component.divider.into(),
                label_color: label_color.into(),
                placeholder_color: {
                    let color: cosmic::iced::Color = container.on.into();
                    color.blend_alpha(background, 0.7)
                },
                selected_text_color: palette.on_accent_color().into(),
                icon_color: None,
                text_color: Some(cosmic::iced::Color::from_rgb8(255, 0, 0)),
                selected_fill: palette.accent_color().into(),
            }
        }),
    }
}
/// a
pub fn green_style() -> cosmic::theme::TextInput {
    cosmic::theme::TextInput::Custom {
        active: Box::new(|t| {
            let palette = t.cosmic();
            let container = t.current_container();
            let background: cosmic::iced::Color = container.component.base.into();
            let corner = palette.corner_radii;

            let label_color = palette.palette.neutral_9;
            cosmic::widget::text_input::Appearance {
                background: background.into(),
                border_radius: corner.radius_s.into(),
                border_width: 2.,
                border_offset: None,
                border_color: container.component.divider.into(),
                label_color: label_color.into(),
                placeholder_color: {
                    let color: cosmic::iced::Color = container.on.into();
                    color.blend_alpha(background, 0.7)
                },
                selected_text_color: palette.on_accent_color().into(),
                icon_color: None,
                text_color: Some(cosmic::iced::Color::from_rgb8(0, 255, 0)),
                selected_fill: palette.accent_color().into(),
            }
        }),
        error: Box::new(|t| {
            let palette = t.cosmic();
            let container = t.current_container();
            let background: cosmic::iced::Color = container.component.base.into();
            let corner = palette.corner_radii;

            let label_color = palette.palette.neutral_9;
            cosmic::widget::text_input::Appearance {
                background: background.into(),
                border_radius: corner.radius_s.into(),
                border_width: 2.,
                border_offset: Some(2.),
                border_color: cosmic::iced::Color::from(palette.destructive_color()),
                icon_color: None,
                text_color: Some(cosmic::iced::Color::from_rgb8(0, 255, 0)),
                placeholder_color: {
                    let color: cosmic::iced::Color = container.on.into();
                    color.blend_alpha(background, 0.7)
                },
                selected_text_color: palette.on_accent_color().into(),
                selected_fill: palette.accent_color().into(),
                label_color: label_color.into(),
            }
        }),
        hovered: Box::new(|t| {
            let palette = t.cosmic();
            let container = t.current_container();
            let background: cosmic::iced::Color = container.component.base.into();
            let corner = palette.corner_radii;

            let label_color = palette.palette.neutral_9;
            cosmic::widget::text_input::Appearance {
                background: background.into(),
                border_radius: corner.radius_s.into(),
                border_width: 2.,
                border_offset: None,
                border_color: palette.accent.base.into(),
                icon_color: None,
                text_color: cosmic::iced::Color::from_rgb8(0, 255, 0).into(),
                placeholder_color: {
                    let color: cosmic::iced::Color = container.on.into();
                    color.blend_alpha(background, 0.7)
                },
                selected_text_color: palette.on_accent_color().into(),
                selected_fill: palette.accent_color().into(),
                label_color: label_color.into(),
            }
        }),
        focused: Box::new(|t| {
            let palette = t.cosmic();
            let container = t.current_container();
            let background: cosmic::iced::Color = container.component.base.into();
            let corner = palette.corner_radii;

            let label_color = palette.palette.neutral_9;
            cosmic::widget::text_input::Appearance {
                background: background.into(),
                border_radius: corner.radius_s.into(),
                border_width: 2.,
                border_offset: Some(2.),
                border_color: palette.accent.base.into(),
                icon_color: None,
                text_color: Some(cosmic::iced::Color::from_rgb8(0, 255, 0)),
                placeholder_color: {
                    let color: cosmic::iced::Color = container.on.into();
                    color.blend_alpha(background, 0.7)
                },
                selected_text_color: palette.on_accent_color().into(),
                selected_fill: palette.accent_color().into(),
                label_color: label_color.into(),
            }
        }),
        disabled: Box::new(|t| {
            let palette = t.cosmic();
            let container = t.current_container();
            let background: cosmic::iced::Color = container.component.base.into();
            let corner = palette.corner_radii;

            let label_color = palette.palette.neutral_9;
            cosmic::widget::text_input::Appearance {
                background: background.into(),
                border_radius: corner.radius_s.into(),
                border_width: 2.,
                border_offset: None,
                border_color: container.component.divider.into(),
                label_color: label_color.into(),
                placeholder_color: {
                    let color: cosmic::iced::Color = container.on.into();
                    color.blend_alpha(background, 0.7)
                },
                selected_text_color: palette.on_accent_color().into(),
                icon_color: None,
                text_color: Some(cosmic::iced::Color::from_rgb8(0, 255, 0)),
                selected_fill: palette.accent_color().into(),
            }
        }),
    }
}
