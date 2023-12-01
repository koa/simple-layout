use crate::{layoutable::Layoutable, ComponentSize};
use embedded_graphics::{
    prelude::{DrawTarget, PixelColor, Point},
    primitives::Rectangle,
};
use std::marker::PhantomData;

pub fn expand<L: Layoutable<C>, C: PixelColor>(input: L) -> impl Layoutable<C> {
    ExpandLayoutable {
        layoutable: input,
        p: Default::default(),
    }
}

struct ExpandLayoutable<L: Layoutable<C>, C: PixelColor> {
    layoutable: L,
    p: PhantomData<C>,
}

impl<L: Layoutable<C>, C: PixelColor> Layoutable<C> for ExpandLayoutable<L, C> {
    fn size(&self) -> ComponentSize {
        let ComponentSize { width, height } = self.layoutable.size();
        ComponentSize {
            width: width.expand_max(),
            height: height.expand_max(),
        }
    }

    fn draw_placed<DrawError>(
        &self,
        target: &mut impl DrawTarget<Color = C, Error = DrawError>,
        position: Rectangle,
    ) -> Result<Point, DrawError> {
        self.layoutable.draw_placed(target, position)
    }
}
