use std::{ffi::c_void, mem};

use ctru::{
    prelude::{Console, Gfx, KeyPad},
    services::{
        fs::{self, Archive, ArchiveID, Fs, FsMediaType, PathType},
        Apt, Hid,
    },
};

mod home_menu;

fn main() {
    ctru::use_panic_handler();

    let gfx = Gfx::new().expect("Couldn't obtain GFX controller");
    let mut hid = Hid::new().expect("Couldn't obtain HID controller");
    let apt = Apt::new().expect("Couldn't obtain APT controller");
    let fs = Fs::new().expect("Couldn't get FS controller");
    let _console = Console::new(gfx.top_screen.borrow_mut());

    println!("Home menu extdata stuff");
    let (extdata, id) = open_extdata(fs).expect("Couldn't open Home Menu extra data");
    println!("Extdata ID: {:08x}", id);
    println!("Filesystem:");
    for f in fs::read_dir(&extdata, "").unwrap() {
        println!(" - {:?}", f.unwrap().path());
    }

    println!("\nPress START to exit");

    // Main loop
    while apt.main_loop() {
        //Scan all the inputs. This should be done once for each frame
        hid.scan_input();

        if hid.keys_down().contains(KeyPad::START) {
            break;
        }

        //Wait for VBlank
        gfx.wait_for_vblank();
    }
}

const EXTDATA_IDS: [u64; 3] = [0x00000082, 0x0000008f, 0x00000098];

pub fn open_extdata(fs: Fs) -> Option<(Archive, u64)> {
    for id in EXTDATA_IDS {
        match fs.extdata(id, FsMediaType::Sd) {
            Ok(c) => return Some((c, id)),
            Err(e) =>println!(" Error opening {:08x}\n {}", id, e)
        }
    }
    None
}

#[allow(dead_code)]
struct TotallyNotArchive {
    id: ArchiveID,
    handle: u64,
}

trait FsPlus {
    fn extdata(&self, id: u64, media_type: FsMediaType) -> ctru::Result<Archive>;
    fn binary_path<const T: usize>(&self, data: &[u32; T]) -> ctru_sys::FS_Path {
        ctru_sys::FS_Path {
            type_: PathType::Binary.into(),
            size: T as u32 * 4,
            data: data.as_ptr() as *const c_void,
        }
    }
}

impl FsPlus for Fs {
    fn extdata(&self, id: u64, media_type: FsMediaType) -> ctru::Result<Archive> {
        let id_lower = id as u32;
        let id_higher = (id >> 32) as u32;
        unsafe {
            let mut handle = 0;
            let id = ArchiveID::Extdata;
            let path = self.binary_path(&[media_type.into(), id_lower, id_higher]);
            println!("{:?} {:X?}", path, *(path.data as *const [u32; 3]));
            let r = ctru_sys::FSUSER_OpenArchive(&mut handle, id.into(), path);
            if r < 0 {
                Err(ctru::Error::from(r))
            } else {
                Ok(mem::transmute(TotallyNotArchive { handle, id }))
            }
        }
    }
}
