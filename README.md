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

Download the release matching your operating system from the [releases page](https://github.com/brndd/vanilla-tweaks/releases). Extract the executable from the archive into your WoW directory.

### Simple usage (Windows)

Open your WoW directory and drag WoW.exe on top of vanilla_tweaks.exe. This will create a new file called WoW_tweaked.exe, which has all the patches applied with their default settings. You should then start the game from WoW_tweaked.exe instead. You may also rename your original WoW.exe to something else and then rename WoW_tweaked.exe to WoW.exe if you prefer. However, do note that this may cause issues if the server you are playing on uses the game's update system to update the game.

### Customizing the settings

To customize the values changed by the patches, or to disable some patches, you must run vanilla-tweaks from the command line.

First, open a command prompt and navigate to your WoW directory. The easiest way to do this on Windows is to click File -> Open Windows PowerShell. On Mac, you can control-click the folder in the path bar and select Open in Terminal. On Linux, you can probably right-click on an empty space in the directory and open a terminal from there, but as a Linux user you probably know how to use the `cd` command anyway.

With your terminal open in your WoW directory, you may then run vanilla-tweaks with custom parameters like this:

```sh
./vanilla-tweaks --no-sound-in-background --nameplatedistance 60 WoW.exe
```

The example here disables the sound in background patch and sets nameplate distance to 60 feet rather than 40.

To see a full list of the available options, you may use the `--help` parameter:

```sh
./vanilla-tweaks --help
```

## Launch scripts

(Pull requests to add scripts for other platforms here are welcome!)

### Linux/Lutris

Here is an example Lutris launch script that clears the game's cache folder and regenerates the patched executable if WoW.exe has changed since the last time the patches were applied (e.g. the server shipped an update to the game files).

Make sure to modify the game path to match your installation. You can then enable the script by setting it in Lutris via Configure > System Options > Pre-launch script (make sure "Wait for pre-launch script completion" is active).

```bash
#!/bin/bash

cd /media/ssd0/games/turtle-wow/drive_c/turtle_client_116/
#Clear cache
rm -f /media/ssd0/games/turtle-wow/drive_c/turtle_client_116/WDB/*

#Check hash of WoW.exe to see if it has changed
if ! sha256sum --status --check WoW.exe.sha256; then
    echo "WoW.exe has changed, updating WoW.exe.sha256 and WoW_tweaked.exe"
    sha256sum WoW.exe > WoW.exe.sha256
    ./vanilla-tweaks WoW.exe
fi
```

