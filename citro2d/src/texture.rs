pub use citro3d::texture::ColourFormat;

use crate::pixel_type::PixelType;

#[doc(alias = "C3D_Tex")]
#[repr(transparent)]
pub struct Tex(pub(crate) citro3d_sys::C3D_Tex);

impl Tex {
    pub fn size(&self) -> (u16, u16) {
        let dims = unsafe { self.0.__bindgen_anon_2.__bindgen_anon_1 };
        (dims.width, dims.height)
    }

    #[doc(alias = "C3D_TexInit")]
    pub fn new(width: u16, height: u16, format: ColourFormat) -> Self {
        let width = (width + 7) & !7;
        let height = (height + 7) & !7;
        let mut texture = std::mem::MaybeUninit::<citro3d_sys::C3D_Tex>::uninit();
        let init_success =
            unsafe { citro3d_sys::C3D_TexInit(texture.as_mut_ptr(), width, height, format as _) };
        assert!(init_success);
        let c3d_tex = unsafe { texture.assume_init() };
        Self(c3d_tex)
    }

    // TODO: When const generic expressions are stable O should be removed and replaced with M * N
    pub fn swizzle_and_upload<T: PixelType, const M: usize, const N: usize, const O: usize>(
        &mut self,
        texture: &[[T; M]; N],
    ) {
        let h = unsafe { self.0.__bindgen_anon_2.__bindgen_anon_1.height as usize };
        let w = unsafe { self.0.__bindgen_anon_2.__bindgen_anon_1.width as usize };
        let fmt = self.0._bitfield_1.get(0, 4) as u8;
        let fmt = ColourFormat::try_from(fmt).unwrap();
        debug_assert_eq!(M % 8, 0);
        debug_assert_eq!(N % 8, 0);
        debug_assert_eq!(h, M);
        debug_assert_eq!(w, N);
        debug_assert_eq!(size_of::<T>(), bytes_per_pixel(fmt));
        debug_assert_eq!(M * N, O);

        let mut texture = *texture;
        swizzle::<T, M, N, O>(&mut texture);

        unsafe {
            citro3d_sys::C3D_TexUpload(&raw mut self.0, texture.as_ptr() as *const std::ffi::c_void)
        };
    }

    #[doc(alias = "C3D_TexUpload")]
    pub fn upload_swizzled(&mut self, texture: &[u8]) {
        let _d = unsafe { self.0.__bindgen_anon_2.dim };
        let h = unsafe { self.0.__bindgen_anon_2.__bindgen_anon_1.height } as usize;
        let w = unsafe { self.0.__bindgen_anon_2.__bindgen_anon_1.width } as usize;
        let fmt = self.0._bitfield_1.get(0, 4) as u8;
        let fmt = ColourFormat::try_from(fmt).unwrap();
        debug_assert_eq!(texture.len(), h * w * bytes_per_pixel(fmt));
        debug_assert_eq!(h % 8, 0);
        debug_assert_eq!(w % 8, 0);

        unsafe {
            // citro3d_sys::C3D_TexUpload(&raw mut self.0, texture.as_ptr() as *const std::ffi::c_void)
            // citro3d_sys::C3D_TexLoadImage(
            //     &raw mut self.0,
            //     texture.as_ptr() as *const std::ffi::c_void,
            //     ctru_sys::GPU_TEXFACE_2D,
            //     0,
            // );
            // {
            //     let data = texture.as_ptr() as *const std::ffi::c_void;
            //     let mut size: u32 = 0;
            //     let out: *mut std::ffi::c_void = citro3d_sys::C3D_TexGetImagePtr(
            //         &raw mut self.0,
            //         // if dbg!(
            //         //     !(citro3d_sys::C3D_TexGetType(&raw mut self.0)
            //         //         == ctru_sys::GPU_TEX_CUBE_MAP
            //         //         || citro3d_sys::C3D_TexGetType(&raw mut self.0)
            //         //             == ctru_sys::GPU_TEX_SHADOW_CUBE)
            //         // ) {
            //         (*&raw mut self.0).__bindgen_anon_1.data,
            //         // } else {
            //         //     ((*(*&raw mut self.0).__bindgen_anon_1.cube).data)
            //         //         [ctru_sys::GPU_TEXFACE_2D as usize]
            //         // },
            //         0,
            //         &raw mut size,
            //     );

            //     if (!{
            //         let addr = out;
            //         let vaddr = addr as u32;
            //         vaddr >= ctru_sys::OS_VRAM_VADDR
            //             && vaddr < ctru_sys::OS_VRAM_VADDR + ctru_sys::OS_VRAM_SIZE
            //     }) {
            //         // memcpy(out, data, size);
            //         std::slice::from_raw_parts_mut(out as *mut u8, size as usize).copy_from_slice(
            //             std::slice::from_raw_parts(data as *mut u8, size as usize),
            //         );
            //     } else {
            //         citro3d_sys::C3D_SyncTextureCopy(
            //             data as *mut u32,
            //             0,
            //             out as *mut u32,
            //             0,
            //             size,
            //             8,
            //         );
            //     }
            // }
            {
                let tex_size = self.0._bitfield_1.get(4, 28) as u32;
                let size = citro3d_sys::C3D_TexCalcLevelSize(tex_size, 0);
                let out = self.0.__bindgen_anon_1.data;

                if addr_is_vram(out) {
                    let src =
                        std::slice::from_raw_parts(texture.as_ptr() as *mut u8, size as usize);
                    let dst = std::slice::from_raw_parts_mut(out as *mut u8, size as usize);
                    dst.copy_from_slice(src);
                } else {
                    citro3d_sys::C3D_SyncTextureCopy(
                        texture.as_ptr() as *mut u32,
                        0,
                        out as *mut u32,
                        0,
                        size,
                        8,
                    );
                }
            }
        };
    }

    pub fn swizzle_and_update_tile<T: PixelType>(
        &mut self,
        mut texture: [[T; 8]; 8],
        tile_x: u16,
        tile_y: u16,
    ) {
        swizzle::<T, 8, 8, 64>(&mut texture);
        self.update_tile(&texture, tile_x, tile_y);
    }

    pub fn update_tile<T: PixelType>(&mut self, texture: &[[T; 8]; 8], tile_x: u16, tile_y: u16) {
        let dst = self.raw_tile(tile_x, tile_y);
        *dst = *texture;
    }

    pub fn raw_tile<T: PixelType>(&mut self, tile_x: u16, tile_y: u16) -> &mut [[T; 8]; 8] {
        debug_assert!(!addr_is_vram(unsafe { self.0.__bindgen_anon_1.data }));

        unsafe {
            let actual_width = self.0.__bindgen_anon_2.__bindgen_anon_1.width;
            let raw_data_ptr =
                citro3d_sys::C3D_Tex2DGetImagePtr(&raw mut self.0, 0, std::ptr::null_mut())
                    .byte_add((tile_x + tile_y * actual_width / 8) as usize * 64 * size_of::<T>());
            &mut *(raw_data_ptr as *mut [[T; 8]; 8])
        }
    }

    pub fn raw_flat_tile<T: PixelType>(&mut self, tile_x: u16, tile_y: u16) -> &mut [T; 64] {
        debug_assert!(!addr_is_vram(unsafe { self.0.__bindgen_anon_1.data }));

        unsafe {
            let actual_width = self.0.__bindgen_anon_2.__bindgen_anon_1.width;
            let raw_data_ptr =
                citro3d_sys::C3D_Tex2DGetImagePtr(&raw mut self.0, 0, std::ptr::null_mut())
                    .byte_add((tile_x + tile_y * actual_width / 8) as usize * 64 * size_of::<T>());
            &mut *(raw_data_ptr as *mut [T; 64])
        }
    }

    pub fn raw_texture<T: PixelType>(&self) -> &[T] {
        debug_assert!(!addr_is_vram(unsafe { self.0.__bindgen_anon_1.data }));

        unsafe {
            let dims = self.0.__bindgen_anon_2.__bindgen_anon_1;
            let n = dims.width as usize * dims.height as usize;
            let raw_data_ptr = citro3d_sys::C3D_Tex2DGetImagePtr(
                &raw const self.0 as *mut _,
                0,
                std::ptr::null_mut(),
            );
            std::slice::from_raw_parts(raw_data_ptr as *const T, n)
        }
    }

    pub fn raw_texture_mut<T: PixelType>(&mut self) -> &mut [T] {
        debug_assert!(!addr_is_vram(unsafe { self.0.__bindgen_anon_1.data }));

        unsafe {
            let dims = self.0.__bindgen_anon_2.__bindgen_anon_1;
            let n = dims.width as usize * dims.height as usize;
            let raw_data_ptr =
                citro3d_sys::C3D_Tex2DGetImagePtr(&raw mut self.0, 0, std::ptr::null_mut());
            std::slice::from_raw_parts_mut(raw_data_ptr as *mut T, n)
        }
    }
}

#[doc(alias = "addrIsVRAM")]
fn addr_is_vram(out: *mut std::ffi::c_void) -> bool {
    let vaddr = out as u32;
    (ctru_sys::OS_VRAM_VADDR..ctru_sys::OS_VRAM_VADDR + ctru_sys::OS_VRAM_SIZE).contains(&vaddr)
}

fn bytes_per_pixel(fmt: ColourFormat) -> usize {
    match fmt {
        ColourFormat::Rgba8 => 4,
        ColourFormat::Rgb8 => 3,
        ColourFormat::Rgba4
        | ColourFormat::La8
        | ColourFormat::Hilo8
        | ColourFormat::Rgba5551
        | ColourFormat::Rgb565 => 2,
        ColourFormat::L8 | ColourFormat::A8 | ColourFormat::La4 | ColourFormat::Etc1A4 => 1,
        ColourFormat::L4 | ColourFormat::A4 | ColourFormat::Etc1 => 1, // 0.5?
    }
}

fn swizzle<T: Copy + Default, const M: usize, const N: usize, const O: usize>(
    data: &mut [[T; M]; N],
) {
    debug_assert_eq!(M % 8, 0);
    debug_assert_eq!(N % 8, 0);
    debug_assert_eq!(M * N, O);

    // Check assembly, is this initialisation optimised out or do we need to use some MaybeUninit hack?
    let mut out = [Default::default(); O];

    for ((x, y), dst) in (0..N)
        .step_by(8)
        .flat_map(|n| (0..M).step_by(8).map(move |m| (m, n)))
        .flat_map(move |(x, y)| {
            SWIZZLE_ORDER_2D
                .iter()
                .copied()
                .map(move |(dx, dy)| (x + dx, dy + y))
        })
        .zip(out.iter_mut())
    {
        *dst = data[x][y]
    }

    let data = unsafe { std::mem::transmute::<&mut [[T; M]; N], &mut [T; O]>(data) };
    *data = out;
}

#[rustfmt::skip]
const SWIZZLE_ORDER_2D: [(usize, usize); 64] = [
    (0,0), (1,0), (0,1), (1,1),
    (2,0), (3,0), (2,1), (3,1),
    (0,2), (1,2), (0,3), (1,3),
    (2,2), (3,2), (2,3), (3,3),
    (4,0), (5,0), (4,1), (5,1),
    (6,0), (7,0), (6,1), (7,1),
    (4,2), (5,2), (4,3), (5,3),
    (6,2), (7,2), (6,3), (7,3),
    (0,4), (1,4), (0,5), (1,5),
    (2,4), (3,4), (2,5), (3,5),
    (0,6), (1,6), (0,7), (1,7),
    (2,6), (3,6), (2,7), (3,7),
    (4,4), (5,4), (4,5), (5,5),
    (6,4), (7,4), (6,5), (7,5),
    (4,6), (5,6), (4,7), (5,7),
    (6,6), (7,6), (6,7), (7,7),
];
