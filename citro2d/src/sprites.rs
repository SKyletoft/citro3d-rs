use std::{marker::PhantomData, rc::Rc};

use citro2d_sys::{
    C2D_DrawParams, C2D_DrawParams__bindgen_ty_1, C2D_DrawParams__bindgen_ty_2, C2D_DrawSprite,
    C2D_Image, C2D_Sprite, Tex3DS_SubTexture,
};
use citro3d::texture::ColourFormat;

use crate::{
    shapes::Shape,
    sprite_alloc::{TexAlloc, TexCloneAlloc, TexMutAlloc, TexOwnedAlloc, TexReclaimAlloc},
    texture::Tex,
};

#[doc(alias = "C2D_Sprite")]
#[repr(transparent)]
pub struct Sprite<A: TexAlloc = Rc<Tex>>(pub(crate) C2D_Sprite, PhantomData<A>);

impl<A: TexOwnedAlloc> Default for Sprite<A> {
    fn default() -> Self {
        Self::new()
    }
}

impl<A: TexOwnedAlloc> Sprite<A> {
    pub fn new() -> Self {
        Self::from_tex(Tex::new(32, 32, ColourFormat::Rgb565))
    }

    pub fn from_tex(tex: Tex) -> Self {
        Self::from_shared_tex(A::from_tex(tex))
    }
}

impl<A: TexAlloc> Sprite<A> {
    pub fn from_shared_tex(tex: A) -> Self {
        let width = unsafe { tex.as_tex().0.__bindgen_anon_2.__bindgen_anon_1.width } as f32;
        let height = unsafe { tex.as_tex().0.__bindgen_anon_2.__bindgen_anon_1.height } as f32;

        let tex = A::alloc(tex);
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
        Self(inner, PhantomData)
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

    pub fn texture_ref(&self) -> &Tex {
        debug_assert!(!self.0.image.tex.is_null());
        unsafe { &*(self.0.image.tex as *const Tex) }
    }

    pub fn set_mirroring(&mut self, mirroring: &Mirroring) {
        let subtex = unsafe { &mut *(self.0.image.subtex as *mut citro2d_sys::Tex3DS_SubTexture) };
        let (left, top, right, bottom) = mirroring.into();
        subtex.left = left;
        subtex.top = top;
        subtex.right = right;
        subtex.bottom = bottom;
    }

    pub fn with_mirroring(mut self, mirroring: &Mirroring) -> Self {
        self.set_mirroring(mirroring);
        self
    }
}

impl<A: TexCloneAlloc> Sprite<A> {
    pub fn texture(&self) -> A {
        unsafe { A::clone(self.0.image.tex) }
    }
}

impl<A: TexMutAlloc> Sprite<A> {
    pub fn texture_mut(&mut self) -> Option<&mut Tex> {
        debug_assert!(!self.0.image.tex.is_null());
        unsafe { A::texture_mut(self.0.image.tex) }
    }
}

impl<A: TexReclaimAlloc> Sprite<A> {
    pub fn destruct(self) -> (A, Box<Tex3DS_SubTexture>) {
        let C2D_Image { tex, subtex } = self.0.image;
        std::mem::forget(self);
        debug_assert!(!tex.is_null());
        debug_assert!(!subtex.is_null());
        unsafe {
            (
                A::from_raw(tex),
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

impl From<&Mirroring> for (f32, f32, f32, f32) {
    fn from(mirroring: &Mirroring) -> Self {
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
            } => (*left, *top, *right, *bottom),
        }
    }
}

impl Mirroring {
    pub const fn top(&self) -> f32 {
        match self {
            Mirroring::Normal | Mirroring::MirrorX => 1.,
            Mirroring::MirrorY | Mirroring::MirrorXY => 0.,
            Mirroring::Custom { top, .. } => *top,
        }
    }

    pub fn bottom(&self) -> f32 {
        match self {
            Mirroring::Normal | Mirroring::MirrorX => 0.,
            Mirroring::MirrorY | Mirroring::MirrorXY => 1.,
            Mirroring::Custom { bottom, .. } => *bottom,
        }
    }

    pub fn left(&self) -> f32 {
        match self {
            Mirroring::Normal | Mirroring::MirrorY => 0.,
            Mirroring::MirrorX | Mirroring::MirrorXY => 1.,
            Mirroring::Custom { left, .. } => *left,
        }
    }

    pub fn right(&self) -> f32 {
        match self {
            Mirroring::Normal | Mirroring::MirrorY => 1.,
            Mirroring::MirrorX | Mirroring::MirrorXY => 0.,
            Mirroring::Custom { right, .. } => *right,
        }
    }
}

impl<A: TexAlloc> Shape for Sprite<A> {
    #[doc(alias = "C2D_DrawSprite")]
    fn render(&self) -> bool {
        unsafe { C2D_DrawSprite(&raw const self.0 as *mut C2D_Sprite) }
    }
}

impl<A: TexAlloc> Drop for Sprite<A> {
    fn drop(&mut self) {
        let C2D_Sprite {
            image: C2D_Image { tex, subtex },
            ..
        } = self.0;
        debug_assert!(!tex.is_null());
        debug_assert!(!subtex.is_null());
        unsafe {
            A::free(tex);
            let _ = Box::from_raw(subtex as *mut Tex3DS_SubTexture);
        }
    }
}
