#![feature(io_error_downcast, let_chains)]

use ctru::{
    prelude::{Console, Gfx, KeyPad},
    services::{
        fs::{Archive, Fs, FsMediaType, File},
        Apt, Hid,
    },
};

mod ctru_fs_extension;
mod home_menu;

use ctru_fs_extension::*;

fn main() {
    ctru::use_panic_handler();

    let gfx = Gfx::new().expect("Couldn't obtain GFX controller");
    let mut hid = Hid::new().expect("Couldn't obtain HID controller");
    let apt = Apt::new().expect("Couldn't obtain APT controller");
    let fs = Fs::new().expect("Couldn't get FS controller");
    let _console = Console::new(gfx.top_screen.borrow_mut());

    println!("Home menu extdata stuff");
    println!("patataofcourse#5556\n");
    let (extdata, id) = open_extdata(fs).expect("Couldn't open Home Menu extra data");
    println!("Extdata ID: {:08x}", id);

    let f_savedata = File::open(&extdata, "/SaveData.dat");
    let f_cache = File::open(&extdata, "/Cache.dat");
    let f_cached = File::open(&extdata, "/CacheD.dat");

    println!("SaveData: {}", if f_savedata.is_ok() {"ok"} else {"err"});
    println!("Cache: {}", if f_cache.is_ok() {"ok"} else {"err"});
    println!("CacheD: {}", if f_cached.is_ok() {"ok"} else {"err"});

    if f_savedata.is_err() || f_cache.is_err() {
        println!("\nSomething seems to be wrong with your Home Menu!");
        println!("Run the home menu and then try again");
        prompt_exit(&apt, &mut hid, &gfx);
    }
    if f_cached.is_err() {
        println!("\nCouldn't load the icon cache - likely in use");
        println!("Make sure to run this on autoboot mode");
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
            Err(_e) => {
                //println!(" Error opening {:08x}\n {}", id, e)
            }
        }
    }
    None
}

pub fn prompt_exit(apt: &Apt, hid: &mut Hid, gfx: &Gfx) -> ! {
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
    std::process::exit(0);
}