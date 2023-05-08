#![feature(io_error_downcast)]


use ctru::{
    prelude::{Console, Gfx, KeyPad},
    services::{
        fs::{Archive, Fs, FsMediaType},
        Apt, Hid,
    },
};

mod home_menu;
mod ctru_fs_extension;

use ctru_fs_extension::*;

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
    println!("SaveData: {:?}", extdata.check_file("SaveData.dat"));
    println!("Cache: {:?}", extdata.check_file("Cache.dat"));
    println!("CacheD: {:?}", extdata.check_file("CacheD.dat"));

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
            Err(e) => println!(" Error opening {:08x}\n {}", id, e),
        }
    }
    None
}