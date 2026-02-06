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
            top: 1f32,
            right: 1f32,
            bottom: 0f32,
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

    pub fn pos(&self) -> (f32, f32) {
        let x = self.0.params.pos.x;
        let y = self.0.params.pos.y;
        (x, y)
    }
    pub fn set_pos(&mut self, (x, y): (f32, f32)) {
        self.0.params.pos.x = x;
        self.0.params.pos.y = y;
    }
    pub fn pos_mut(&mut self) -> (&mut f32, &mut f32) {
        (&mut self.0.params.pos.x, &mut self.0.params.pos.y)
    }

    pub fn size(&self) -> (f32, f32) {
        let w = self.0.params.pos.w;
        let h = self.0.params.pos.h;
        (w, h)
    }
    pub fn set_size(&mut self, (w, h): (f32, f32)) {
        self.0.params.pos.w = w;
        self.0.params.pos.h = h;
    }
    pub fn size_mut(&mut self) -> (&mut f32, &mut f32) {
        (&mut self.0.params.pos.w, &mut self.0.params.pos.h)
    }

    pub fn angle(&self) -> f32 {
        self.0.params.angle
    }
    pub fn set_angle(&mut self, angle: f32) {
        self.0.params.angle = angle;
    }
    pub fn angle_mut(&mut self) -> &mut f32 {
        &mut self.0.params.angle
    }

    pub fn depth(&self) -> f32 {
        self.0.params.depth
    }
    pub fn set_depth(&mut self, depth: f32) {
        self.0.params.depth = depth;
    }
    pub fn depth_mut(&mut self) -> &mut f32 {
        &mut self.0.params.depth
    }

    pub fn centre(&self) -> (f32, f32) {
        let C2D_DrawParams__bindgen_ty_2 { x, y } = self.0.params.center;
        (x, y)
    }
    pub fn set_centre(&mut self, (x, y): (f32, f32)) {
        self.0.params.center.x = x;
        self.0.params.center.y = y;
    }
    pub fn centre_mut(&mut self) -> (&mut f32, &mut f32) {
        (&mut self.0.params.center.x, &mut self.0.params.center.y)
    }

    pub fn center(&self) -> (f32, f32) {
        self.centre()
    }
    pub fn set_center(&mut self, center: (f32, f32)) {
        self.set_centre(center)
    }
    pub fn center_mut(&mut self) -> (&mut f32, &mut f32) {
        self.centre_mut()
    }

    pub fn texture(&self) -> Option<&Tex> {
        unsafe { std::mem::transmute::<*mut citro3d_sys::C3D_Tex, Option<&Tex>>(self.0.image.tex) }
    }
    pub fn texture_mut(&mut self) -> Option<&mut Tex> {
        unsafe {
            std::mem::transmute::<*mut citro3d_sys::C3D_Tex, Option<&mut Tex>>(self.0.image.tex)
        }
    }

    pub fn set_mirroring(&mut self, mirroring: Mirroring) {
        let subtex = unsafe { &mut *(self.0.image.subtex as *mut citro2d_sys::Tex3DS_SubTexture) };
        match mirroring {
            Mirroring::Normal => {
                subtex.left = 0.;
                subtex.top = 1.;
                subtex.right = 1.;
                subtex.bottom = 0.;
            }
            Mirroring::MirrorX => {
                subtex.left = 1.;
                subtex.top = 1.;
                subtex.right = 0.;
                subtex.bottom = 0.;
            }
            Mirroring::MirrorY => {
                subtex.left = 0.;
                subtex.top = 0.;
                subtex.right = 1.;
                subtex.bottom = 1.;
            }
            Mirroring::MirrorXY => {
                subtex.left = 1.;
                subtex.top = 0.;
                subtex.right = 0.;
                subtex.bottom = 1.;
            }
        }
    }
}

pub enum Mirroring {
    Normal,
    MirrorX,
    MirrorY,
    MirrorXY,
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
