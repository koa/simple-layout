#[cfg(feature = "simulate-example")]
use std::{thread, time::Duration};

#[cfg(feature = "simulate-example")]
use chrono::Local;
#[cfg(feature = "simulate-example")]
use embedded_graphics::{
    geometry::Dimensions,
    mono_font::{iso_8859_1::FONT_6X9, MonoTextStyle},
    pixelcolor::BinaryColor,
    prelude::{DrawTarget, Point, Size},
    text::Text,
};
#[cfg(feature = "simulate-example")]
use embedded_graphics_simulator::{
    BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};
#[cfg(feature = "simulate-example")]
use env_logger::Env;

#[cfg(feature = "simulate-example")]
use simple_layout::prelude::{
    bordered, center, expand, horizontal_layout, optional_placement, padding, scale, south,
    vertical_layout, DashedLine, Layoutable, RoundedLine,
};

#[cfg(not(feature = "simulate-example"))]
fn main() {
    // dummy
    println!("Enable feature simulate-example for a test");
}
#[cfg(feature = "simulate-example")]
fn main() -> Result<(), core::convert::Infallible> {
    env_logger::init_from_env(Env::default().filter_or("LOG_LEVEL", "info"));

    let text_style = MonoTextStyle::new(&FONT_6X9, BinaryColor::On);

    let clock = Local::now().format("%H:%M").to_string();
    let clock_text = Text::new(&clock, Point::zero(), text_style);

    let output_settings = OutputSettingsBuilder::new()
        .theme(BinaryColorTheme::LcdWhite)
        .build();
    let mut window = Window::new("Display Test", &output_settings);
    let mut modified = true;
    let mut minus_pos = None;
    let mut plus_pos = None;
    let mut scale_value = 0.5;
    let mut display = SimulatorDisplay::<BinaryColor>::new(Size::new(64, 128));
    let rectangle = display.bounding_box();

    'running: loop {
        if modified {
            display.clear(BinaryColor::Off)?;
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
                    optional_placement(
                        &mut minus_pos,
                        bordered(
                            padding(Text::new("-", Point::zero(), text_style), -1, 0, -1, 0),
                            RoundedLine::new(BinaryColor::On),
                        ),
                    ),
                    0,
                )
                .append(south(scale(scale_value, BinaryColor::On)), 1)
                .append(
                    optional_placement(
                        &mut plus_pos,
                        bordered(
                            padding(Text::new("+", Point::zero(), text_style), -1, 0, -1, 0),
                            RoundedLine::new(BinaryColor::On),
                        ),
                    ),
                    0,
                ),
                0,
            )
            .draw_placed(&mut display, rectangle)?;
            window.update(&display);
        }
        modified = false;

        for event in window.events() {
            match event {
                SimulatorEvent::KeyUp { .. } => {}
                SimulatorEvent::KeyDown { .. } => {}
                SimulatorEvent::MouseButtonUp { point, .. } => {
                    if plus_pos.map(|p| p.contains(point)).unwrap_or(false) {
                        scale_value += 0.2;
                        modified = true;
                    }
                    if minus_pos.map(|p| p.contains(point)).unwrap_or(false) {
                        scale_value -= 0.2;
                        modified = true;
                    }
                }
                SimulatorEvent::MouseButtonDown { .. } => {}
                SimulatorEvent::MouseWheel { .. } => {}
                SimulatorEvent::MouseMove { .. } => {}
                SimulatorEvent::Quit => {
                    break 'running;
                }
            }
        }

        thread::sleep(Duration::from_millis(20));
    }

    Ok(())
}
