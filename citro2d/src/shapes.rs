//! Safe bindings to shapes supported by citro2d
use crate::{Point, Size, render::Colour};

/// Holds information for rendering multi coloured shapes
/// most shapes have a 'solid'
pub struct MultiColor {
    pub top_left: Colour,
    pub top_right: Colour,
    pub bottom_left: Colour,
    pub bottom_right: Colour,
}
pub use MultiColor as MultiColour;

/// A trait to help render all 2D shapes supported by citro2d
pub trait Shape {
    //TODO possibly return Option<self>.
    fn render(&self) -> bool;
}

/// Holds information for rendering a C2D_DrawRectangle
pub struct Rectangle {
    pub point: Point,
    pub size: Size,
    pub multi_colour: MultiColour,
}

impl Shape for Rectangle {
    /// Draws a multi colour rectangle
    #[doc(alias = "C2D_DrawRectangle")]
    fn render(&self) -> bool {
        unsafe {
            citro2d_sys::C2D_DrawRectangle(
                self.point.x,
                self.point.y,
                self.point.z,
                self.size.width,
                self.size.height,
                self.multi_colour.top_left.into(),
                self.multi_colour.top_right.into(),
                self.multi_colour.bottom_left.into(),
                self.multi_colour.bottom_right.into(),
            )
        }
    }
}

/// Holds the information needed to draw a solid colour Rectangle
pub struct RectangleSolid {
    pub point: Point,
    pub size: Size,
    pub colour: Colour,
}

impl Shape for RectangleSolid {
    /// Draws a single coloured Rectangle
    #[doc(alias = "C2D_DrawRectSolid")]
    fn render(&self) -> bool {
        unsafe {
            citro2d_sys::C2D_DrawRectSolid(
                self.point.x,
                self.point.y,
                self.point.z,
                self.size.width,
                self.size.height,
                self.colour.into(),
            )
        }
    }
}

/// Holds the information needed to draw a solid colour Triangle
pub struct Triangle {
    pub top: Point,
    pub top_colour: Colour,
    pub left: Point,
    pub left_colour: Colour,
    pub right: Point,
    pub right_colour: Colour,
    pub depth: f32,
}

impl Shape for Triangle {
    /// Draws a multi colour Triangle
    #[doc(alias = "C2D_DrawTriangle")]
    fn render(&self) -> bool {
        unsafe {
            citro2d_sys::C2D_DrawTriangle(
                self.top.x,
                self.top.y,
                self.top_colour.into(),
                self.left.x,
                self.left.y,
                self.left_colour.into(),
                self.right.x,
                self.right.y,
                self.right_colour.into(),
                self.depth,
            )
        }
    }
}

/// Holds the information needed to draw a Ellipse
pub struct Ellipse {
    pub point: Point,
    pub size: Size,
    pub multi_colour: MultiColour,
}

impl Shape for Ellipse {
    /// Draws a multi colour Ellipse
    #[doc(alias = "C2D_DrawEllipse")]
    fn render(&self) -> bool {
        unsafe {
            citro2d_sys::C2D_DrawEllipse(
                self.point.x,
                self.point.y,
                self.point.z,
                self.size.width,
                self.size.height,
                self.multi_colour.top_left.into(),
                self.multi_colour.top_right.into(),
                self.multi_colour.bottom_left.into(),
                self.multi_colour.bottom_right.into(),
            )
        }
    }
}

/// Holds the information needed to draw a solid colour Triangle
pub struct EllipseSolid {
    pub point: Point,
    pub size: Size,
    pub colour: Colour,
}

impl Shape for EllipseSolid {
    ///Draws a solid colour Ellipse
    #[doc(alias = "C2D_DrawEllipseSolid")]
    fn render(&self) -> bool {
        unsafe {
            citro2d_sys::C2D_DrawEllipseSolid(
                self.point.x,
                self.point.y,
                self.point.z,
                self.size.width,
                self.size.height,
                self.colour.into(),
            )
        }
    }
}
/// Holds the information needed to draw a multi coloured circle
pub struct Circle {
    pub point: Point,
    pub radius: f32,
    pub multi_colour: MultiColour,
}

impl Shape for Circle {
    /// Draws a multi colour Ellipse
    #[doc(alias = "C2D_DrawCircle")]
    fn render(&self) -> bool {
        unsafe {
            citro2d_sys::C2D_DrawCircle(
                self.point.x,
                self.point.y,
                self.point.z,
                self.radius,
                self.multi_colour.top_left.into(),
                self.multi_colour.top_right.into(),
                self.multi_colour.bottom_left.into(),
                self.multi_colour.bottom_right.into(),
            )
        }
    }
}

/// Holds the information needed to draw a solid colour Circle
pub struct CircleSolid {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub radius: f32,
    pub colour: Colour,
}

impl Shape for CircleSolid {
    /// Renders a solid Circle
    #[doc(alias = "C2D_DrawCircleSolid")]
    fn render(&self) -> bool {
        unsafe {
            citro2d_sys::C2D_DrawCircleSolid(
                self.x,
                self.y,
                self.z,
                self.radius,
                self.colour.into(),
            )
        }
    }
}

/// Holds the information needed to draw a solid colour Circle
pub struct Line {
    pub start: Point,
    pub end: Point,
    pub start_colour: Colour,
    pub end_colour: Colour,
    pub thickness: f32,
    pub depth: f32,
}

impl Shape for Line {
    /// Renders a line
    #[doc(alias = "C2D_DrawLine")]
    fn render(&self) -> bool {
        unsafe {
            citro2d_sys::C2D_DrawLine(
                self.start.x,
                self.start.y,
                self.start_colour.into(),
                self.end.x,
                self.end.y,
                self.end_colour.into(),
                self.thickness,
                self.depth,
            )
        }
    }
}
