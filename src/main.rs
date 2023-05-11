#![feature(io_error_downcast, let_chains)]

use std::{
    io::Read,
    mem::{self},
};

use ctru::{
    prelude::{Console, Gfx, KeyPad},
    services::{
        fs::{self, Archive, File, Fs, FsMediaType},
        Apt, Hid,
    },
};

mod ctru_fs_extension;
mod home_menu;

use ctru_fs_extension::*;
use ctru_sys::svcExitProcess;

// for debugging :D
fn offset_of<T1, T2>(a: &T1, b: &T2) -> isize {
    b as *const T2 as isize - a as *const T1 as isize
}

fn main() {
    ctru::use_panic_handler();

    let gfx = Gfx::new().expect("Couldn't obtain GFX controller");
    let mut hid = Hid::new().expect("Couldn't obtain HID controller");
    let apt = Apt::new().expect("Couldn't obtain APT controller");
    let mut fs = Fs::new().expect("Couldn't get FS controller");
    let _console = Console::new(gfx.top_screen.borrow_mut());

    println!("Home menu extdata stuff");
    println!("patataofcourse#5556\n");

    let sdmc = fs.sdmc().unwrap();

    let Some((extdata, id)) = open_extdata(fs) else {
        println!("\nNo Home Menu extdata could be found!");
        println!("Run the home menu and then try again");
        prompt_exit(&apt, &mut hid, &gfx);
    };
    println!("Extdata ID: {:08x}", id);

    let f_savedata = File::open(&extdata, "/SaveData.dat");
    let f_cache = File::open(&extdata, "/Cache.dat");
    let f_cached = File::open(&extdata, "/CacheD.dat");

    println!(
        "SaveData: {}",
        if f_savedata.is_ok() { "ok" } else { "err" }
    );
    println!("Cache: {}", if f_cache.is_ok() { "ok" } else { "err" });
    println!("CacheD: {}", if f_cached.is_ok() { "ok" } else { "err" });

    let (Ok(mut f_savedata), Ok(mut f_cache)) =  (f_savedata, f_cache) else {
        println!("\nSomething seems to be wrong with your Home Menu!");
        println!("Run the home menu and then try again");
        prompt_exit(&apt, &mut hid, &gfx);
    };
    let Ok(mut f_cached) = f_cached else {
        println!("\nCouldn't load the icon cache - likely in use");
        println!("Make sure to run this on autoboot mode");
        prompt_exit(&apt, &mut hid, &gfx);
    };

    if f_savedata.metadata().unwrap().len() != mem::size_of::<home_menu::SaveData>() as u64 {
        println!("\n/SaveData.dat is the wrong size!");
        println!("Update your console, run the home menu and then try again");
        prompt_exit(&apt, &mut hid, &gfx);
    }

    let mut savedata = [0u8; mem::size_of::<home_menu::SaveData>()];
    f_savedata.read_exact(&mut savedata).unwrap();
    let savedata: home_menu::SaveData = unsafe { mem::transmute(savedata) };

    if savedata.version < 4 {
        println!("Outdated /Savedata.dat version!");
        println!("Update your console, run the home menu and then try again");
        prompt_exit(&apt, &mut hid, &gfx);
    }

    let mut icon_titles = vec![];
    {
        let Ok(icon_dir) = fs::read_dir(&sdmc, "/3ds/ctricon-install") else {
            println!("Couldn't open /3ds/ctricon-install folder!");
            println!("Make the folder and store the icons to install there");
            prompt_exit(&apt, &mut hid, &gfx);
        };
        for title in icon_dir {
            let title = title.unwrap().path();
            if let Some(Some("icn")) = title.extension().map(|c| c.to_str()) {
                let name = title.file_stem().unwrap().to_str().unwrap();
                if let Ok(c) = u32::from_str_radix(name, 16) {
                    icon_titles.push(c);
                }
            }
        }
    }

    let mut titles = vec![];
    for title in savedata.titles {
        if (title >> 32) == 0x00040000 && icon_titles.contains(&(title as u32)) {
            titles.push(title as u32);
        }
    }
    println!("{}", titles.len());

    println!("\nPress A to print titles (10 at a time)");
    println!("Press START to exit");

    let mut title_pos = 0;

    // Main loop
    while apt.main_loop() {
        //Scan all the inputs. This should be done once for each frame
        hid.scan_input();

        if hid.keys_down().contains(KeyPad::A) && titles.len() > title_pos {
            for id in titles.iter().skip(title_pos).take(10) {
                println!("{id:016x}");
            }
            println!("\nPress A to print 10 more");
            println!("Press START to exit");
            title_pos += 10;
        }

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
    unsafe {svcExitProcess();}
    //std::process::exit(1);
}
