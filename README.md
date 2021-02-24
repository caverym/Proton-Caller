# Proton-Caller
Run any Windows program through [Valve's Proton](https://github.com/ValveSoftware/Proton).

[Usage](https://github.com/caverym/Proton-Caller#usage)

Please create an issue if you want added features or have an issue.

\> [FAQ](https://github.com/caverym/Proton-Caller/wiki/FAQ)

https://aur.archlinux.org/packages/proton-caller/

## Problem Reporting:
Please create an issue on the [Github](https://github.com/caverym/Proton-Caller) page which lists: system, kernel version, game, shell, and if it is or isn't a Steam game – provide how you had installed it and where it is installed. Additionally provide screenshots of the shell. Try many methods to get it to work and describe what you did in your issue.
Warning: if you're not using a release build, **use a release build**


## Usage:

```
Usage: proton-call VERSION PROGRAM
   or: basename OPTION PATH PROGRAM
Execute PROGRAM with Proton VERSION
If specified, run proton PATH

  -c, --custom PATH       use proton from PATH
  -h, --help              display this help message
  -s, --setup             display setup information
  -v, --version           display version information
```

```
proton-call 5.13 SpaceEngine.exe
```

```
proton-call -c ./Proton\ 5.13/ SpaceEngine.exe
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
or: (Requires Rust)
```
git clone https://github.com/caverym/Proton-Caller.git
cd Proton-Caller
cargo b --release --lock
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
