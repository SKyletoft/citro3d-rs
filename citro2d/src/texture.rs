use crate::{
    Instance,
    render::{Color, Target},
    shapes::Shape,
};
use citro2d_sys::{
    C2D_DrawParams, C2D_DrawParams__bindgen_ty_1, C2D_DrawParams__bindgen_ty_2, C2D_DrawSprite,
    C2D_Image, C2D_Sprite,
};
use citro3d::texture::ColorFormat;
use citro3d_sys::{C3D_Tex, C3D_TexInit};

#[doc(alias = "C3D_Tex")]
#[repr(transparent)]
pub struct Tex(pub(crate) C3D_Tex);

impl Tex {
    #[doc(alias = "C3D_TexInit")]
    pub fn new(width: u16, height: u16, format: ColorFormat) -> Self {
        let mut texture = std::mem::MaybeUninit::<citro3d_sys::C3D_Tex>::uninit();
        let init_success = unsafe { C3D_TexInit(texture.as_mut_ptr(), width, height, format as _) };
        assert!(init_success);
        let mut c3d_tex = unsafe { texture.assume_init() };
        Self(c3d_tex)
    }

    pub fn upload(&mut self, texture: &[u8]) {
        let h = unsafe { self.0.__bindgen_anon_2.__bindgen_anon_1.height as usize };
        let w = unsafe { self.0.__bindgen_anon_2.__bindgen_anon_1.width as usize };
        let fmt = self.0._bitfield_1.get(0, 4) as u8;
        let fmt = ColorFormat::try_from(fmt).unwrap();
        debug_assert_eq!(texture.len(), h * w * bytes_per_pixel(fmt));

        unsafe {
            citro3d_sys::C3D_TexUpload(&raw mut self.0, texture.as_ptr() as *const std::ffi::c_void)
        };
    }
}

fn bytes_per_pixel(fmt: ColorFormat) -> usize {
    use ColorFormat::*;
    match fmt {
        Rgba8 => 4,
        Rgb8 | Rgba5551 | Rgb565 => 3,
        _ => todo!(),
    }
}
