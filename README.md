![Build and tests](https://github.com/brndd/vanilla-tweaks/actions/workflows/rust.yml/badge.svg)
 
# vanilla-tweaks

These are some custom patches for the old 1.12.1 World of Warcraft client, which lacks many of the conveniences of more modern clients.

## Current patches

- **Widescreen FoV fix**
- **Sound in background patch**
- **Sound channel count default increase**
  - By default the game only uses 12 channels to play sound. This is essentially the number of sounds that can play at the same time, so the low default value means many sound effects in group content do not play.
  - Increased to 64 by default. The default in TBC is 32. The default in newer expansions is 64. 
  - This can also be set with `/console SoundSoftwareChannels 64`, but this patcher changes the default value so that it survives Config.wtf deletions.
  - Values above 64 have been reported to cause crashes. If you run into performance issues, try decreasing this setting further.
- **Farclip (max render distance) increase**
  - Farclip is changed with `/console farclip 1000` (777 is the default maximum)
  - This patch allows up to 10000, but this may cause crashes. Enabling the Large Address Aware patch (enabled by default) may help reduce crashing.
- **Frilldistance (max grass render distance) increase**
  - You may want to customize the value used here if you use a very high frilldensity in order to maintain performance. The default (300) works fine on my machine with a relatively low frilldensity (64), but causes FPS to drop below 144 with high frilldensity.
  - Frill density (grass density) is changed with `/console frilldensity 100`. 256 is the max value (unchanged by patcher as it is already very dense).
- **Quickloot reverse patch (hold shift to manual loot)**
  - This should work reliably for all types of looting. Please make an issue if it doesn't (e.g. if it occasionally fails to loot).
- **Nameplate range change**
  - Increased to 41 yards to match the maximum value in Classic and TBC Classic. 20 yards is the default value.
- **CameraDistanceMax limit increase**
  - Allows you to increase the CameraDistanceMax limit. This only changes the max value settable; the actual max camera distance can be changed with /console CameraDistanceMax <value>.
  - Unchanged by default. Default maximum value is 50.
- **Large Address Aware patch**
  - This allows the game to use more than 2GB of memory by setting a flag in the executable header. See https://codekabinett.com/rdumps.php?Lang=2&targetDoc=largeaddressaware-msaccess-exe for more information.
  - If you experience inexplicable crashes, try disabling this patch, and if you manage to reproduce them let me know via an issue. The client *should* have no issues being Large Address Aware, but you never know.

## Usage
Download this tool from [Releases](https://github.com/brndd/vanilla-tweaks/releases) that matches your operating system. Extract the vanilla-tweaks executable to your WoW directory, and:

### Simple Method: Apply the Default Configuration
With your file browser, drag your WoW.exe ontop of the Vanilla-Tweaks executable. WoW_tweaked.exe will be made, and you should use this new exe from now on. If you prefer, it is okay to rename/backup your original WoW.exe to something/somewhere else, and rename this new tweaked .exe to be WoW.exe.

### Advanced Method: Customize with the Command Line
With your terminal changed to the directory of your wow folder, pass the WoW.exe to modify as the final paramater to vanilla-tweaks. Add parameters to disable and reconfig tweaks to your desire, and any omitted parameters will use the defaults. For example:

Windows:
```
vanilla-tweaks.exe --no-sound-in-background --nameplatedistance 36 WoW.exe
```
Mac/Linux:
```sh
./vanilla-tweaks --no-sound-in-background --nameplatedistance 36 WoW.exe
```

Use ```--help``` for more info on the available parameters.
