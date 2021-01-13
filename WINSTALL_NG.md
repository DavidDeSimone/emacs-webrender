need rust
install rustup, set target to be x86_64-pc-windows-msvc AND x86_64-pc-windows-gnu. Add path to cargo to msys2 PATH

need msvc

need everything listed in nt/INSTALL, including running mingw

need clang
 clang needs to be in C://msys2/mingw64/...

remacs-lib needs to be a dll, and that needs to be moved to places for dynamic linking
 TODO keep remacs-lib static on linux/osx

anything that links to rust needs either remacs-lib, or to be a dll