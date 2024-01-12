use std::marker::PhantomData;

use embedded_graphics::{
    prelude::{DrawTarget, PixelColor},
    primitives::Rectangle,
};

use crate::{layoutable::Layoutable, ComponentSize};

///
/// remove the maximum size constraints (set width and height to u32::MAX)
///
/// # Arguments
///
/// * `input`: element
///
/// returns: impl Layoutable<C>+Sized
///
/// # Examples
///
/// let center a text within the available space
/// ```
/// use embedded_graphics::mono_font::iso_8859_1::FONT_10X20;
/// use embedded_graphics::mono_font::MonoTextStyle;
/// use embedded_graphics::pixelcolor::BinaryColor;
/// use simple_layout::prelude::{center, expand, owned_text};
/// let value=42;
/// let centered_number = expand(center(owned_text(format!("{value:.1}"), MonoTextStyle::new(&FONT_10X20, BinaryColor::On))));
/// ```
pub fn expand<L: Layoutable<C>, C: PixelColor>(input: L) -> impl Layoutable<C> {
    ExpandLayoutable {
        layoutable: input,
        p: Default::default(),
        p1: PhantomData::<AreaExpander>,
    }
}
/// Let the containing element expand horizontally
pub fn expand_horizontal<L: Layoutable<C>, C: PixelColor>(input: L) -> impl Layoutable<C> {
    ExpandLayoutable {
        layoutable: input,
        p: Default::default(),
        p1: PhantomData::<HorizontalExpander>,
    }
}
/// Let the containing element expand vertically
pub fn expand_vertical<L: Layoutable<C>, C: PixelColor>(input: L) -> impl Layoutable<C> {
    ExpandLayoutable {
        layoutable: input,
        p: Default::default(),
        p1: PhantomData::<VerticalExpander>,
    }
}
trait Expander {
    fn expand_size(size: ComponentSize) -> ComponentSize;
}

impl<L: Layoutable<C>, C: PixelColor, E: Expander> Layoutable<C> for ExpandLayoutable<L, C, E> {
    fn size(&self) -> ComponentSize {
        E::expand_size(self.layoutable.size())
    }

    fn draw_placed<DrawError>(
        &self,
        target: &mut impl DrawTarget<Color = C, Error = DrawError>,
        position: Rectangle,
    ) -> Result<(), DrawError> {
        self.layoutable.draw_placed(target, position)
    }
}

struct AreaExpander;
impl Expander for AreaExpander {
    fn expand_size(size: ComponentSize) -> ComponentSize {
        let ComponentSize { width, height } = size;
        ComponentSize {
            width: width.expand_max(),
            height: height.expand_max(),
        }
    }
}

struct ExpandLayoutable<L: Layoutable<C>, C: PixelColor, E: Expander> {
    layoutable: L,
    p: PhantomData<C>,
    p1: PhantomData<E>,
}

struct HorizontalExpander;

impl Expander for HorizontalExpander {
    fn expand_size(size: ComponentSize) -> ComponentSize {
        let ComponentSize { width, height } = size;
        ComponentSize {
            width: width.expand_max(),
            height,
        }
    }
}

struct VerticalExpander;

impl Expander for VerticalExpander {
    fn expand_size(size: ComponentSize) -> ComponentSize {
        let ComponentSize { width, height } = size;
        ComponentSize {
            width,
            height: height.expand_max(),
        }
    }
}
