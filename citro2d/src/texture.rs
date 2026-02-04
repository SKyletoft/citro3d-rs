use citro3d::texture::ColourFormat;
use citro3d_sys::{C3D_Tex, C3D_TexInit};

use crate::{
    Instance,
    render::{Colour, Target},
    shapes::Shape,
};

#[doc(alias = "C3D_Tex")]
#[repr(transparent)]
pub struct Tex(pub(crate) C3D_Tex);

impl Tex {
    #[doc(alias = "C3D_TexInit")]
    pub fn new(width: u16, height: u16, format: ColourFormat) -> Self {
        let mut texture = std::mem::MaybeUninit::<citro3d_sys::C3D_Tex>::uninit();
        let init_success = unsafe { C3D_TexInit(texture.as_mut_ptr(), width, height, format as _) };
        assert!(init_success);
        let mut c3d_tex = unsafe { texture.assume_init() };
        Self(c3d_tex)
    }

    #[doc(alias = "C3D_TexUpload")]
    pub fn upload<T, const M: usize, const N: usize>(&mut self, mut texture: [[T; M]; N])
    where
        Assert<{ M % 8 == 0 }>: IsTrue,
        Assert<{ N % 8 == 0 }>: IsTrue,
    {
        let h = unsafe { self.0.__bindgen_anon_2.__bindgen_anon_1.height as usize };
        let w = unsafe { self.0.__bindgen_anon_2.__bindgen_anon_1.width as usize };
        let fmt = self.0._bitfield_1.get(0, 4) as u8;
        let fmt = ColourFormat::try_from(fmt).unwrap();

        swizzle(&mut texture);
        unsafe {
            citro3d_sys::C3D_TexUpload(&raw mut self.0, texture.as_ptr() as *const std::ffi::c_void)
        };
    }

    pub fn upload_swizzled(&mut self, texture: &[u8]) {
        let h = unsafe { self.0.__bindgen_anon_2.__bindgen_anon_1.height as usize };
        let w = unsafe { self.0.__bindgen_anon_2.__bindgen_anon_1.width as usize };
        let fmt = self.0._bitfield_1.get(0, 4) as u8;
        let fmt = ColourFormat::try_from(fmt).unwrap();
        debug_assert_eq!(texture.len(), h * w * bytes_per_pixel(fmt));

        unsafe {
            citro3d_sys::C3D_TexUpload(&raw mut self.0, texture.as_ptr() as *const std::ffi::c_void)
        };
    }
}

fn bytes_per_pixel(fmt: ColourFormat) -> usize {
    match fmt {
        ColourFormat::Rgba8 => 4,
        ColourFormat::Rgb8 => 3,
        ColourFormat::Rgba5551 | ColourFormat::Rgb565 => 2,
        _ => todo!(),
    }
}

fn swizzle<T, const M: usize, const N: usize>(data: &mut [[T; M]; N])
where
    Assert<{ M % 8 == 0 }>: IsTrue,
    Assert<{ N % 8 == 0 }>: IsTrue,
{
    for y in 0..M {
        for x in 0..N {
            // swizzle_block([x, x + 1, x + 2, x + 3], data);
        }
    }
}

fn swizzle_block<T>(entries: [usize; 8 * 8], data: &mut [T]) {}

pub enum Assert<const CHECK: bool> {}
pub trait IsTrue {}
impl IsTrue for Assert<true> {}
