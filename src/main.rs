//Patches some QoL stuff into the old 1.12.1 WoW client.

//Files can be verified after patching with:
//cmp -l WoW_patched.exe WoW.exe | gawk '{printf "%08X %02X %02X\n", $1, strtonum(0$2), strtonum(0$3)}'

use std::fs;
use std::process::ExitCode;
use std::ffi::OsString;
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
- Farclip (terrain render distance) maximum value change
- Frilldistance (grass render distance) change
- Quickloot by default patch (hold shift for manual loot)")]
struct Args {
    /// Path to WoW.exe.
    #[clap(value_parser)]
    infile: String,

    /// Filename of the output file.
    #[clap(short, default_value_t = String::from("WoW_patched.exe"), value_parser)]
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
    no_quickloot: bool

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

    // Quickloot key reverse patch (hold shift to manual loot)
    if !args.no_quickloot {
        const QUICKLOOT_CONTAINER_OFFSET: usize = 0x1EDCAC;
        const QUICKLOOT_CONTAINER_BYTES: [u8; 1] = [0x90];
        const QUICKLOOT_WORLDOBJECT_OFFSET: usize = 0x1F869A;
        const QUICKLOOT_WORLDOBJECT_BYTES: [u8; 1] = [0x94];
        const QUICKLOOT_CORPSE_OFFSET: usize = 0x20BFDF;
        const QUICKLOOT_CORPSE_BYTES: [u8; 1] = [0x94];
        print!("Applying patch: quickloot reverse (container items)...");
        file[QUICKLOOT_CONTAINER_OFFSET..QUICKLOOT_CONTAINER_OFFSET+QUICKLOOT_CONTAINER_BYTES.len()].copy_from_slice(&QUICKLOOT_CONTAINER_BYTES);
        println!(" Success!");

        print!("Applying patch: quickloot reverse (world objects)...");
        file[QUICKLOOT_WORLDOBJECT_OFFSET..QUICKLOOT_WORLDOBJECT_OFFSET+QUICKLOOT_WORLDOBJECT_BYTES.len()].copy_from_slice(&QUICKLOOT_WORLDOBJECT_BYTES);
        println!(" Success!");

        print!("Applying patch: quickloot reverse (corpses)...");
        file[QUICKLOOT_CORPSE_OFFSET..QUICKLOOT_CORPSE_OFFSET+QUICKLOOT_CORPSE_BYTES.len()].copy_from_slice(&QUICKLOOT_CORPSE_BYTES);
        println!(" Success!");

        //let quickloot_reverse_find_unkn: Vec::<u8> = vec![0x85, 0xd2, 0x0f, 0x95, 0xc0, 0x8b, 0xce, 0x50, 0xe8, 0xf3, 0x87, 0x00, 0x00, 0x8b, 0xce, 0xe8, 0x8c, 0x8c, 0x03, 0x00, 0x8b, 0xce, 0x50, 0x57, 0xe8, 0xb3, 0x84, 0x00, 0x00, 0x5f, 0x5e, 0x5d];
        //let quickloot_reverse_repl_unkn: Vec::<u8> = vec![0x85, 0xd2, 0x0f, 0x94, 0xc0, 0x8b, 0xce, 0x50, 0xe8, 0xf3, 0x87, 0x00, 0x00, 0x8b, 0xce, 0xe8, 0x8c, 0x8c, 0x03, 0x00, 0x8b, 0xce, 0x50, 0x57, 0xe8, 0xb3, 0x84, 0x00, 0x00, 0x5f, 0x5e, 0x5d];
        
        // print!("Applying patch: quickloot reverse (unkn)...");
        // match replace(&mut file, quickloot_reverse_find_unkn, quickloot_reverse_repl_unkn){
        //     true => println!(" Success!"),
        //     false => {
        //         println!(" FAILED!");
        //         return ExitCode::from(1);
        //     }
        // }
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
