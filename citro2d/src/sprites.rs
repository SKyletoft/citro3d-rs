use citro2d_sys::{
    C2D_DrawParams, C2D_DrawParams__bindgen_ty_1, C2D_DrawParams__bindgen_ty_2, C2D_DrawSprite,
    C2D_Image, C2D_Sprite, Tex3DS_SubTexture,
};
use citro3d::texture::ColorFormat;

use crate::{
    Instance,
    render::{Color, Target},
    shapes::Shape,
    texture::Tex,
};

#[doc(alias = "C2D_Sprite")]
#[repr(transparent)]
pub struct Sprite(pub(crate) C2D_Sprite);

impl Sprite {
    pub fn new() -> Self {
        let mut texture = Tex::new(32, 32, ColorFormat::Rgb565);

        let mut tex3ds_subtexture = Tex3DS_SubTexture {
            width: 32,
            height: 32,

            // What is the coordinate space for these?
            left: 0f32,
            top: 0f32,
            right: 1f32,
            bottom: 1f32,
        };
        let mut c2d_image = C2D_Image {
            tex: &raw mut texture.0,
            subtex: &raw mut tex3ds_subtexture,
        };
        let mut c2d_drawparams = C2D_DrawParams {
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
        unsafe { C2D_DrawSprite(&raw const self.0 as *mut _) }
    }
}
