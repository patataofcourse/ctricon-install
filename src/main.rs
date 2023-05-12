#![feature(io_error_downcast, let_chains, never_type)]

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

mod abort;
mod ctru_fs_extension;
mod home_menu;

use abort::HowFix;
use ctru_fs_extension::*;
use home_menu::Utf16;

fn main() -> anyhow::Result<()> {
    ctru::use_panic_handler();

    let gfx = Gfx::new()?;
    let mut hid = Hid::new()?;
    let apt = Apt::new()?;
    let mut fs = Fs::new()?;
    let _console = Console::new(gfx.top_screen.borrow_mut());

    println!("Home menu extdata stuff");
    println!("patataofcourse#5556\n");

    let sdmc = fs.sdmc()?;

    let Some((extdata, id)) = open_extdata(fs) else {
        abort!(apt, hid, gfx, "No Home Menu extra data could be found!", HowFix::RunMenu)
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
        abort!(apt, hid, gfx, "Something seems to be wrong with your Home Menu!", HowFix::RunMenu)
    };
    let Ok(mut f_cached) = f_cached else {
        abort!(apt, hid, gfx, "Couldn't load the icon cache - likely in use", HowFix::Autoboot)
    };

    if f_savedata.metadata()?.len() != mem::size_of::<home_menu::SaveData>() as u64 {
        abort!(
            apt,
            hid,
            gfx,
            "/SaveData.dat is the wrong size!",
            HowFix::Update
        )
    }
    if f_cache.metadata()?.len() != mem::size_of::<home_menu::CacheDat>() as u64 {
        abort!(
            apt,
            hid,
            gfx,
            "/Cache.dat is the wrong size!",
            HowFix::RunMenu
        )
    }
    if f_cached.metadata()?.len() != mem::size_of::<home_menu::CacheDDat>() as u64 {
        abort!(
            apt,
            hid,
            gfx,
            "/CacheD.dat is the wrong size!",
            HowFix::RunMenu
        )
    }

    let mut savedata = [0u8; mem::size_of::<home_menu::SaveData>()];
    f_savedata.read_exact(&mut savedata)?;
    let savedata: home_menu::SaveData = unsafe { mem::transmute(savedata) };

    if savedata.version < 4 {
        abort!(
            apt,
            hid,
            gfx,
            "Outdated /Savedata.dat version!",
            HowFix::Update
        )
    }

    let mut cache_dat = [0u8; mem::size_of::<home_menu::CacheDat>()];
    f_cache.read_exact(&mut cache_dat)?;
    let cache_dat: home_menu::CacheDat = unsafe { mem::transmute(cache_dat) };

    let mut icon_titles = vec![];
    {
        let Ok(icon_dir) = fs::read_dir(&sdmc, "/3ds/ctricon-install") else {
            abort!(apt, hid, gfx, "Couldn't open /3ds/ctricon-install folder!", HowFix::CtrIconFolder)
        };
        for title in icon_dir {
            let title = title?.path();
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
            titles.push(title);
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
                println!(
                    "{id:016x} - {}",
                    (&home_menu::get_cache_d_icon(&mut f_cached, cache_dat.position(*id))?
                        .ctr()
                        .title_en
                        .short)
                        .read_utf16()?
                );
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
    Ok(())
}

const EXTDATA_IDS: [u64; 3] = [0x00000082, 0x0000008f, 0x00000098];

pub fn open_extdata(fs: Fs) -> Option<(Archive, u64)> {
    for id in EXTDATA_IDS {
        match fs.extdata(id, FsMediaType::Sd) {
            Ok(c) => return Some((c, id)),
            Err(_e) => {}
        }
    }
    None
}
