pub enum ThemeType {
    None,
    BuiltIn,
    FromSD,
    Unk3,
    Unk4,
    Unk5
}

pub struct ThemeEntry {
    pub index: u32,
    pub dlc_titleid: u8,
    pub theme_type: ThemeType,
    pub null: [u8; 2]
}

pub struct SaveData {
    pub version: u8,
    pub pad0: [u8; 7],
    pub titles: [u64; 360],
    pub is_icon_present: [bool; 360],
    pub pad1: [u8; 8],
    pub array_a: [i16; 360],
    pub array_b: [i8; 360],
    pub pad2: [u8; 0x2d0],
    pub theme_standard: ThemeEntry,
    pub theme_shuffle: [ThemeEntry; 10],
    pub pad3: [u8; 0xb],
    pub is_shuffle: bool,
    pub pad4: [u8; 0x1984], // literally 1984
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