#![allow(clippy::missing_safety_doc)]

use std::rc::Rc;

use crate::texture::Tex;

pub unsafe trait TexAlloc: Sized {
    fn as_tex(&self) -> &Tex;
    fn alloc(tex: Self) -> *mut citro3d_sys::C3D_Tex;
    unsafe fn free(tex: *mut citro3d_sys::C3D_Tex) {
        let _ = unsafe { Box::from_raw(tex as *mut Tex) };
    }
}

pub trait TexOwnedAlloc: TexAlloc {
    fn from_tex(tex: Tex) -> Self;
}

pub unsafe trait TexReclaimAlloc: TexAlloc {
    unsafe fn from_raw(tex: *mut citro3d_sys::C3D_Tex) -> Self;
}

pub unsafe trait TexCloneAlloc: TexAlloc {
    unsafe fn clone(tex: *mut citro3d_sys::C3D_Tex) -> Self;
}

pub unsafe trait TexMutAlloc: TexAlloc {
    unsafe fn texture_mut<'a>(tex: *mut citro3d_sys::C3D_Tex) -> Option<&'a mut Tex>;
}

unsafe impl TexAlloc for Rc<Tex> {
    fn as_tex(&self) -> &Tex {
        self
    }

    fn alloc(tex: Self) -> *mut citro3d_sys::C3D_Tex {
        Rc::into_raw(tex) as *mut citro3d_sys::C3D_Tex
    }

    unsafe fn free(tex: *mut citro3d_sys::C3D_Tex) {
        let _ = unsafe { Rc::from_raw(tex as *const Tex) };
    }
}

impl TexOwnedAlloc for Rc<Tex> {
    fn from_tex(tex: Tex) -> Self {
        Rc::new(tex)
    }
}

unsafe impl TexReclaimAlloc for Rc<Tex> {
    unsafe fn from_raw(tex: *mut citro3d_sys::C3D_Tex) -> Self {
        unsafe { Rc::from_raw(tex as *const Tex) }
    }
}

unsafe impl TexCloneAlloc for Rc<Tex> {
    unsafe fn clone(tex: *mut citro3d_sys::C3D_Tex) -> Self {
        let rc = unsafe { Rc::from_raw(tex as *const Tex) };
        let ret = rc.clone();
        std::mem::forget(rc);
        ret
    }
}

unsafe impl TexMutAlloc for Rc<Tex> {
    unsafe fn texture_mut<'a>(tex: *mut citro3d_sys::C3D_Tex) -> Option<&'a mut Tex> {
        let rc = unsafe { Rc::from_raw(tex as *const Tex) };
        let can_mutate = Rc::strong_count(&rc) == 1 && Rc::weak_count(&rc) == 0;
        std::mem::forget(rc);

        if can_mutate {
            unsafe { Some(&mut *(tex as *mut Tex)) }
        } else {
            None
        }
    }
}

unsafe impl TexAlloc for Box<Tex> {
    fn as_tex(&self) -> &Tex {
        self
    }

    fn alloc(tex: Self) -> *mut citro3d_sys::C3D_Tex {
        Box::into_raw(tex) as *mut citro3d_sys::C3D_Tex
    }
}

impl TexOwnedAlloc for Box<Tex> {
    fn from_tex(tex: Tex) -> Self {
        Box::new(tex)
    }
}

unsafe impl TexReclaimAlloc for Box<Tex> {
    unsafe fn from_raw(tex: *mut citro3d_sys::C3D_Tex) -> Self {
        unsafe { Box::from_raw(tex as *mut Tex) }
    }
}

unsafe impl TexMutAlloc for Box<Tex> {
    unsafe fn texture_mut<'a>(tex: *mut citro3d_sys::C3D_Tex) -> Option<&'a mut Tex> {
        unsafe { Some(&mut *(tex as *mut Tex)) }
    }
}

unsafe impl TexAlloc for &Tex {
    fn as_tex(&self) -> &Tex {
        self
    }

    fn alloc(tex: Self) -> *mut citro3d_sys::C3D_Tex {
        tex as *const Tex as *mut citro3d_sys::C3D_Tex
    }

    unsafe fn free(_tex: *mut citro3d_sys::C3D_Tex) {}
}

unsafe impl TexReclaimAlloc for &Tex {
    unsafe fn from_raw(tex: *mut citro3d_sys::C3D_Tex) -> Self {
        unsafe { &*(tex as *const Tex) }
    }
}

unsafe impl TexCloneAlloc for &Tex {
    unsafe fn clone(tex: *mut citro3d_sys::C3D_Tex) -> Self {
        unsafe { &*(tex as *const Tex) }
    }
}

unsafe impl TexMutAlloc for &Tex {
    unsafe fn texture_mut<'a>(_tex: *mut citro3d_sys::C3D_Tex) -> Option<&'a mut Tex> {
        None
    }
}

unsafe impl TexAlloc for &mut Tex {
    fn as_tex(&self) -> &Tex {
        self
    }

    fn alloc(tex: Self) -> *mut citro3d_sys::C3D_Tex {
        tex as *mut Tex as *mut citro3d_sys::C3D_Tex
    }

    unsafe fn free(_tex: *mut citro3d_sys::C3D_Tex) {}
}

unsafe impl TexReclaimAlloc for &mut Tex {
    unsafe fn from_raw(tex: *mut citro3d_sys::C3D_Tex) -> Self {
        unsafe { &mut *(tex as *mut Tex) }
    }
}

unsafe impl TexMutAlloc for &mut Tex {
    unsafe fn texture_mut<'a>(tex: *mut citro3d_sys::C3D_Tex) -> Option<&'a mut Tex> {
        unsafe { Some(&mut *(tex as *mut Tex)) }
    }
}
