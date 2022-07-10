![Build and tests](https://github.com/brndd/vanilla-tweaks/actions/workflows/rust.yml/badge.svg)

# vanilla-tweaks

These are some custom patches for the old 1.12.1 World of Warcraft client, which lacks many of the conveniences of more modern clients.

## Current patches

- Widescreen FoV fix
- Sound in background
- Farclip (max render distance) increase
  - Farclip is changed with `/console farclip 1000` (777 is the default maximum)
  - This patch allows up to 10000, but this may cause crashes. Patching the client to be large address aware may help with crashing.
- Frilldistance (max grass render distance) increase
  - You may want to customize the value used here if you use a very high frilldensity in order to maintain performance. The default (300) works fine on my machine with a relatively low frilldensity (64), but causes FPS to drop below 144 with high frilldensity.
  - Frill density (grass density) is changed with `/console frilldensity 100`. 256 is the max value (unchanged by patcher as it is already very dense).
- Quickloot reverse patch (hold shift to manual loot)
- Nameplate range change. Increased to 41 yards to match the maximum value in Classic and TBC Classic. 20 yards is the default value.

## Usage

Pass the path to WoW.exe as a parameter. The patcher creates WoW_patched.exe next to the original binary. Run the game from that exe (or replace the original exe with it, I don't care).

There are command-line options available to customize the values of and disable any tweaks you don't want. Run the program with the `--help` parameter to get a list of them.
