use std::marker::PhantomData;

use embedded_graphics::{
    prelude::{DrawTarget, PixelColor, Point},
    primitives::Rectangle,
};

use crate::{layoutable::Layoutable, ComponentSize};

pub fn expand<L: Layoutable<C>, C: PixelColor>(input: L) -> impl Layoutable<C> {
    ExpandLayoutable {
        layoutable: input,
        p: Default::default(),
        p1: PhantomData::<AreaExpander>,
    }
}
pub fn expand_horizontal<L: Layoutable<C>, C: PixelColor>(input: L) -> impl Layoutable<C> {
    ExpandLayoutable {
        layoutable: input,
        p: Default::default(),
        p1: PhantomData::<HorizontalExpander>,
    }
}
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
    ) -> Result<Point, DrawError> {
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
