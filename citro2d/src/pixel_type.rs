use bitfields::bitfield;

pub trait PixelType : Sized + Default + Copy {}

use crate::render::Colour as Rgba8;
impl PixelType for Rgba8 {}

#[repr(C)]
#[derive(Copy, Clone, Default)]
pub struct Bgr8 {
    blue: u8,
    green: u8,
    red: u8,
}
impl PixelType for Bgr8 {}

#[derive(Copy, Clone)]
#[bitfield(u16, order = msb)]
pub struct Rgba5551 {
    #[bits(5)]
    red: u8,
    #[bits(5)]
    green: u8,
    #[bits(5)]
    blue: u8,
    alpha: bool,
}
impl PixelType for Rgba5551 {}

impl Rgba5551 {
    pub const TRANSPARENT: Rgba5551 = Rgba5551::from_bits(0b00000_00000_00000_1);
}

#[derive(Copy, Clone)]
#[bitfield(u16, order = msb)]
pub struct Rgb565 {
    #[bits(5)]
    red: u8,
    #[bits(6)]
    green: u8,
    #[bits(5)]
    blue: u8,
}
impl PixelType for Rgb565 {}
