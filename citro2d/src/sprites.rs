use citro2d_sys::{
    C2D_DrawParams, C2D_DrawParams__bindgen_ty_1, C2D_DrawParams__bindgen_ty_2, C2D_DrawSprite,
    C2D_Image, C2D_Sprite, Tex3DS_SubTexture,
};
use citro3d::texture::ColorFormat;

use crate::{shapes::Shape, texture::Tex};

#[doc(alias = "C2D_Sprite")]
#[repr(transparent)]
pub struct Sprite(pub(crate) C2D_Sprite);

impl Sprite {
    pub fn new() -> Self {
        Sprite::from_tex(Tex::new(32, 32, ColorFormat::Rgb565))
    }

    pub fn from_tex(tex: Tex) -> Self {
        let width = unsafe { tex.0.__bindgen_anon_2.__bindgen_anon_1.width } as f32;
        let height = unsafe { tex.0.__bindgen_anon_2.__bindgen_anon_1.height } as f32;

        let tex = Box::leak(Box::new(tex.0)) as *mut citro3d_sys::C3D_Tex;
        let subtex = Box::leak(Box::new(Tex3DS_SubTexture {
            width: width as u16,
            height: height as u16,
            left: 0f32,
            top: 0f32,
            right: 1f32,
            bottom: 1f32,
        })) as *mut Tex3DS_SubTexture;

        let c2d_image = C2D_Image { tex, subtex };

        let c2d_drawparams = C2D_DrawParams {
            pos: C2D_DrawParams__bindgen_ty_1 {
                x: 50.,
                y: 50.,
                w: 32. * 4.,
                h: 32. * 4.,
            },
            center: C2D_DrawParams__bindgen_ty_2 { x: 0., y: 0. },
            depth: 1.,
            angle: 0.,
        };
        let inner = C2D_Sprite {
            image: c2d_image,
            params: c2d_drawparams,
        };
        Self(inner)
    }

    pub fn texture_mut(&mut self) -> Option<&mut Tex> {
        unsafe {
            std::mem::transmute::<*mut citro3d_sys::C3D_Tex, Option<&mut Tex>>(self.0.image.tex)
        }
    }
}

impl Shape for Sprite {
    #[doc(alias = "C2D_DrawSprite")]
    fn render(&self) -> bool {
        unsafe { C2D_DrawSprite(&raw const self.0 as *mut C2D_Sprite) }
    }
}

impl Drop for Sprite {
    fn drop(&mut self) {
        let C2D_Sprite {
            image: C2D_Image { tex, subtex },
            ..
        } = self.0;
        unsafe {
            let _ = Box::from_raw(tex as *mut Tex);
            let _ = Box::from_raw(subtex as *mut Tex3DS_SubTexture);
        }
    }
}
