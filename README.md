# Proton-Caller
A program to automatically run programs using Steam's Proton

[Usage](https://github.com/caverym/Proton-Caller#usage)

Please create an issue if you want added features or have an issue.

https://aur.archlinux.org/packages/proton-caller/

## Problem Reporting:
Please create an issue on the [Github](https://github.com/caverym/Proton-Caller) page which lists: system, kernel version, game, shell, and if it is or isn't a Steam game – provide how you had installed it and where it is installed. Additionally provide screenshots of the shell. Try many methods to get it to work and describe what you did in your issue.


## Usage:

-c \- custom mode<br>
-h \- help information<br>
help \- same as -h
```
proton-call 5 SpaceEngine.exe
```

```
proton-call -c "/home/avery/.steam/steam/steamapps/common/Proton 5.13" SpaceEngine.exe
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
or:
```
git clone https://github.com/caverym/Proton-Caller.git
cd Proton-Caller
make
sudo make install
```

### Space Engine example:
   To make a .desktop launcher create a shell script such as `/usr/bin/SpaceEngine`:
   ```
   #!/usr/bin/env zsh
   export STEAM_COMPAT_DATA_PATH=/home/avery/proton
   cd /home/avery/proton/pfx/drive_c/SpaceEngine/system
   proton-call 5 SpaceEngine.exe
   ```
   Point it at the executable, then run it using proton-call.
   ```
   chmod +x /usr/bin/SpaceEngine
   ```
   
   Then create a .desktop file which points at `/usr/bin/SpaceEngine`
   ```
   Exec=SpaceEngine
   Terminal=false
   Type=Application
   Categories=Game;;
   ```
