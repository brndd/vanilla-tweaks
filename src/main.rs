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
    print!("Applying patch: sound in background...");
    match replace(&mut file, widescreen_fov_find, widescreen_fov_repl) {
        true => println!(" Success!"),
        false => {
            println!(" FAILED!");
            return ExitCode::from(1);
        }
    }

    // Sound in background patch
    // FROM:
    // 007a4860 85 c9           TEST       ECX,ECX
    // 007a4862 74 2d           JZ         LAB_007a4891
    // 007a4864 8b 01           MOV        EAX,dword ptr [ECX]

    // TO:
    // 007a4860 b8 01 00        MOV        EAX,0x1
    //          00 00
    // 007a4865 c3              RET
    let sound_in_background_find: Vec::<u8> = vec![0x85, 0xc9, 0x74, 0x2d, 0x8b, 0x01, 0x85, 0xc0, 0x74, 0x14, 0x6a, 0x00, 0x6a, 0xfd, 0xa3,
                                                   0xb0, 0x55, 0xcf, 0x00, 0xe8, 0xc0, 0x92, 0x05, 0x00, 0xb8, 0x01, 0x00, 0x00, 0x00, 0xc3];
    let sound_in_background_repl: Vec::<u8> = vec![0xb8, 0x01, 0x00, 0x00, 0x00, 0xc3, 0x85, 0xc0, 0x74, 0x14, 0x6a, 0x00, 0x6a, 0xfd, 0xa3,
                                                   0xb0, 0x55, 0xcf, 0x00, 0xe8, 0xc0, 0x92, 0x05, 0x00, 0xb8, 0x01, 0x00, 0x00, 0x00, 0xc3];

    print!("Applying patch: sound in background...");
    match replace(&mut file, sound_in_background_find, sound_in_background_repl) {
        true => println!(" Success!"),
        false => {
            println!(" FAILED!");
            return ExitCode::from(1);
        }
    }


    //Write out patched file
    match fs::write(&outfile_path, file) {
        Err(err) => {
            println!("File writing failed: {err}");
            return ExitCode::from(1);
        },
        Ok(_) => println!("Wrote file  {}", outfile_path.to_string_lossy())
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
