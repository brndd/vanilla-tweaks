//Patches some QoL stuff into the old 1.12.1 WoW client.

//Files can be verified after patching with:
//cmp -l WoW_patched.exe WoW.exe | gawk '{printf "%08X %02X %02X\n", $1, strtonum(0$2), strtonum(0$3)}'

use std::fs;
use std::process::ExitCode;
use std::ffi::OsString;
use std::ffi::CString;
use clap::Parser;

// Command line arguments
#[derive(Parser, Debug)]
#[clap(author)]
#[clap(version)]
#[clap(about = "Applies patches to enhance the functionality of the 1.12.1 World of Warcraft client")]
#[clap(long_about = "Applies patches to enhance the functionality of the 1.12.1 World of Warcraft client.

The following patches are currently available and are all applied by default:
- Widescreen FoV fix
- Sound in background patch
- Sound channel count increase
- Farclip (terrain render distance) maximum value change
- Frilldistance (grass render distance) change
- Quickloot by default patch (hold shift for manual loot)
- Nameplate range change
- Large address aware patch
- Camera rotation skip glitch fix")]
struct Args {
    /// Path to WoW.exe.
    #[clap(value_parser)]
    infile: String,

    /// Filename of the output file.
    #[clap(short, default_value_t = String::from("WoW_tweaked.exe"), value_parser)]
    outfile: String,

    /// FoV value in radians. Default game value is 1.5708.
    #[clap(long, default_value_t = 1.925f32, value_parser)]
    fov: f32,

    /// Farclip (terrain render distance) maximum value. Default game value is 777. Set with `/console farclip 1000` in-game.
    #[clap(long, default_value_t = 10000f32, value_parser)]
    farclip: f32,

    /// Frilldistance (grass render distance) value. Default game value is 70.
    #[clap(long, default_value_t = 300f32, value_parser)]
    frilldistance: f32,

    /// Nameplate distance in yards. Default game value is 20.
    #[clap(long, default_value_t = 41f32, value_parser)]
    nameplatedistance: f32,

    /// Default sound channel count. This can also be set with /console SoundSoftwareChannels 64, but is included here so that the changes persist if Config.wtf is deleted.
    /// Default game value is 12. Default value in TBC is 32(?). Default value in modern client is 64. 999 is the maximum value settable here.
    /// If you experience problems with performance, try lowering this. Values above 64 may cause crashes.
    #[clap(long, default_value_t = 64i32, value_parser = clap::value_parser!(i32).range(1..999))]
    soundchannels: i32,

    /// Max camera distance LIMIT. Current max camera distance is a setting in the menu & a console command. Default game value is 50. Unchanged by default. Should be greater than 0, otherwise bad things may happen.
    /// After patching, change with /console CameraDistanceMax 100
    #[clap(long, value_parser)]
    maxcameradistance: Option<f32>,

    /// If set, do not patch FoV.
    #[clap(long, default_value_t = false, value_parser)]
    no_fov: bool,

    /// If set, do not patch farclip.
    #[clap(long, default_value_t = false, value_parser)]
    no_farclip: bool,
    
    /// If set, do not patch frilldistance.
    #[clap(long, default_value_t = false, value_parser)]
    no_frilldistance: bool,

    /// If set, do not patch sound in background.
    #[clap(long, default_value_t = false, value_parser)]
    no_sound_in_background: bool,

    /// If set, do not patch quickloot.
    #[clap(long, default_value_t = false, value_parser)]
    no_quickloot: bool,

    /// If set, do not patch nameplate distance.
    #[clap(long, default_value_t = false, value_parser)]
    no_nameplatedistance: bool,

    /// If set, do not patch the number of sound channels.
    #[clap(long, default_value_t = false, value_parser)]
    no_soundchannels: bool,

    /// If set, do not patch the executable to be Large Address Aware.
    /// You may want to enable this if playing on incredibly low-end hardware with less than 3 GiB RAM.
    #[clap(long, default_value_t = false, value_parser)]
    no_largeaddressaware: bool,

    // If set, do not patch the fix for the camera sometimes skipping to a random direction when rotated.
    #[clap(long, default_value_t = false, value_parser)]
    no_cameraskipfix: bool
}

/**
 * Replaces the first occurrence of find with replace, mutating haystack.
 * Returns true if a replacement occurred, false if not.
 */
#[allow(dead_code)] //unused, but I want to keep this here in case it's necessary later, so shut up compiler
fn replace(haystack: &mut Vec<u8>, find: &Vec<u8>, replace: &Vec<u8>) -> bool {
    if haystack.len() < find.len() {
        return false;
    }

    if haystack.len() < replace.len() {
        return false;
    }

    let mut match_index: Option<usize> = None;
    for i in 0..haystack.len() - find.len() + 1 {
        if haystack[i..i+find.len()] == find[..] {
            match_index = Some(i);
        }
    }

    let match_index = match match_index {
        None => return false,
        Some(idx) => idx
    };

    haystack.splice(match_index..match_index+replace.len(), replace.iter().cloned());
    return true;
}

fn main() -> ExitCode {
    let args = Args::parse();

    //Open input file
    let file_path = &args.infile;
    let mut file: std::vec::Vec<u8> = match fs::read(file_path) {
        Ok(file) => file,
        Err(err) => {
            println!("Unable to read file: {err}");
            return ExitCode::from(1);
        }
    };

    let outfile_path = OsString::from(&args.outfile);

    /*
     * PATCHES PATCHES PATCHES PATCHES
     */

    // Large address aware patch
    if !args.no_largeaddressaware {
        const CHARACTERISTICS_OFFSET: usize = 0x126;
        let mut characteristics = u16::from_le_bytes(file[CHARACTERISTICS_OFFSET..CHARACTERISTICS_OFFSET+2].try_into().expect("slice with incorrect length!"));
        characteristics = characteristics | 0x20; // https://docs.microsoft.com/en-us/windows/win32/debug/pe-format#characteristics
        let characteristics_bytes = characteristics.to_le_bytes();
        print!("Applying patch: make executable large address aware...");
        file[CHARACTERISTICS_OFFSET..CHARACTERISTICS_OFFSET+characteristics_bytes.len()].copy_from_slice(&characteristics_bytes);
        println!(" Success!");
    }

    // Farclip patch
    if !args.no_farclip {
        const FARCLIP_OFFSET: usize = 0x40FED8;
        let farclip_bytes: [u8; 4] = args.farclip.to_le_bytes();
        print!("Applying patch: increased farclip max value...");
        file[FARCLIP_OFFSET..FARCLIP_OFFSET+farclip_bytes.len()].copy_from_slice(&farclip_bytes);
        println!(" Success!");
    }

    // Widescreen FoV patch
    if !args.no_fov {
        const FOV_OFFSET: usize = 0x4089B4;
        let fov_bytes = args.fov.to_le_bytes();
        print!("Applying patch: widescreen FoV fix...");
        file[FOV_OFFSET..FOV_OFFSET+fov_bytes.len()].copy_from_slice(&fov_bytes);
        println!(" Success!");
    }

    // Frilldistance patch
    if !args.no_frilldistance {
        const FRILLDISTANCE_OFFSET: usize = 0x467958;
        let frilldistance_bytes = args.frilldistance.to_le_bytes();
        print!("Applying patch: frilldistance (grass distance) increase...");
        file[FRILLDISTANCE_OFFSET..FRILLDISTANCE_OFFSET+frilldistance_bytes.len()].copy_from_slice(&frilldistance_bytes);
        println!(" Success!");
    }

    // Sound in background patch
    if !args.no_sound_in_background {
        const SOUND_IN_BACKGROUND_OFFSET: usize = 0x3A4869;
        const SOUND_IN_BACKGROUND_BYTES: [u8; 1] = [0x27];
        print!("Applying patch: sound in background...");
        file[SOUND_IN_BACKGROUND_OFFSET..SOUND_IN_BACKGROUND_OFFSET+SOUND_IN_BACKGROUND_BYTES.len()].copy_from_slice(&SOUND_IN_BACKGROUND_BYTES);
        println!(" Success!");
    }

    // Sound channels patch
    if !args.no_soundchannels {
        const SOUNDCHANNEL_OFFSET: usize = 0x435d38;
        let soundchannel_string = args.soundchannels.to_string();
        print!("Applying patch: software sound channels default increase...");
        let cstring = CString::new(soundchannel_string).expect("CString::new failed");
        let soundchannel_bytes = cstring.to_bytes_with_nul();
        if soundchannel_bytes.len() <= 4 {
            file[SOUNDCHANNEL_OFFSET..SOUNDCHANNEL_OFFSET+soundchannel_bytes.len()].copy_from_slice(&soundchannel_bytes);
            println!(" Success!");
        }
        else {
            println!(" FAILED!");
            println!("Sound channels value is too long.");
            return ExitCode::from(1);
        }
    }

    // Quickloot key reverse patch (hold shift to manual loot)
    if !args.no_quickloot {
        const QUICKLOOT_OFFSET: usize = 0x0C1ECF;
        const QUICKLOOT_BYTES: [u8; 1] = [0x75];
        const QUICKLOOT_OFFSET2: usize = 0x0C2B25;
        const QUICKLOOT_BYTES2: [u8; 1] = [0x75];
        print!("Applying patch: quickloot reverse...");
        file[QUICKLOOT_OFFSET..QUICKLOOT_OFFSET+QUICKLOOT_BYTES.len()].copy_from_slice(&QUICKLOOT_BYTES);
        file[QUICKLOOT_OFFSET2..QUICKLOOT_OFFSET2+QUICKLOOT_BYTES2.len()].copy_from_slice(&QUICKLOOT_BYTES2);
        println!(" Success!");
    }

    // Nameplate range change patch
    if !args.no_nameplatedistance {
        const NAMEPLATE_OFFSET: usize = 0x40c448;
        let nameplate_bytes: [u8; 4] = args.nameplatedistance.to_le_bytes();
        print!("Applying patch: nameplate range...");
        file[NAMEPLATE_OFFSET..NAMEPLATE_OFFSET+nameplate_bytes.len()].copy_from_slice(&nameplate_bytes);
        println!(" Success!");
    }

    // Max camera distance patch
    if let Some(maxcameradistance) = args.maxcameradistance {
        const MAXCAMERADISTANCE_OFFSET: usize = 0x4089a4;
        let maxcamera_bytes: [u8; 4] = maxcameradistance.to_le_bytes();
        print!("Applying patch: max camera distance...");
        file[MAXCAMERADISTANCE_OFFSET..MAXCAMERADISTANCE_OFFSET+maxcamera_bytes.len()].copy_from_slice(&maxcamera_bytes);
        println!(" Success!");
    }

    // Camera skip glitch fix.
    // Thanks to Bon on the Turtle WoW Discord for implementing this patch, and phamd for submitting the PR to include it.
    if !args.no_cameraskipfix {
        let patches: [(usize, Vec<u8>); 5] = [
            (0x02ccd0, vec![0x55, 0x8b, 0x05, 0x48, 0x4e, 0x88, 0x00, 0x8b, 0x0d, 0x44, 0x4e, 0x88, 0x00, 0xe9, 0x33, 0x90,
                            0x32, 0x00, 0x83, 0xc0, 0x32, 0x83, 0xc1, 0x32, 0x3b, 0x0d, 0xa8, 0xeb, 0xc4, 0x00, 0x7e, 0x03,
                            0x83, 0xe9, 0x01, 0x3b, 0x05, 0xac, 0xeb, 0xc4, 0x00, 0x7e, 0x03, 0x83, 0xe8, 0x01, 0x83, 0xe9,
                            0x32, 0x83, 0xe8, 0x32, 0x89, 0x05, 0x48, 0x4e, 0x88, 0x00, 0x89, 0x0d, 0x44, 0x4e, 0x88, 0x00,
                            0x5d, 0xeb, 0x0d]),
            (0x02d326, vec![0xe9, 0xb1, 0x8a, 0x32, 0x00]),
            (0x02d334, vec![0x8b, 0x35, 0x48, 0x4e, 0x88, 0x00]),
            (0x355d15, vec![                              0x83, 0xf8, 0x32, 0x7d, 0x03, 0x83, 0xc0, 0x01, 0x83, 0xf9, 0x32,
                            0x7d, 0x03, 0x83, 0xc1, 0x01, 0xe9, 0xb8, 0x6f, 0xcd, 0xff]),
            (0x355ddc, vec![                                                                        0x8d, 0x4d, 0xf0, 0x51,
                            0xff, 0x35, 0x00, 0x4e, 0x88, 0x00, 0xff, 0x15, 0x50, 0xf6, 0x7f, 0x00, 0x8b, 0x45, 0xf0, 0x8b,
                            0x15, 0x44, 0x4e, 0x88, 0x00, 0xe9, 0x35, 0x75, 0xcd, 0xff])
        ];

        print!("Applying patch: camera skip glitch fix...");
        for (address, bytes) in patches.iter() {
            file[*address..*address+bytes.len()].copy_from_slice(&bytes);
        }
        println!(" Success!");
    }

    //Write out patched file
    match fs::write(&outfile_path, file) {
        Err(err) => {
            println!("File writing failed: {err}");
            return ExitCode::from(1);
        },
        Ok(_) => println!("Wrote file {}", outfile_path.to_string_lossy())
    };

    return ExitCode::from(0);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn replace_should_succeed() {
        let mut data: Vec::<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let find: Vec::<u8> = vec![3, 4, 5, 6];
        let repl: Vec::<u8> = vec![6, 5, 4, 3];
        let return_val = replace(&mut data, &find, &repl);
        assert_eq!(data, [1u8, 2, 6, 5, 4, 3, 7, 8, 9, 10]);
        assert!(return_val);
    }

    #[test]
    fn replace_should_fail() {
        let mut data: Vec::<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let find: Vec::<u8> = vec![6, 6, 6, 6];
        let repl: Vec::<u8> = vec![6, 5, 4, 3];
        let return_val = replace(&mut data, &find, &repl);
        assert!(!return_val);
    }

    #[test]
    fn replace_shouldnt_panic() {
        let mut data: Vec::<u8> = vec![1, 2];
        let find: Vec::<u8> = vec![3, 4, 5, 6];
        let repl: Vec::<u8> = vec![6, 5, 4, 3];
        let return_val = replace(&mut data, &find, &repl);
        assert!(!return_val);
    }
}
