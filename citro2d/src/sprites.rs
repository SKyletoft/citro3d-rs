use std::rc::Rc;

use citro2d_sys::{
    C2D_DrawParams, C2D_DrawParams__bindgen_ty_1, C2D_DrawParams__bindgen_ty_2, C2D_DrawSprite,
    C2D_Image, C2D_Sprite, Tex3DS_SubTexture,
};
use citro3d::texture::ColourFormat;

use crate::{shapes::Shape, texture::Tex};

#[doc(alias = "C2D_Sprite")]
#[repr(transparent)]
pub struct Sprite(pub(crate) C2D_Sprite);

impl Default for Sprite {
    fn default() -> Self {
        Self::new()
    }
}

impl Sprite {
    pub fn new() -> Self {
        Sprite::from_tex(Tex::new(32, 32, ColourFormat::Rgb565))
    }

    pub fn from_tex(tex: Tex) -> Self {
        let width = unsafe { tex.0.__bindgen_anon_2.__bindgen_anon_1.width } as f32;
        let height = unsafe { tex.0.__bindgen_anon_2.__bindgen_anon_1.height } as f32;

        let tex = Rc::into_raw(Rc::new(tex.0)) as *mut citro3d_sys::C3D_Tex;
        debug_assert!(!tex.is_null());
        let subtex = Box::leak(Box::new(Tex3DS_SubTexture {
            width: width as u16,
            height: height as u16,
            left: 0f32,
            top: 1f32,
            right: 1f32,
            bottom: 0f32,
        })) as *mut Tex3DS_SubTexture;
        debug_assert!(!subtex.is_null());

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
    pub fn with_pos(mut self, pos: (f32, f32)) -> Self {
        self.set_pos(pos);
        self
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
    pub fn with_size(mut self, size: (f32, f32)) -> Self {
        self.set_size(size);
        self
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
    pub fn with_angle(mut self, angle: f32) -> Self {
        self.set_angle(angle);
        self
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
    pub fn with_depth(mut self, depth: f32) -> Self {
        self.set_depth(depth);
        self
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

    pub fn with_centre(mut self, centre: (f32, f32)) -> Self {
        self.set_centre(centre);
        self
    }

    pub fn center(&self) -> (f32, f32) {
        self.centre()
    }
    pub fn set_center(&mut self, center: (f32, f32)) {
        self.set_centre(center)
    }
    pub fn with_center(mut self, center: (f32, f32)) -> Self {
        self.set_center(center);
        self
    }
    pub fn center_mut(&mut self) -> (&mut f32, &mut f32) {
        self.centre_mut()
    }

    pub fn texture(&self) -> &Tex {
        debug_assert!(!self.0.image.tex.is_null());
        unsafe { &*(self.0.image.tex as *const Tex) }
    }
    pub fn texture_mut(&mut self) -> Option<&mut Tex> {
        debug_assert!(!self.0.image.tex.is_null());
        let rc = unsafe { Rc::from_raw(self.0.image.tex as *const Tex) };
        let can_mutate = Rc::strong_count(&rc) == 1 && Rc::weak_count(&rc) == 0;
        std::mem::forget(rc);
        if can_mutate {
            unsafe { Some(&mut *(self.0.image.tex as *mut Tex)) }
        } else {
            None
        }
    }

    pub fn set_mirroring(&mut self, mirroring: Mirroring) {
        let subtex = unsafe { &mut *(self.0.image.subtex as *mut citro2d_sys::Tex3DS_SubTexture) };
        let (left, top, right, bottom) = mirroring.into();
        subtex.left = left;
        subtex.top = top;
        subtex.right = right;
        subtex.bottom = bottom;
    }

    pub fn with_mirroring(mut self, mirroring: Mirroring) -> Self {
        self.set_mirroring(mirroring);
        self
    }

    pub fn destruct(self) -> (Rc<Tex>, Box<Tex3DS_SubTexture>) {
        let C2D_Sprite {
            image: C2D_Image { tex, subtex },
            ..
        } = self.0;
        debug_assert!(!tex.is_null());
        debug_assert!(!subtex.is_null());
        unsafe {
            (
                Rc::from_raw(tex as *const Tex),
                Box::from_raw(subtex as *mut Tex3DS_SubTexture),
            )
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Mirroring {
    Normal,
    MirrorX,
    MirrorY,
    MirrorXY,
    Custom {
        left: f32,
        top: f32,
        right: f32,
        bottom: f32,
    },
}

impl From<Mirroring> for (f32, f32, f32, f32) {
    fn from(mirroring: Mirroring) -> Self {
        match mirroring {
            Mirroring::Normal => (0., 1., 1., 0.),
            Mirroring::MirrorX => (1., 1., 0., 0.),
            Mirroring::MirrorY => (0., 0., 1., 1.),
            Mirroring::MirrorXY => (1., 0., 0., 1.),
            Mirroring::Custom {
                left,
                top,
                right,
                bottom,
            } => (left, top, right, bottom),
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
        debug_assert!(!tex.is_null());
        debug_assert!(!subtex.is_null());
        unsafe {
            let _ = Rc::from_raw(tex);
            let _ = Box::from_raw(subtex as *mut Tex3DS_SubTexture);
        }
    }
}
