use std::marker::PhantomData;

use embedded_graphics::{
    geometry::Size,
    image::Image,
    prelude::{Dimensions, DrawTarget, ImageDrawable, PixelColor, Point},
    primitives::Rectangle,
    text::{renderer::TextRenderer, Text, TextStyle, TextStyleBuilder},
    Drawable,
};

use crate::{draw::OffsetDrawable, ComponentSize};

///
/// Defines any Layoutable element (anything that can be layouted)
///
pub trait Layoutable<Color: PixelColor> {
    ///
    /// Returns size constraints of that element
    /// returns ComponentSize preferred placement constraints of this element
    fn size(&self) -> ComponentSize;
    ///
    /// Draws this element onto a defined region onto a target
    ///
    /// # Arguments
    ///
    /// * `target`: Target where this element should be drawn
    /// * `position`: place to draw this element onto
    ///
    /// returns: Result<(), DrawError>
    ///
    fn draw_placed<DrawError>(
        &self,
        target: &mut impl DrawTarget<Color = Color, Error = DrawError>,
        position: Rectangle,
    ) -> Result<(), DrawError>;
}

///
/// Generates a Layoutable text around a owned (possible generated) string
///
/// # Arguments
///
/// * `text`: String to render
/// * `character_style`: Font and style of the text
///
/// returns: impl Layoutable<C>+Sized
///
/// # Examples
///
/// ```
/// use embedded_graphics::{
///     mono_font::{
///         iso_8859_1::FONT_6X12,
///         MonoTextStyle
///     },
///     pixelcolor::BinaryColor
/// };
/// use simple_layout::prelude::owned_text;
/// let temperature=21.3;
/// let temperature_layout = owned_text(format!("{temperature:.1}°C"), MonoTextStyle::new(&FONT_6X12, BinaryColor::On));
///
/// ```
pub fn owned_text<S: TextRenderer<Color = C> + Copy, C: PixelColor, StrValue: Into<Box<str>>>(
    text: StrValue,
    character_style: S,
) -> impl Layoutable<C> {
    OwnedText {
        text: text.into(),
        character_style,
        text_style: TextStyleBuilder::new().build(),
        p: Default::default(),
    }
}
struct OwnedText<S, C: PixelColor> {
    text: Box<str>,
    character_style: S,
    text_style: TextStyle,
    p: PhantomData<C>,
}

impl<S: TextRenderer<Color = C> + Copy, C: PixelColor> Layoutable<C> for OwnedText<S, C> {
    fn size(&self) -> ComponentSize {
        Text::with_text_style(
            &self.text,
            Point::zero(),
            self.character_style,
            self.text_style,
        )
        .size()
    }

    fn draw_placed<DrawError>(
        &self,
        target: &mut impl DrawTarget<Color = C, Error = DrawError>,
        position: Rectangle,
    ) -> Result<(), DrawError> {
        Text::with_text_style(
            &self.text,
            Point::zero(),
            self.character_style,
            self.text_style,
        )
        .draw_placed(target, position)
    }
}

///
/// Render Text as Layoutable
///
impl<'a, S: TextRenderer<Color = Color>, Color: PixelColor> Layoutable<Color> for Text<'a, S> {
    fn size(&self) -> ComponentSize {
        let mut total_height = 0;
        let mut max_line_length = 0;
        for line in self.text.split('\n') {
            let metrics = self.character_style.measure_string(
                line,
                Point::default(),
                self.text_style.baseline,
            );
            let bbox = metrics.bounding_box;
            if bbox.size.width > max_line_length {
                max_line_length = bbox.size.width;
            }
            total_height += bbox.size.height;
        }
        ComponentSize::fixed_size(max_line_length - 1, total_height)
    }

    fn draw_placed<DrawError>(
        &self,
        target: &mut impl DrawTarget<Color = Color, Error = DrawError>,
        position: Rectangle,
    ) -> Result<(), DrawError> {
        let offset = if let Some(first_line) = self.text.split('\n').next() {
            self.character_style
                .measure_string(first_line, Point::default(), self.text_style.baseline)
                .bounding_box
                .top_left
        } else {
            Point::zero()
        };
        let offset = position.top_left - self.position - offset;
        self.draw(&mut OffsetDrawable::new(target, offset))?;
        Ok(())
    }
}

///
/// Render Image as Layoutable
///
impl<'a, C: PixelColor, T: ImageDrawable<Color = C>> Layoutable<C> for Image<'a, T> {
    fn size(&self) -> ComponentSize {
        let Rectangle {
            size: Size { width, height },
            ..
        } = self.bounding_box();
        ComponentSize::fixed_size(width, height)
    }

    fn draw_placed<DrawError>(
        &self,
        target: &mut impl DrawTarget<Color = C, Error = DrawError>,
        position: Rectangle,
    ) -> Result<(), DrawError> {
        let Rectangle { top_left, .. } = self.bounding_box();
        let offset = position.top_left - top_left;
        self.draw(&mut OffsetDrawable::new(target, offset))?;
        Ok(())
    }
}

///
/// Render a `Optional<impl Layoutable> as Layoutable`
///
impl<C: PixelColor, L: Layoutable<C>> Layoutable<C> for Option<L> {
    fn size(&self) -> ComponentSize {
        match self {
            None => ComponentSize::default(),
            Some(l) => l.size(),
        }
    }

    fn draw_placed<DrawError>(
        &self,
        target: &mut impl DrawTarget<Color = C, Error = DrawError>,
        position: Rectangle,
    ) -> Result<(), DrawError> {
        match self {
            None => Ok(()),
            Some(l) => l.draw_placed(target, position),
        }
    }
}
