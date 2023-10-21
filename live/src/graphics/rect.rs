use std::ops::{Add, Sub, Div, Mul, MulAssign, SubAssign, AddAssign, Neg};

use super::ffi as ffi;

type Pos2 = Vec2;

#[derive(Clone, Copy, PartialEq)]
pub struct Vec2{
    pub x: f32,
    pub y: f32,
}
impl Neg for Vec2 {
    type Output = Vec2;

    #[inline(always)]
    fn neg(self) -> Vec2 {
        vec2(-self.x, -self.y)
    }
}

impl AddAssign for Vec2 {
    #[inline(always)]
    fn add_assign(&mut self, rhs: Vec2) {
        *self = Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        };
    }
}

impl SubAssign for Vec2 {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: Vec2) {
        *self = Vec2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        };
    }
}

impl Add for Vec2 {
    type Output = Vec2;

    #[inline(always)]
    fn add(self, rhs: Vec2) -> Vec2 {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Vec2 {
    type Output = Vec2;

    #[inline(always)]
    fn sub(self, rhs: Vec2) -> Vec2 {
        Vec2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

/// Element-wise multiplication
impl Mul<Vec2> for Vec2 {
    type Output = Vec2;

    #[inline(always)]
    fn mul(self, vec: Vec2) -> Vec2 {
        Vec2 {
            x: self.x * vec.x,
            y: self.y * vec.y,
        }
    }
}

/// Element-wise division
impl Div<Vec2> for Vec2 {
    type Output = Vec2;

    #[inline(always)]
    fn div(self, rhs: Vec2) -> Vec2 {
        Vec2 {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}

impl MulAssign<f32> for Vec2 {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl Mul<f32> for Vec2 {
    type Output = Vec2;

    #[inline(always)]
    fn mul(self, factor: f32) -> Vec2 {
        Vec2 {
            x: self.x * factor,
            y: self.y * factor,
        }
    }
}

impl Mul<Vec2> for f32 {
    type Output = Vec2;

    #[inline(always)]
    fn mul(self, vec: Vec2) -> Vec2 {
        Vec2 {
            x: self * vec.x,
            y: self * vec.y,
        }
    }
}

impl Div<f32> for Vec2 {
    type Output = Vec2;

    #[inline(always)]
    fn div(self, factor: f32) -> Vec2 {
        Vec2 {
            x: self.x / factor,
            y: self.y / factor,
        }
    }
}

impl std::fmt::Debug for Vec2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{:.1} {:.1}]", self.x, self.y)
    }
}

impl Vec2 {
    pub fn new(x: f32, y: f32)->Self{
        Self{x, y}
    }
    pub const fn splat(v: f32)->Self{
        Self { x: v, y: v }
    }
    pub const X: Vec2 = Vec2 { x: 1.0, y: 0.0 };
    pub const Y: Vec2 = Vec2 { x: 0.0, y: 1.0 };

    pub const RIGHT: Vec2 = Vec2 { x: 1.0, y: 0.0 };
    pub const LEFT: Vec2 = Vec2 { x: -1.0, y: 0.0 };
    pub const UP: Vec2 = Vec2 { x: 0.0, y: -1.0 };
    pub const DOWN: Vec2 = Vec2 { x: 0.0, y: 1.0 };

    pub const ZERO: Self = Self { x: 0.0, y: 0.0 };
    pub const ONE: Self = Self { x: 1.0, y: 1.0 };
    pub const INFINITY: Self = Self::splat(f32::INFINITY);

    pub fn smaller_comp(&self)->f32{
        self.x.min(self.y)
    }
    pub fn sqr_distance(self, other: Vec2)->f32{
        f32::powi(self.x - other.x, 2)
        +f32::powi(self.y - other.y, 2)
    }
    pub fn distance(self, other: Vec2)->f32{
        self.sqr_distance(other).sqrt()
    }

}
impl Into<ffi::Vector2> for Vec2 {
    fn into(self) -> ffi::Vector2 {
        ffi::Vector2 { x: self.x, y: self.y }
    }
}
impl Into<Vec2> for ffi::Vector2 {
    fn into(self) -> Vec2 {
        Vec2 { x: self.x, y: self.y }
    }
}
pub fn vec2(x: impl Into<f32>, y: impl Into<f32>)->Vec2{
    Vec2 { x: x.into(), y: y.into() }
}
pub fn pos2(x: f32, y: f32)->Vec2{
    Vec2 { x, y }
}
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Rect{
    pub min: Vec2,
    pub max: Vec2,
}

impl Into<ffi::Rectangle> for Rect {
    fn into(self) -> ffi::Rectangle {
        ffi::Rectangle { x: self.min.x, y: self.min.y, width: self.width(), height: self.height() }
    }
}
impl Into<Rect> for ffi::Rectangle {
    fn into(self) -> Rect {
        Rect::from_min_size(Vec2{x: self.x, y: self.y}, Vec2{x: self.width, y: self.height})
    }
}

impl Rect{

    pub fn translate(self, vec: Vec2) -> Rect{
        Rect { min: self.min + vec, max: self.max + vec }
    }

    pub fn center(&self) -> Pos2 {
        Pos2 {
            x: (self.min.x + self.max.x) / 2.0,
            y: (self.min.y + self.max.y) / 2.0,
        }
    }

    pub fn new(x: f32, y: f32, width: f32, height: f32)->Self{
        Self { min: vec2(x, y), max: vec2(x+width, y+height) }
    }
    pub fn from_min_max(min: Pos2, max: Pos2) -> Self{
        Self { min, max }
    }
    pub fn from_center_size(center: Pos2, size: Vec2) -> Self{
        let hs = size * 0.5;
        Self { min: center - hs, max: center + hs }
    }
    #[must_use]
    pub fn shrink(self, amnt: f32) -> Self {
        self.shrink2(Vec2::splat(amnt))
    }

    #[must_use]
    pub fn shrink2(self, amnt: Vec2) -> Self {
        Rect::from_min_max(self.min + amnt, self.max - amnt)
    }
    #[inline(always)]
    pub fn from_min_size(min: Pos2, size: Vec2) -> Self {
        Rect {
            min,
            max: min + size,
        }
    }
    #[inline(always)]
    pub fn from_2_points(p1: Pos2, p2: Pos2) -> Self {
        Rect {
            min: pos2(f32::min(p1.x, p2.x), f32::min(p1.y, p2.y)),
            max: pos2(f32::max(p1.x, p2.x), f32::max(p1.y, p2.y)),
        }
    }
    #[inline(always)]
    pub fn size(&self) -> Vec2 {
        self.max - self.min
    }

    #[inline(always)]
    pub fn width(&self) -> f32 {
        self.max.x - self.min.x
    }

    #[inline(always)]
    pub fn height(&self) -> f32 {
        self.max.y - self.min.y
    }
}

pub struct Verts{
    rect: Rect,
    i: u8,
}
impl Rect{
    // calculate at what index the points would show up when calling verts() if they were part of a
    // Rect
    pub fn vert_inds(p1: Pos2, p2: Pos2)->(u8, u8){
        let left = p1.x <= p2.x;
        let top = p1.y <= p2.y;
        match (top, left){
            (true,  true)  => (0,2),
            (true,  false) => (1,3),
            (false, false) => (2,0),
            (false, true)  => (3,1),
        }
    }
    // vertices of rectangle starting at top left going clockwise
    pub fn verts(self)->Verts{
        Verts { rect: self, i: 0 }
    }
}
impl Iterator for Verts{
    type Item = Pos2;

    fn next(&mut self) -> Option<Self::Item> {
        let rect = self.rect;
        if self.i <= 4 {
            self.i += 1;
        }
        match self.i{
            1 => Some(rect.left_top()),
            2 => Some(rect.right_top()),
            3 => Some(rect.right_bottom()),
            4 => Some(rect.left_bottom()),
            _ => None,
        }
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
}
impl ExactSizeIterator for Verts {
    fn len(&self) -> usize {
        let len = 4 - self.i as usize;
        len
    }
}



impl Rect {

    pub fn contains(&self, pos: Pos2)->bool{
        let min = self.min;
        let max = self.max;

        min.x <= pos.x &&
        min.y <= pos.y &&
        pos.x <= max.x &&
        pos.y <= max.y

    }

    // change center keeping size
    pub fn align_center(self, center: Pos2)->Self{
        Self::from_center_size(center, self.size())
    }
    pub fn align_center_x(self, center_x: f32)->Self{
        self.align_center(pos2(center_x, self.center().x))
    }
    pub fn align_center_y(self, center_y: f32)->Self{
        self.align_center(pos2(self.center().x, center_y))
    }
    pub fn align_left(self, left: f32)->Self{
        let size = self.size();
        Self::new(left, self.min.y, size.x, size.y)
    }

    pub fn grow_to(self, point: Pos2)->Self{

        let min = pos2(self.min.x.min(point.x), self.min.y.min(point.y));
        let max = pos2(self.max.x.max(point.x), self.max.y.max(point.y));

        Self::from_min_max(min, max)
    }
    pub fn merge(self, other: Self)->Self{
        let min = pos2(self.min.x.min(other.min.x), self.min.y.min(other.min.y));
        let max = pos2(self.max.x.max(other.max.x), self.max.y.max(other.max.y));

        Self::from_min_max(min, max)
    }

    /// `min.x`
    #[inline(always)]
    pub fn left(&self) -> f32 {
        self.min.x
    }

    /// `min.x`
    #[inline(always)]
    pub fn left_mut(&mut self) -> &mut f32 {
        &mut self.min.x
    }

    /// `min.x`
    #[inline(always)]
    pub fn set_left(&mut self, x: f32) {
        self.min.x = x;
    }

    /// `max.x`
    #[inline(always)]
    pub fn right(&self) -> f32 {
        self.max.x
    }

    /// `max.x`
    #[inline(always)]
    pub fn right_mut(&mut self) -> &mut f32 {
        &mut self.max.x
    }

    /// `max.x`
    #[inline(always)]
    pub fn set_right(&mut self, x: f32) {
        self.max.x = x;
    }

    /// `min.y`
    #[inline(always)]
    pub fn top(&self) -> f32 {
        self.min.y
    }

    /// `min.y`
    #[inline(always)]
    pub fn top_mut(&mut self) -> &mut f32 {
        &mut self.min.y
    }

    /// `min.y`
    #[inline(always)]
    pub fn set_top(&mut self, y: f32) {
        self.min.y = y;
    }

    /// `max.y`
    #[inline(always)]
    pub fn bottom(&self) -> f32 {
        self.max.y
    }

    /// `max.y`
    #[inline(always)]
    pub fn bottom_mut(&mut self) -> &mut f32 {
        &mut self.max.y
    }

    /// `max.y`
    #[inline(always)]
    pub fn set_bottom(&mut self, y: f32) {
        self.max.y = y;
    }

    #[inline(always)]
    pub fn left_top(&self) -> Pos2 {
        pos2(self.left(), self.top())
    }

    #[inline(always)]
    pub fn center_top(&self) -> Pos2 {
        pos2(self.center().x, self.top())
    }

    #[inline(always)]
    pub fn right_top(&self) -> Pos2 {
        pos2(self.right(), self.top())
    }

    #[inline(always)]
    pub fn left_center(&self) -> Pos2 {
        pos2(self.left(), self.center().y)
    }

    #[inline(always)]
    pub fn right_center(&self) -> Pos2 {
        pos2(self.right(), self.center().y)
    }

    #[inline(always)]
    pub fn left_bottom(&self) -> Pos2 {
        pos2(self.left(), self.bottom())
    }

    #[inline(always)]
    pub fn center_bottom(&self) -> Pos2 {
        pos2(self.center().x, self.bottom())
    }

    #[inline(always)]
    pub fn right_bottom(&self) -> Pos2 {
        pos2(self.right(), self.bottom())
    }

    ///// Split rectangle in left and right halves. `t` is expected to be in the (0,1) range.
    //pub fn split_left_right_at_fraction(&self, t: f32) -> (Rect, Rect) {
    //    self.split_left_right_at_x(lerp(self.min.x..=self.max.x, t))
    //}

    /// Split rectangle in left and right halves at the given `x` coordinate.
    pub fn split_left_right_at_x(&self, split_x: f32) -> (Rect, Rect) {
        let left = Rect::from_min_max(self.min, Pos2::new(split_x, self.max.y));
        let right = Rect::from_min_max(Pos2::new(split_x, self.min.y), self.max);
        (left, right)
    }

    ///// Split rectangle in top and bottom halves. `t` is expected to be in the (0,1) range.
    //pub fn split_top_bottom_at_fraction(&self, t: f32) -> (Rect, Rect) {
    //    self.split_top_bottom_at_y(lerp(self.min.y..=self.max.y, t))
    //}

    /// Split rectangle in top and bottom halves at the given `y` coordinate.
    pub fn split_top_bottom_at_y(&self, split_y: f32) -> (Rect, Rect) {
        let top = Rect::from_min_max(self.min, Pos2::new(self.max.x, split_y));
        let bottom = Rect::from_min_max(Pos2::new(self.min.x, split_y), self.max);
        (top, bottom)
    }
}
