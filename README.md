# nasm_proj

A simple project manager for windows

## Installing

To install, get mingw, nasm and git, put the exe, install.bat and nasm_proj.json (the one in the root) in their own folder and run install.bat.
After running install.bat, you can delete it and add the \bin folder onto the path

## C++ Linking Problems

If you have an issue like this with c++ code like this:
```
running clang build\\main.cpp.o -o hello_world.exe
ld.lld: error: undefined symbol: ...
```
You need to link with the c++ standard library by changing this line in your nasm_proj.json from
```
    "link": "clang $obj -o $proj.exe"
```
to
```
    "link": "clang++ $obj -o $proj.exe"
```
If the issue is in \lib linking make sure that you include .hpp and compile files and make sure that you add the source file in your nasm_proj json like this:
```
    "lib": []
```
```
    "lib": ["<filename>"]
```