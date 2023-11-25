use std::{
    cmp::Ordering,
    marker::PhantomData,
    ops::{AddAssign, Deref, Range},
};

use embedded_graphics::{
    geometry::{Dimensions, Size},
    pixelcolor::PixelColor,
    prelude::{DrawTarget, Point},
    primitives::Rectangle,
    text::renderer::TextRenderer,
    text::Text,
    Drawable, Pixel,
};

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct ComponentSize {
    width: ValueRange<u32>,
    height: ValueRange<u32>,
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct ValueRange<V> {
    preferred_value: V,
    min_value: V,
    max_value: V,
}

impl<V: PartialOrd + Clone> ValueRange<V> {
    fn expand(&mut self, rhs: &Self) {
        if self.preferred_value < rhs.preferred_value {
            self.preferred_value = rhs.preferred_value.clone();
        }
        if self.min_value < rhs.min_value {
            self.min_value = rhs.min_value.clone()
        }
        if self.max_value < rhs.max_value {
            self.max_value = rhs.max_value.clone()
        }
    }
}

impl<V: AddAssign> AddAssign for ValueRange<V> {
    fn add_assign(&mut self, rhs: Self) {
        self.preferred_value += rhs.preferred_value;
        self.min_value += rhs.min_value;
        self.max_value += rhs.max_value;
    }
}

impl<V: Clone> ValueRange<V> {
    fn fixed(value: V) -> Self {
        Self {
            preferred_value: value.clone(),
            min_value: value.clone(),
            max_value: value,
        }
    }
}

impl ValueRange<u32> {
    fn expand_max(&self) -> Self {
        Self {
            preferred_value: self.preferred_value,
            min_value: self.min_value,
            max_value: u32::MAX,
        }
    }
}

impl ComponentSize {
    pub fn fixed_size(width: u32, height: u32) -> ComponentSize {
        ComponentSize {
            width: ValueRange::fixed(width),
            height: ValueRange::fixed(height),
        }
    }
    pub fn new(
        preferred_width: u32,
        preferred_height: u32,
        width_range: Range<u32>,
        height_range: Range<u32>,
    ) -> Self {
        Self {
            width: ValueRange {
                preferred_value: preferred_width,
                min_value: width_range.start,
                max_value: width_range.end,
            },
            height: ValueRange {
                preferred_value: preferred_height,
                min_value: height_range.start,
                max_value: height_range.end,
            },
        }
    }
}

pub trait Orientation {
    fn split_component_size(size: ComponentSize) -> (ValueRange<u32>, ValueRange<u32>);
    fn split_size(size: Size) -> (u32, u32);
    fn split_point(p: Point) -> (i32, i32);
    fn create_component_size(along: ValueRange<u32>, cross: ValueRange<u32>) -> ComponentSize;
    fn create_size(along: u32, across: u32) -> Size;
    fn create_point(along: i32, cross: i32) -> Point;
}

pub struct Horizontal {}

impl Orientation for Horizontal {
    #[inline]
    fn split_component_size(size: ComponentSize) -> (ValueRange<u32>, ValueRange<u32>) {
        (size.width, size.height)
    }

    #[inline]
    fn split_size(size: Size) -> (u32, u32) {
        (size.width, size.height)
    }

    #[inline]
    fn split_point(p: Point) -> (i32, i32) {
        let Point { x, y } = p;
        (x, y)
    }

    #[inline]
    fn create_component_size(along: ValueRange<u32>, cross: ValueRange<u32>) -> ComponentSize {
        ComponentSize {
            width: along,
            height: cross,
        }
    }

    #[inline]
    fn create_size(along: u32, across: u32) -> Size {
        Size {
            width: along,
            height: across,
        }
    }

    #[inline]
    fn create_point(along: i32, cross: i32) -> Point {
        Point { x: along, y: cross }
    }
}

pub struct Vertical {}

impl Orientation for Vertical {
    fn split_component_size(size: ComponentSize) -> (ValueRange<u32>, ValueRange<u32>) {
        (size.height, size.width)
    }

    fn split_size(size: Size) -> (u32, u32) {
        (size.height, size.width)
    }

    fn split_point(p: Point) -> (i32, i32) {
        (p.y, p.x)
    }

    fn create_component_size(along: ValueRange<u32>, cross: ValueRange<u32>) -> ComponentSize {
        ComponentSize {
            width: cross,
            height: along,
        }
    }

    fn create_size(along: u32, across: u32) -> Size {
        Size {
            width: across,
            height: along,
        }
    }

    fn create_point(along: i32, cross: i32) -> Point {
        Point { x: cross, y: along }
    }
}

pub trait Layoutable<Color: PixelColor> {
    fn size(&self) -> ComponentSize;
    fn draw_placed<DrawError>(
        &self,
        target: &mut impl DrawTarget<Color = Color, Error = DrawError>,
        position: Rectangle,
    ) -> Result<Point, DrawError>;
}

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
        ComponentSize::fixed_size(max_line_length, total_height)
    }

    fn draw_placed<DrawError>(
        &self,
        target: &mut impl DrawTarget<Color = Color, Error = DrawError>,
        position: Rectangle,
    ) -> Result<Point, DrawError> {
        let offset = if let Some(first_line) = self.text.split('\n').next() {
            self.character_style
                .measure_string(first_line, Point::default(), self.text_style.baseline)
                .bounding_box
                .top_left
        } else {
            Point::zero()
        };
        let offset = position.top_left - self.position - offset;
        Drawable::draw(self, &mut OffsetDrawable { target, offset })
    }
}

struct OffsetDrawable<'a, Color, Error, Target>
where
    Target: DrawTarget<Color = Color, Error = Error>,
    Color: PixelColor,
{
    target: &'a mut Target,
    offset: Point,
}

impl<'a, Color, Error, Target> Dimensions for OffsetDrawable<'a, Color, Error, Target>
where
    Target: DrawTarget<Color = Color, Error = Error>,
    Color: PixelColor,
{
    fn bounding_box(&self) -> Rectangle {
        let bbox = self.target.bounding_box();
        Rectangle {
            top_left: bbox.top_left - self.offset,
            size: bbox.size,
        }
    }
}

impl<'a, Color, Error, Target> DrawTarget for OffsetDrawable<'a, Color, Error, Target>
where
    Target: DrawTarget<Color = Color, Error = Error>,
    Color: PixelColor,
{
    type Color = Color;
    type Error = Error;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        let offset = self.offset;
        self.target.draw_iter(
            pixels
                .into_iter()
                .map(|Pixel::<Self::Color>(p, c)| Pixel(p + offset, c)),
        )
    }
}
pub trait LinearLayout<C: PixelColor, O: Orientation>: Sized {
    fn len() -> usize;
    fn fill_sizes(&self, sizes: &mut [ComponentSize]);
    fn fill_weights(&self, weights: &mut [u32]);
    fn draw_placed_components<DrawError>(
        &self,
        target: &mut impl DrawTarget<Color = C, Error = DrawError>,
        places: &[Rectangle],
    ) -> Result<Point, DrawError>;
}

#[derive(Default, Debug)]
pub struct SingleLinearLayout<L: Layoutable<C>, C: PixelColor, O: Orientation> {
    layout: L,
    weight: u32,
    p1: PhantomData<C>,
    p2: PhantomData<O>,
}

impl<L: Layoutable<C>, C: PixelColor, O: Orientation> Layoutable<C>
    for SingleLinearLayout<L, C, O>
{
    fn size(&self) -> ComponentSize {
        self.layout.size()
    }

    fn draw_placed<DrawError>(
        &self,
        target: &mut impl DrawTarget<Color = C, Error = DrawError>,
        position: Rectangle,
    ) -> Result<Point, DrawError> {
        self.layout.draw_placed(target, position)
    }
}

impl<L: Layoutable<C>, C: PixelColor, O: Orientation> LinearLayout<C, O>
    for SingleLinearLayout<L, C, O>
{
    #[inline]
    fn len() -> usize {
        1
    }

    #[inline]
    fn fill_sizes(&self, sizes: &mut [ComponentSize]) {
        sizes[0] = self.size();
    }

    #[inline]
    fn fill_weights(&self, weights: &mut [u32]) {
        weights[0] = self.weight;
    }

    #[inline]
    fn draw_placed_components<DrawError>(
        &self,
        target: &mut impl DrawTarget<Color = C, Error = DrawError>,
        places: &[Rectangle],
    ) -> Result<Point, DrawError> {
        self.layout.draw_placed(target, places[0])
    }
}

pub struct LayoutableLinearLayout<C: PixelColor, O: Orientation, LL: LinearLayout<C, O>>(
    LL,
    PhantomData<C>,
    PhantomData<O>,
);

impl<C: PixelColor, O: Orientation, LL: LinearLayout<C, O>> LayoutableLinearLayout<C, O, LL> {
    pub fn append<L>(
        self,
        l: L,
        weight: u32,
    ) -> LayoutableLinearLayout<C, O, ChainingLinearLayout<LL, L, C, O>>
    where
        L: Layoutable<C>,
    {
        LayoutableLinearLayout(
            ChainingLinearLayout {
                base_layout: self.0,
                layoutable: l,
                weight,
                p: Default::default(),
                o: Default::default(),
            },
            PhantomData,
            PhantomData,
        )
    }
}

impl<C: PixelColor, O: Orientation, LL: LinearLayout<C, O>> From<LL>
    for LayoutableLinearLayout<C, O, LL>
{
    fn from(value: LL) -> Self {
        LayoutableLinearLayout(value, PhantomData, PhantomData)
    }
}

impl<C: PixelColor, O: Orientation, LL: LinearLayout<C, O>> Layoutable<C>
    for LayoutableLinearLayout<C, O, LL>
{
    fn size(&self) -> ComponentSize {
        let mut sizes = vec![ComponentSize::default(); LL::len()].into_boxed_slice();
        self.0.fill_sizes(&mut sizes);
        let mut total_along = ValueRange::default();
        let mut total_cross = ValueRange::default();
        for size in sizes.iter() {
            let (along, cross) = O::split_component_size(*size);
            total_along += along;
            total_cross.expand(&cross);
        }
        O::create_component_size(total_along, total_cross)
    }
    fn draw_placed<DrawError>(
        &self,
        target: &mut impl DrawTarget<Color = C, Error = DrawError>,
        position: Rectangle,
    ) -> Result<Point, DrawError> {
        let (along_target, cross_target) = O::split_size(position.size);
        let (mut along_offset, cross_offset) = O::split_point(position.top_left);

        let mut sizes = vec![ComponentSize::default(); LL::len()].into_boxed_slice();
        self.0.fill_sizes(&mut sizes);
        let sizes = sizes
            .iter()
            .map(|s| O::split_component_size(*s).0)
            .collect::<Box<_>>();
        let preferred_sizes = sizes.iter().map(|s| s.preferred_value).collect::<Box<_>>();
        let total_preferred: u32 = preferred_sizes.iter().sum();
        let places = match along_target.cmp(&total_preferred) {
            Ordering::Less => {
                let min_sizes = sizes.iter().map(|s| s.min_value).collect::<Box<_>>();
                let total_min = min_sizes.iter().map(|v| *v as u64).sum::<u64>();
                if total_min >= along_target as u64 {
                    min_sizes
                } else {
                    let mut remaining_budget = total_preferred - along_target;
                    let mut result_sizes = preferred_sizes;
                    let mut weights = vec![0; LL::len()].into_boxed_slice();
                    self.0.fill_weights(&mut weights);
                    while remaining_budget > 0 {
                        let remaining_budget_before = remaining_budget;
                        let mut entries_with_headroom = weights
                            .iter()
                            .zip(result_sizes.iter_mut())
                            .zip(sizes.iter())
                            .filter(|((weight, result_size), size)| {
                                **weight > 0 && **result_size > size.min_value
                            })
                            .collect::<Box<_>>();
                        let mut remaining_weights: u32 = entries_with_headroom
                            .iter()
                            .map(|((weight, _), _)| **weight)
                            .sum();
                        if remaining_weights == 0 {
                            break;
                        }
                        for ((weight, result_size), size) in entries_with_headroom.iter_mut() {
                            let theoretical_decrease =
                                remaining_budget * *weight / remaining_weights;
                            let selected_decrease =
                                (theoretical_decrease).min(**result_size - size.min_value);
                            **result_size -= selected_decrease;
                            remaining_budget -= theoretical_decrease;
                            remaining_weights -= *weight;
                        }
                        if remaining_budget_before == remaining_budget {
                            // nothing more to distribute -> break
                            break;
                        }
                    }
                    result_sizes
                }
            }
            Ordering::Equal => preferred_sizes,
            Ordering::Greater => {
                let max_sizes = sizes.iter().map(|s| s.max_value).collect::<Box<_>>();
                let total_max = max_sizes.iter().map(|v| *v as u64).sum::<u64>();
                if total_max <= along_target as u64 {
                    max_sizes
                } else {
                    let mut remaining_budget = along_target - total_preferred;
                    let mut result_sizes = preferred_sizes;
                    let mut weights = vec![0; LL::len()].into_boxed_slice();
                    self.0.fill_weights(&mut weights);
                    while remaining_budget > 0 {
                        let remaining_budget_before = remaining_budget;
                        let mut entries_with_headroom = weights
                            .iter()
                            .zip(result_sizes.iter_mut())
                            .zip(sizes.iter())
                            .filter(|((weight, result_size), size)| {
                                **weight > 0 && **result_size < size.max_value
                            })
                            .collect::<Box<_>>();
                        let mut remaining_weights: u32 = entries_with_headroom
                            .iter()
                            .map(|((weight, _), _)| **weight)
                            .sum();
                        if remaining_weights == 0 {
                            break;
                        }

                        for ((weight, result_size), size) in entries_with_headroom.iter_mut() {
                            let theoretical_increase =
                                remaining_budget * *weight / remaining_weights;
                            let selected_increase =
                                (theoretical_increase).min(size.max_value - **result_size);
                            **result_size += selected_increase;
                            remaining_budget -= theoretical_increase;
                            remaining_weights -= *weight;
                        }
                        if remaining_budget_before == remaining_budget {
                            // nothing more to distribute -> break
                            break;
                        }
                    }
                    result_sizes
                }
            }
        }
        .iter()
        .map(|l| {
            let place = Rectangle {
                top_left: O::create_point(along_offset, cross_offset),
                size: O::create_size(*l, cross_target),
            };
            along_offset += *l as i32;
            place
        })
        .collect::<Box<_>>();
        self.0.draw_placed_components(target, &places)
    }
}

impl<C: PixelColor, O: Orientation, LL: LinearLayout<C, O>> Deref
    for LayoutableLinearLayout<C, O, LL>
{
    type Target = LL;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/*
impl<L1: Layoutable<C>, L2: Layoutable<C>, C: PixelColor, O: Orientation> From<(L1, L2)>
    for LinearPair<L1, L2, C, O>
{
    fn from((l1, l2): (L1, L2)) -> Self {
        Self {
            l1,
            l2,
            weights: [1, 1],
            p1: PhantomData,
            p2: PhantomData,
        }
    }
}
 */
pub struct ChainingLinearLayout<
    LL: LinearLayout<C, O>,
    L: Layoutable<C>,
    C: PixelColor,
    O: Orientation,
> {
    base_layout: LL,
    layoutable: L,
    weight: u32,
    p: PhantomData<C>,
    o: PhantomData<O>,
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

impl<LL: LinearLayout<C, O>, L: Layoutable<C>, C: PixelColor, O: Orientation> LinearLayout<C, O>
    for ChainingLinearLayout<LL, L, C, O>
{
    #[inline]
    fn len() -> usize {
        LL::len() + 1
    }

    #[inline]
    fn fill_sizes(&self, sizes: &mut [ComponentSize]) {
        let idx = Self::len() - 1;
        self.base_layout.fill_sizes(&mut sizes[0..idx]);
        sizes[idx] = self.layoutable.size();
    }

    #[inline]
    fn fill_weights(&self, weights: &mut [u32]) {
        let idx = Self::len() - 1;
        self.base_layout.fill_weights(&mut weights[0..idx]);
        weights[idx] = self.weight;
    }

    #[inline]
    fn draw_placed_components<DrawError>(
        &self,
        target: &mut impl DrawTarget<Color = C, Error = DrawError>,
        places: &[Rectangle],
    ) -> Result<Point, DrawError> {
        let idx = Self::len() - 1;
        self.base_layout
            .draw_placed_components(target, &places[0..idx])?;
        self.layoutable.draw_placed(target, places[idx])
    }
}

pub fn expand<L: Layoutable<C>, C: PixelColor>(input: L) -> impl Layoutable<C> {
    ExpandLayoutable {
        layoutable: input,
        p: Default::default(),
    }
}

pub fn vertical_layout<L: Layoutable<C>, C: PixelColor>(
    l: L,
    w: u32,
) -> LayoutableLinearLayout<C, Vertical, SingleLinearLayout<L, C, Vertical>> {
    LayoutableLinearLayout(
        SingleLinearLayout {
            layout: l,
            weight: w,
            p1: PhantomData,
            p2: PhantomData,
        },
        PhantomData,
        PhantomData,
    )
}
pub fn horizontal_layout<L: Layoutable<C>, C: PixelColor>(
    l: L,
    w: u32,
) -> LayoutableLinearLayout<C, Horizontal, SingleLinearLayout<L, C, Horizontal>> {
    LayoutableLinearLayout(
        SingleLinearLayout {
            layout: l,
            weight: w,
            p1: PhantomData,
            p2: PhantomData,
        },
        PhantomData,
        PhantomData,
    )
}
