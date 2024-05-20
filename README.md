# nasm_proj

A simple project manager for windows

## Installing

To install, get mingw and nasm and put the exe on the path

## C++ Linking Problems

If you have an issue like this with c++ code:
```
running clang build\\main.cpp.o -o hello_world.exe
ld.lld: error: undefined symbol: ...
```
Change this line in your nasm_proj.json from
```
    "link": "clang $obj -o $proj.exe"
```
to
```
    "link": "clang++ $obj -o $proj.exe"
```
to use clang++ to link with the c++ standard library