use crate::{
    event::ClickHandler,
    properties::{
        FontIcon, FontIconProperty, OrientationProperty, PaddingProperty,
        PressedProperty, Text, TextProperty, Foreground, ForegroundProperty,
    },
    theme::Selector,
    styling,
    widget::{Container, FontIconBlock, SharedProperty, Stack, Template, TextBlock, Widget},
};

/// The `Button` widget can be clicked by user. It's used to perform an action.
///
/// # Properties
///
/// * `text` - String used to display the text of the button.
/// * `font_icon` - String used to display the font icon of the button.
/// * `selector` - CSS selector with  element name `button`, used to request the theme of the widget.
/// * `pressed` - Bool value represents the pressed state of the button.
pub struct Button;

impl Widget for Button {
    type Template = ButtonTemplate;

    fn create() -> Self::Template {
        let text = SharedProperty::new(Text::default());
        let icon = SharedProperty::new(FontIcon::default());
        let selector = SharedProperty::new(Selector::from("button"));
        let foreground = SharedProperty::new(Foreground::from(styling::LINK_WATER_COLOR));

        ButtonTemplate::new()
            .height(32.0)
            .min_width(80.0)
            .pressed(false)
            .debug_name("Button")
            .child(
                Container::create()
                    .padding((8.0, 0.0, 8.0, 0.0))
                    .shared_selector(selector.clone())
                    .child(
                        Stack::create()
                            .orientation("Horizontal")
                            .vertical_alignment("Center")
                            .horizontal_alignment("Center")
                            .child(
                                FontIconBlock::create()
                                    .margin((0.0, 0.0, 2.0, 0.0))
                                    .shared_font_icon(icon.clone())
                                    .shared_selector(selector.clone()),
                            )
                            .child(
                                TextBlock::create()
                                    .shared_foreground(foreground.clone())
                                    .shared_text(text.clone())
                                    .shared_selector(selector.clone()),
                            ),
                    ),
            )
            .shared_text(text)
            .shared_font_icon(icon)
            .shared_foreground(foreground)
            .shared_selector(selector)
    }
}

template!(
    ButtonTemplate,
    [
        TextProperty,
        FontIconProperty,
        ForegroundProperty,
        PressedProperty,
        ClickHandler
    ]
);
