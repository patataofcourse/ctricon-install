use std::{error::Error, fmt::Display};

use ctru::{
    prelude::{Gfx, KeyPad},
    services::{Apt, Hid},
};

#[macro_export]
macro_rules! abort {
    ($apt: expr, $hid: expr, $gfx: expr, $explain: literal, $instr: expr) => {{
        static_assertions::const_assert!($explain.len() <= 50);
        println!("");
        println!($explain);
        println!("{}", $instr);
        $crate::abort::prompt_exit(&$apt, &mut $hid, &$gfx)?;
    }};
}

pub fn prompt_exit(apt: &Apt, hid: &mut Hid, gfx: &Gfx) -> Result<!, AbortProgram> {
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
    Err(AbortProgram)
}

#[derive(Debug)]
pub struct AbortProgram;

impl Display for AbortProgram {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Program execution aborted")
    }
}

impl Error for AbortProgram {}

#[allow(unused)]
pub enum HowFix {
    Update,
    RunMenu,
    Autoboot,
    CtrIconFolder,
}

impl Display for HowFix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "{}",
            match self {
                Self::Update => "Update your console, run the home menu and then try again",
                Self::RunMenu => "Run the home menu and then try again",
                Self::Autoboot =>
                    "Make sure to run this on autoboot mode (enable in Luma settings)",
                Self::CtrIconFolder =>
                    "Make the folder and store the icons you want to install there",
            }
        )
    }
}
