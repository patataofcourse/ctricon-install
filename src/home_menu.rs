use std::{
    io::{self, Read, Seek},
    mem,
    string::FromUtf16Error,
};

use static_assertions::const_assert;

#[repr(u8)]
#[allow(dead_code)]
pub enum ThemeType {
    None,
    BuiltIn,
    FromSD,
    Unk3,
    Unk4,
    Unk5,
}

pub struct ThemeEntry {
    pub index: u32,
    pub dlc_titleid: u8,
    pub theme_type: ThemeType,
    pub null: [u8; 2],
}

#[repr(C)]
pub struct SaveData {
    pub version: u8,
    pub unk0: [u8; 7],
    pub titles: [u64; 360],
    pub is_icon_present: [bool; 360],
    pub titles_pos: [i16; 360],
    pub titles_folder: [i8; 360],
    pub unk1: [u8; 0x2d0],
    pub theme_standard: ThemeEntry,
    pub theme_shuffle: [ThemeEntry; 10],
    pub unk2: [u8; 0xb],
    pub is_shuffle: bool,
    pub unk3: [u8; 0x1984], // literally 1984
}

pub struct CacheDatEntry {
    pub titleid: u64,
    pub unk: [u32; 2],
}

pub struct CacheDat {
    pub format_ver: u8,
    pub pad: [u8; 7],
    pub entries: [CacheDatEntry; 360],
}

impl CacheDat {
    pub fn position(&self, id: u64) -> usize {
        self.entries
            .iter()
            .position(|c| c.titleid == id)
            .unwrap_or(0)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct AppTitles {
    pub short: [u8; 0x80],
    pub long: [u8; 0x100],
    pub publisher: [u8; 0x80],
}

#[derive(Clone, Copy, Debug)]
pub struct GameRatings {
    pub cero: u8,
    pub esrb: u8,
    pub null0: u8,
    pub usk: u8,
    pub pegi: u8,
    pub null1: u8,
    pub pegi_prt: u8,
    pub pegi_bffc: u8,
    pub cob: u8,
    pub grb: u8,
    pub cgsrr: u8,
    pub null2: [u8; 5],
}

#[allow(dead_code)]
impl GameRatings {
    pub const HAS_RATING: u32 = 0x80;
    pub const RATING_PENDING: u32 = 0x40;
    pub const NO_RESTRICTION: u32 = 0x20;
    pub const AGE_MASK: u32 = 0x1F;
}

#[derive(Clone, Copy, Debug)]
pub struct AppSettings {
    pub ratings: GameRatings,
    pub regions: u32,
    pub match_maker_id: u32,
    pub match_maker_bit_id: u64,
    pub flags: u32,
    pub eula_minor: u8,
    pub eula_major: u8,
    pub optimal_animation_default_frame: f32,
    pub streetpass_id: u32,
}

#[allow(dead_code)]
impl AppSettings {
    pub const REGION_JPN: u32 = 1;
    pub const REGION_USA: u32 = 2;
    pub const REGION_EUR: u32 = 4;
    pub const REGION_AUS: u32 = 8;
    pub const REGION_CHN: u32 = 0x10;
    pub const REGION_KOR: u32 = 0x20;
    pub const REGION_TWN: u32 = 0x40;

    pub const FLAGS_VISIBLE: u32 = 1;
    pub const FLAGS_AUTOBOOT: u32 = 2;
    pub const FLAGS_ALLOW_3D: u32 = 4;
    pub const FLAGS_REQUIRE_EULA: u32 = 8;
    pub const FLAGS_AUTOSAVE: u32 = 0x10;
    pub const FLAGS_EXBANNER: u32 = 0x20;
    pub const FLAGS_RATING_REQUIRED: u32 = 0x40;
    pub const FLAGS_HAS_SAVE: u32 = 0x80;
    pub const FLAGS_RECORD_USAGE: u32 = 0x100;
    pub const FLAGS_DISABLE_SAVE_BACKUP: u32 = 0x400;
    pub const FLAGS_N3DS_ONLY: u32 = 0x1000;
}

pub type IconGfxSmall = [u8; 0x480];
pub type IconGfxLarge = [u8; 0x1200];

#[derive(Clone, Copy, Debug)]
pub struct CtrIcon {
    pub magic: [u8; 4], // b"SMDH"
    pub version: u16,
    pub reserved: u16,
    pub title_jp: AppTitles,
    pub title_en: AppTitles,
    pub title_fr: AppTitles,
    pub title_de: AppTitles,
    pub title_it: AppTitles,
    pub title_es: AppTitles,
    pub title_ch: AppTitles,
    pub title_kr: AppTitles,
    pub title_nl: AppTitles,
    pub title_pt: AppTitles,
    pub title_ru: AppTitles,
    pub title_tw: AppTitles,
    pub title_unused: [AppTitles; 4],
    pub settings: AppSettings,
    pub null: u64,
    pub icon_small: IconGfxSmall,
    pub icon_large: IconGfxLarge,
}

pub type NtrIcon = [u8; 0x36c0];

pub union Icon {
    ctr: CtrIcon,
    ntr: NtrIcon,
}

impl Icon {
    pub fn ctr(&self) -> &CtrIcon {
        unsafe { &self.ctr }
    }
    pub fn ntr(&self) -> &NtrIcon {
        unsafe { &self.ntr }
    }
}

pub type CacheDDat = [Icon; 360];

pub fn get_cache_d_icon<F: Read + Seek>(f: &mut F, pos: usize) -> io::Result<Icon> {
    f.seek(io::SeekFrom::Start((pos * mem::size_of::<Icon>()) as u64))?;
    let mut icon = [0u8; mem::size_of::<Icon>()];
    f.read_exact(&mut icon)?;
    Ok(unsafe { mem::transmute(icon) })
}

// assertions
const_assert!(mem::size_of::<ThemeEntry>() == 8);
const_assert!(mem::size_of::<SaveData>() == 0x2da0);
const_assert!(mem::size_of::<CacheDatEntry>() == 0x10);
const_assert!(mem::size_of::<CacheDat>() == 0x1688);
const_assert!(mem::size_of::<AppTitles>() == 0x200);
const_assert!(mem::size_of::<GameRatings>() == 0x10);
const_assert!(mem::size_of::<AppSettings>() == 0x30);
const_assert!(mem::size_of::<CtrIcon>() == 0x36c0);
//const_assert!(mem::size_of::<NtrIcon>() == 0x36c0);
const_assert!(mem::size_of::<Icon>() == 0x36c0);

pub trait Utf16 {
    fn read_utf16(&self) -> Result<String, FromUtf16Error>;
}

impl<const N: usize> Utf16 for &[u8; N] {
    fn read_utf16(&self) -> Result<String, FromUtf16Error> {
        String::from_utf16(
            &(0..N / 2)
                .map(|i| u16::from_le_bytes([self[2 * i], self[2 * i + 1]]))
                .collect::<Vec<_>>(),
        )
    }
}
