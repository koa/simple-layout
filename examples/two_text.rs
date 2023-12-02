use chrono::Local;
use embedded_graphics::{
    geometry::Dimensions,
    mono_font::{iso_8859_1::FONT_6X9, MonoTextStyle},
    pixelcolor::BinaryColor,
    prelude::{Point, Size},
    text::Text,
};
use embedded_graphics_simulator::{
    BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay, Window,
};

use simple_layout::prelude::{center, expand, vertical_layout, Bordered, Layoutable};

fn main() -> Result<(), core::convert::Infallible> {
    let mut display = SimulatorDisplay::<BinaryColor>::new(Size::new(64, 128));

    let text_style = MonoTextStyle::new(&FONT_6X9, BinaryColor::On);

    let clock = Local::now().format("%H:%M").to_string();
    let clock_text = Text::new(&clock, Point::zero(), text_style);

    let rectangle = display.bounding_box();
    let pressure_string = "Footer\nXYq";
    vertical_layout(
        expand(Bordered::new(center(clock_text), 0, 1, BinaryColor::On)),
        1,
    )
    .append(
        expand(center(Text::new(
            pressure_string,
            Point::zero(),
            text_style,
        ))),
        2,
    )
    .draw_placed(&mut display, rectangle)?;

    let output_settings = OutputSettingsBuilder::new()
        .theme(BinaryColorTheme::LcdWhite)
        .build();
    Window::new("Display Test", &output_settings).show_static(&display);

    Ok(())
}
