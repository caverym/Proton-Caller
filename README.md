# Proton-Caller
Run any Windows program through [Valve's Proton](https://github.com/ValveSoftware/Proton).

[Usage](https://github.com/caverym/Proton-Caller#usage)

Please create an issue if you want added features or have an issue.

\> [FAQ](https://github.com/caverym/Proton-Caller/wiki/FAQ)

https://aur.archlinux.org/packages/proton-caller/

## Problem Reporting:
Please create an issue on the [Github](https://github.com/caverym/Proton-Caller) page which lists: system, kernel version, game, shell, and if it is or isn't a Steam game – provide how you had installed it and where it is installed. Additionally provide screenshots of the shell. Try many methods to get it to work and describe what you did in your issue.

### Warning: if you are not using a release, use a release.

## Usage:

Defaults to the latest version of Proton.
```
proton-call -r foo.exe
```

Defaults to the latest verson of Proton, all extra arguments passed to the executable.
```
proton-call -r foo.exe --flags --for program
```

Uses specified version of Proton, any extra arguments will be passed to the executable.
```
proton-call -p 5.13 -r foor.exe
```

Uses custom version of Proton, give the past to directory, not the Proton executable itself.
```
proton-call -c '/path/to/Proton version' -r foo.exe
```

## Config:
Configuration files are extremely simple: `~/.config/proton.conf`
   Set your own path to `data` (any empty directory) and `common` (steam's common dirrectory)
```
data = "/home/avery/Documents/Proton/env/"
common = "/home/avery/.steam/steam/steamapps/common/"
```

## Install:

To install `proton-call`
```
yay -S proton-caller
 ``` 

or: (with makepkg)

```
git clone https://aur.archlinux.org/proton-caller.git
cd proton-caller
makepkg -si
```

or: (Requires Rust to build)
```
git clone https://github.com/caverym/Proton-Caller.git
cd Proton-Caller
cargo b --release --locked
sudo install -Dm 755 target/release/proton-call /usr/bin/proton-call 
```

### Space Engine example:
   Make a .desktop launcher. [example file](Space%20Engine.desktop)
   
```
[Desktop Entry]
Type=Application
Name=Space Engine
Comment=Space Engine
Exec=proton-call 5.13 SpaceEngine.exe
Path=/home/avery/Documents/games/SpaceEngine/system
Terminal=false
StartupNotify=false
```
