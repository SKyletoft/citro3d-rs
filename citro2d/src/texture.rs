use citro3d::texture::ColourFormat;

#[doc(alias = "C3D_Tex")]
#[repr(transparent)]
pub struct Tex(pub(crate) citro3d_sys::C3D_Tex);

impl Tex {
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

    #[doc(alias = "C3D_TexUpload")]
    // TODO: When const generic expressions are stable O should be removed and replaced with M * N
    pub fn swizzle_and_upload<T: Default + Copy, const M: usize, const N: usize, const O: usize>(
        &mut self,
        texture: &[[T; M]; N],
    ) {
        let h = unsafe { self.0.__bindgen_anon_2.__bindgen_anon_1.height as usize };
        let w = unsafe { self.0.__bindgen_anon_2.__bindgen_anon_1.width as usize };
        let fmt = self.0._bitfield_1.get(0, 4) as u8;
        let fmt = ColourFormat::try_from(fmt).unwrap();

        let mut texture = texture.clone();
        swizzle::<T, M, N, O>(&mut texture);

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
        ColourFormat::Rgba4
        | ColourFormat::La8
        | ColourFormat::Hilo8
        | ColourFormat::Rgba5551
        | ColourFormat::Rgb565 => 2,
        ColourFormat::L8 | ColourFormat::A8 | ColourFormat::LA4 | ColourFormat::Etc1A4 => 1,
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
