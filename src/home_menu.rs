#[repr(u8)]
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

pub type CtrIcon = [u8; 0x36c0];

pub type CacheDDat = [CtrIcon; 360];
