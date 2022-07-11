![Build and tests](https://github.com/brndd/vanilla-tweaks/actions/workflows/rust.yml/badge.svg)

# vanilla-tweaks

These are some custom patches for the old 1.12.1 World of Warcraft client, which lacks many of the conveniences of more modern clients.

## Current patches

- Widescreen FoV fix
- Sound in background patch
- Sound channel count default increase
  - By default the game only uses 12 software sound channels and up to 12 hardware sound channels (modern sound devices usually only have 1). This causes some sounds to not play in group content. 
  - The default in TBC is 32. The default in newer expansions is 64.
  - This can also be set with `/console SoundSoftwareChannels 128`, but this patcher changes the default value so that it survives Config.wtf deletions.
  - If you run into performance problems in group content where many sounds are playing, try decreasing this to 64. With modern hardware this is unlikely to be an issue though.
- Farclip (max render distance) increase
  - Farclip is changed with `/console farclip 1000` (777 is the default maximum)
  - This patch allows up to 10000, but this may cause crashes. Enabling the Large Address Aware patch (enabled by default) may help reduce crashing.
- Frilldistance (max grass render distance) increase
  - You may want to customize the value used here if you use a very high frilldensity in order to maintain performance. The default (300) works fine on my machine with a relatively low frilldensity (64), but causes FPS to drop below 144 with high frilldensity.
  - Frill density (grass density) is changed with `/console frilldensity 100`. 256 is the max value (unchanged by patcher as it is already very dense).
- Quickloot reverse patch (hold shift to manual loot)
- Nameplate range change.
  - Increased to 41 yards to match the maximum value in Classic and TBC Classic. 20 yards is the default value.
- Large Address Aware patch.
  - This allows the game to use more than 2GB of memory by setting a flag in the executable header. See https://codekabinett.com/rdumps.php?Lang=2&targetDoc=largeaddressaware-msaccess-exe for more information.
  - If you experience inexplicable crashes, try disabling this patch, and if you manage to reproduce them let me know via an issue. The client *should* have no issues being Large Address Aware, but you never know.

## Usage

Pass the path to WoW.exe as a parameter (most easily done by dragging WoW.exe on top of the patcher in Windows Explorer). The patcher creates WoW_patched.exe next to the original binary. Run the game from that exe (or replace the original exe with it, I don't care).

There are command-line options available to customize the values of and disable any tweaks you don't want. Run the program with the `--help` parameter to get a list of them.
