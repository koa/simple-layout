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

use simple_layout::prelude::{
    bordered, center, expand, horizontal_layout, north, padding, scale, south, vertical_layout,
    DashedLine, Layoutable, RoundedLine,
};

fn main() -> Result<(), core::convert::Infallible> {
    let mut display = SimulatorDisplay::<BinaryColor>::new(Size::new(64, 128));

    let text_style = MonoTextStyle::new(&FONT_6X9, BinaryColor::On);

    let clock = Local::now().format("%H:%M").to_string();
    let clock_text = Text::new(&clock, Point::zero(), text_style);

    let rectangle = display.bounding_box();
    let pressure_string = "Footer\nXYq";
    vertical_layout(
        expand(bordered(
            center(clock_text),
            DashedLine::new(2, 2, BinaryColor::On),
        )),
        1,
    )
    .append(
        expand(bordered(
            center(Text::new(pressure_string, Point::zero(), text_style)),
            RoundedLine::new(BinaryColor::On),
        )),
        2,
    )
    .append(
        horizontal_layout(
            bordered(
                padding(Text::new("-", Point::zero(), text_style), -1, 0, -1, 0),
                RoundedLine::new(BinaryColor::On),
            ),
            0,
        )
        .append(south(scale(1.0, BinaryColor::On)), 1)
        .append(
            bordered(
                padding(Text::new("+", Point::zero(), text_style), -1, 0, -1, 0),
                RoundedLine::new(BinaryColor::On),
            ),
            0,
        ),
        0,
    )
    .draw_placed(&mut display, rectangle)?;

    let output_settings = OutputSettingsBuilder::new()
        .theme(BinaryColorTheme::LcdWhite)
        .build();
    Window::new("Display Test", &output_settings).show_static(&display);

    Ok(())
}
