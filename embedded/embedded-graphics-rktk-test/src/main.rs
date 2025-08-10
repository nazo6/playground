use embedded_graphics_simulator::{
    BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay, Window,
};

use embedded_graphics::{
    image::{Image, ImageRaw},
    mono_font::{
        MonoFont, MonoTextStyle, MonoTextStyleBuilder,
        ascii::{FONT_6X9, FONT_6X10, FONT_8X13, FONT_10X20},
    },
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::Rectangle,
    text::{Baseline, Text},
};
use embedded_text::{
    TextBox,
    alignment::HorizontalAlignment,
    style::{HeightMode, TextBoxStyleBuilder},
};
use images::*;
use rotate::RotatedDrawTarget;

mod images;
mod rotate;

const IMAGE_USB: Image<ImageRaw<BinaryColor>> = Image::new(
    &ImageRaw::new(
        &[
            0x00, 0x00, 0x00, 0x00, 0x01, 0x80, 0x03, 0xc0, 0x03, 0xc0, 0x01, 0x80, 0x19, 0x9c,
            0x19, 0xbc, 0x19, 0x9c, 0x19, 0x98, 0x19, 0xb8, 0x1f, 0xf0, 0x07, 0x80, 0x01, 0x80,
            0x03, 0xc0, 0x03, 0xc0, 0x03, 0xc0, 0x03, 0xc0, 0x00, 0x00, 0x00, 0x00,
        ],
        16,
    ),
    Point::zero(),
);

const SIZE: Size = Size::new(32, 128);

fn main() -> Result<(), core::convert::Infallible> {
    let mut display: SimulatorDisplay<BinaryColor> = SimulatorDisplay::new(SIZE);
    let output_settings = OutputSettingsBuilder::new()
        .theme(BinaryColorTheme::OledBlue)
        .build();

    horizontal_center(IMAGE_USB, IMAGE_BATTERY_1, &mut display);

    Text::with_baseline(
        "M",
        Point::new(0, 18),
        MonoTextStyle::new(&FONT_8X13, BinaryColor::On),
        Baseline::Top,
    )
    .draw(&mut display)
    .unwrap();
    Text::with_baseline(
        "R",
        Point::new(8, 18),
        MonoTextStyle::new(&FONT_8X13, BinaryColor::On),
        Baseline::Top,
    )
    .draw(&mut display)
    .unwrap();
    Text::with_baseline(
        "C",
        Point::new(16, 36),
        MonoTextStyleBuilder::new()
            .font(&FONT_8X13)
            .text_color(BinaryColor::Off)
            .background_color(BinaryColor::On)
            .build(),
        Baseline::Top,
    )
    .draw(&mut display)
    .unwrap();
    Text::with_baseline(
        "N",
        Point::new(24, 36),
        MonoTextStyle::new(&FONT_8X13, BinaryColor::On),
        Baseline::Top,
    )
    .draw(&mut display)
    .unwrap();

    {
        let mut rotated = RotatedDrawTarget::new(&mut display);

        let character_style = MonoTextStyle::new(&FONT_6X10, BinaryColor::On);
        let textbox_style = TextBoxStyleBuilder::new()
            .height_mode(HeightMode::FitToText)
            .alignment(HorizontalAlignment::Justified)
            .build();

        // Specify the bounding box. Note the 0px height. The `FitToText` height mode will
        // measure and adjust the height of the text box in `into_styled()`.
        let bounds = Rectangle::new(Point::new(2, 20), Size::new(100, 0));

        // Create the text box and apply styling options.
        TextBox::with_textbox_style(
            "lorem ipsumadadadadwadwadwa",
            bounds,
            character_style,
            textbox_style,
        )
        .draw(&mut rotated)
        .unwrap();
    }

    Window::new("Layout example", &output_settings).show_static(&display);
    Ok(())
}

fn horizontal_center<C>(
    a: impl Drawable<Color = C> + Transform + Dimensions,
    b: impl Drawable<Color = C> + Transform + Dimensions,
    d: &mut impl DrawTarget<Color = C, Error = impl core::fmt::Debug>,
) {
    let a_width = a.bounding_box().size.width as i32;
    let b_width = b.bounding_box().size.width as i32;
    let a_x = (SIZE.width as i32 - (a_width + b_width)) / 2;
    a.translate(Point::new(a_x, 0)).draw(d).unwrap();
    b.translate(Point::new(a_x + a_width, 0)).draw(d).unwrap();
}
