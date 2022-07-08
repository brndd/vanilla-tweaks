//Patches the 1.12 WoW client for a wider FoV and sound in background.

//Files can be verified after patching with:
//cmp -l WoW_patched.exe WoW.exe | gawk '{printf "%08X %02X %02X\n", $1, strtonum(0$2), strtonum(0$3)}'

use std::fs;
use std::env;
use std::vec;
use std::process::ExitCode;
use std::path::Path;
use std::ffi::OsString;

/**
 * Replaces the first occurrence of find with replace, mutating haystack.
 * Returns true if a replacement occurred, false if not.
 */
fn replace(haystack: &mut Vec<u8>, find: Vec<u8>, replace: Vec<u8>) -> bool {
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

    haystack.splice(match_index..match_index+replace.len(), replace);
    return true;
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();

    //Check args and print usage if we get no parameter
    if args.len() < 2 {
        let cmd;
        match args.len() {
            0 => {
                cmd = "turtle-fixes.exe";
            }
            _ => {
                cmd = &args[0];
            }
        }
        println!("This program attempts to apply the following patches:
- Widescreen FoV patch
- Sound in background patch

Usage: {cmd} path/to/WoW.exe");
        return ExitCode::from(1);
    }

    //Open input file
    let file_path = &args[1];
    let mut file: std::vec::Vec<u8> = match fs::read(file_path) {
        Ok(file) => file,
        Err(err) => {
            println!("Unable to read file: {err}");
            return ExitCode::from(1);
        }
    };

    //Figure out output filename
    let file_path = Path::new(file_path);
    let file_stem = file_path.file_stem();
    let outfile_path = match file_stem {
        None => {
            println!("Filename stem is empty!");
            return ExitCode::from(1);
        },
        Some(stem) => {
            let ext: OsString = match file_path.extension() {
                Some(ext) => { 
                    let mut ostr = OsString::from(".");
                    ostr.push(ext);
                    ostr
                },
                None => OsString::from("")
            };
            let mut ostr = stem.to_os_string();
            ostr.push("_patched");
            ostr.push(ext);
            ostr
        }
    };

    /*
     * PATCHES PATCHES PATCHES PATCHES
     */

    // Widescreen FoV patch
    let widescreen_fov_find: Vec::<u8> = vec![0xDB, 0x0F, 0xC9, 0x3F, 0xE6, 0xF1, 0x47, 0x40, 0x00, 0x00];
    let widescreen_fov_repl: Vec::<u8> = vec![0x66, 0x66, 0xF6, 0x3F, 0xE6, 0xF1, 0x47, 0x40, 0x00, 0x00];
    print!("Applying patch: widescreen FoV fix...");
    match replace(&mut file, widescreen_fov_find, widescreen_fov_repl) {
        true => println!(" Success!"),
        false => {
            println!(" FAILED!");
            return ExitCode::from(1);
        }
    }

    // Sound in background patch
    let sound_in_background_find: Vec::<u8> = vec![0x85, 0xc9, 0x74, 0x2d, 0x8b, 0x01, 0x85, 0xc0, 0x74, 0x14, 0x6a, 0x00, 0x6a, 0xfd, 0xa3, 0xb0, 0x55, 0xcf, 0x00, 0xe8, 0xc0, 0x92, 0x05, 0x00, 0xb8, 0x01, 0x00, 0x00, 0x00, 0xc3, 0x6a, 0x01, 0x6a, 0xfd, 0xc7, 0x05, 0xb0, 0x55, 0xcf, 0x00, 0x00, 0x00, 0x00, 0x00, 0xe8, 0xa7, 0x92, 0x05, 0x00, 0xb8, 0x01, 0x00, 0x00, 0x00, 0xc3];
    let sound_in_background_repl: Vec::<u8> = vec![0x85, 0xc9, 0x74, 0x2d, 0x8b, 0x01, 0x85, 0xc0, 0x74, 0x27, 0x6a, 0x00, 0x6a, 0xfd, 0xa3, 0xb0, 0x55, 0xcf, 0x00, 0xe8, 0xc0, 0x92, 0x05, 0x00, 0xb8, 0x01, 0x00, 0x00, 0x00, 0xc3, 0x6a, 0x01, 0x6a, 0xfd, 0xc7, 0x05, 0xb0, 0x55, 0xcf, 0x00, 0x00, 0x00, 0x00, 0x00, 0xe8, 0xa7, 0x92, 0x05, 0x00, 0xb8, 0x01, 0x00, 0x00, 0x00, 0xc3];

    print!("Applying patch: sound in background...");
    match replace(&mut file, sound_in_background_find, sound_in_background_repl) {
        true => println!(" Success!"),
        false => {
            println!(" FAILED!");
            return ExitCode::from(1);
        }
    }

    // Quickloot key reverse patch (hold shift to manual loot)
    let quickloot_reverse_find_fish: Vec::<u8> = vec![0x50, 0xba, 0x48, 0xf9, 0x85, 0x00, 0xb9, 0x10, 0x00, 0x00, 0x00, 0xe8, 0xd0, 0xfd, 0xe6, 0xff, 0x85, 0xc0, 0x74, 0x10, 0x8b, 0x4d, 0x08, 0x85, 0xc9, 0x0f, 0x95, 0xc1, 0x51, 0x8b, 0xc8, 0xe8];
    let quickloot_reverse_repl_fish: Vec::<u8> = vec![0x50, 0xba, 0x48, 0xf9, 0x85, 0x00, 0xb9, 0x10, 0x00, 0x00, 0x00, 0xe8, 0xd0, 0xfd, 0xe6, 0xff, 0x85, 0xc0, 0x74, 0x10, 0x8b, 0x4d, 0x08, 0x85, 0xc9, 0x0f, 0x94, 0xc1, 0x51, 0x8b, 0xc8, 0xe8];
    
    //let quickloot_reverse_find_unkn: Vec::<u8> = vec![0x85, 0xd2, 0x0f, 0x95, 0xc0, 0x8b, 0xce, 0x50, 0xe8, 0xf3, 0x87, 0x00, 0x00, 0x8b, 0xce, 0xe8, 0x8c, 0x8c, 0x03, 0x00, 0x8b, 0xce, 0x50, 0x57, 0xe8, 0xb3, 0x84, 0x00, 0x00, 0x5f, 0x5e, 0x5d];
    //let quickloot_reverse_repl_unkn: Vec::<u8> = vec![0x85, 0xd2, 0x0f, 0x94, 0xc0, 0x8b, 0xce, 0x50, 0xe8, 0xf3, 0x87, 0x00, 0x00, 0x8b, 0xce, 0xe8, 0x8c, 0x8c, 0x03, 0x00, 0x8b, 0xce, 0x50, 0x57, 0xe8, 0xb3, 0x84, 0x00, 0x00, 0x5f, 0x5e, 0x5d];
    
    let quickloot_reverse_find_containeritem: Vec::<u8> = vec![0x84, 0xa4, 0x00, 0x00, 0x00, 0x33, 0xc9, 0xe8, 0x44, 0x1c, 0xe3, 0xff, 0x48, 0xf7, 0xd8, 0x1a, 0xc0, 0xfe, 0xc0, 0x8b, 0xce, 0x50, 0xe8, 0xa5, 0x17, 0xff, 0xff, 0x8b, 0x57, 0x08, 0x8b, 0x02];
    let quickloot_reverse_repl_containeritem: Vec::<u8> = vec![0x84, 0xa4, 0x00, 0x00, 0x00, 0x33, 0xc9, 0xe8, 0x44, 0x1c, 0xe3, 0xff, 0x90, 0xf7, 0xd8, 0x1a, 0xc0, 0xfe, 0xc0, 0x8b, 0xce, 0x50, 0xe8, 0xa5, 0x17, 0xff, 0xff, 0x8b, 0x57, 0x08, 0x8b, 0x02];

    let quickloot_reverse_find_corpse: Vec::<u8> = vec![0x04, 0x3b, 0x15, 0x9c, 0xda, 0xc4, 0x00, 0x75, 0x10, 0x8b, 0x53, 0x08, 0x85, 0xd2, 0x0f, 0x95, 0xc0, 0x8b, 0xce, 0x50, 0xe8, 0x77, 0x34, 0xfd, 0xff, 0x8b, 0xce, 0xe8, 0x10, 0x39, 0x00, 0x00];
    let quickloot_reverse_repl_corpse: Vec::<u8> = vec![0x04, 0x3b, 0x15, 0x9c, 0xda, 0xc4, 0x00, 0x75, 0x10, 0x8b, 0x53, 0x08, 0x85, 0xd2, 0x0f, 0x94, 0xc0, 0x8b, 0xce, 0x50, 0xe8, 0x77, 0x34, 0xfd, 0xff, 0x8b, 0xce, 0xe8, 0x10, 0x39, 0x00, 0x00];
    
    print!("Applying patch: quickloot reverse (world objects)...");
    match replace(&mut file, quickloot_reverse_find_fish, quickloot_reverse_repl_fish) {
        true => println!(" Success!"),
        false => {
            println!(" FAILED!");
            return ExitCode::from(1);
        }
    }

    print!("Applying patch: quickloot reverse (container items)...");
    match replace(&mut file, quickloot_reverse_find_containeritem, quickloot_reverse_repl_containeritem) {
        true => println!(" Success!"),
        false => {
            println!(" FAILED!");
            return ExitCode::from(1);
        }
    }

    print!("Applying patch: quickloot reverse (corpses)...");
    match replace(&mut file, quickloot_reverse_find_corpse, quickloot_reverse_repl_corpse) {
        true => println!(" Success!"),
        false => {
            println!(" FAILED!");
            return ExitCode::from(1);
        }
    }

    // print!("Applying patch: quickloot reverse (unkn)...");
    // match replace(&mut file, quickloot_reverse_find_unkn, quickloot_reverse_repl_unkn){
    //     true => println!(" Success!"),
    //     false => {
    //         println!(" FAILED!");
    //         return ExitCode::from(1);
    //     }
    // }

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
        let return_val = replace(&mut data, find, repl);
        assert_eq!(data, [1u8, 2, 6, 5, 4, 3, 7, 8, 9, 10]);
        assert!(return_val);
    }

    #[test]
    fn replace_should_fail() {
        let mut data: Vec::<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let find: Vec::<u8> = vec![6, 6, 6, 6];
        let repl: Vec::<u8> = vec![6, 5, 4, 3];
        let return_val = replace(&mut data, find, repl);
        assert!(!return_val);
    }

    #[test]
    fn replace_shouldnt_panic() {
        let mut data: Vec::<u8> = vec![1, 2];
        let find: Vec::<u8> = vec![3, 4, 5, 6];
        let repl: Vec::<u8> = vec![6, 5, 4, 3];
        let return_val = replace(&mut data, find, repl);
        assert!(!return_val);
    }
}
