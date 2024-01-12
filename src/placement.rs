use std::marker::PhantomData;
use std::ops::DerefMut;
use std::sync::Mutex;

use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::pixelcolor::PixelColor;
use embedded_graphics::primitives::Rectangle;
#[cfg(feature = "log")]
use log::warn;

use crate::layoutable::Layoutable;
use crate::ComponentSize;

///
/// Get a callback from the layout process about the final placement of the containing element. So you can
/// map a touched point onto the correct element
///
/// # Arguments
///
/// * `callback`: Callback processing the information about the latest placement of the containing layoutable
/// * `layoutable`: Element to be watched
///
/// returns: impl Layoutable<C>+Sized
///
pub fn callback_placement<L: Layoutable<C>, C: PixelColor, F: FnMut(Rectangle)>(
    callback: F,
    layoutable: L,
) -> impl Layoutable<C> {
    CallbackPlacement {
        callback: Mutex::new(callback),
        layoutable,
        p: PhantomData,
    }
}
///
/// Updates the placement onto a Option<Rectangle>
///
/// # Arguments
///
/// * `target`: target variable to update with the found placement informations
/// * `layoutable`: layoutable to be watched
///
/// returns: impl Layoutable<C>+Sized
///
pub fn optional_placement<'a, L: Layoutable<C> + 'a, C: PixelColor + 'a>(
    target: &'a mut Option<Rectangle>,
    layoutable: L,
) -> impl Layoutable<C> + 'a {
    callback_placement(|rectangle: Rectangle| *target = Some(rectangle), layoutable)
}

struct CallbackPlacement<L: Layoutable<C>, C: PixelColor, F: FnMut(Rectangle)> {
    callback: Mutex<F>,
    layoutable: L,
    p: PhantomData<C>,
}

impl<L: Layoutable<C>, C: PixelColor, F: FnMut(Rectangle)> Layoutable<C>
    for CallbackPlacement<L, C, F>
{
    fn size(&self) -> ComponentSize {
        self.layoutable.size()
    }

    fn draw_placed<DrawError>(
        &self,
        target: &mut impl DrawTarget<Color = C, Error = DrawError>,
        position: Rectangle,
    ) -> Result<(), DrawError> {
        if let Ok(mut mutex) = self.callback.try_lock() {
            (mutex.deref_mut())(position);
        } else {
            #[cfg(feature = "log")]
            warn!("Cannot lock callback");
        }
        self.layoutable.draw_placed(target, position)
    }
}
