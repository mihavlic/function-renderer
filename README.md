# Function Renderer
This project implements the dual contouring algorithm in Vulkan compute shaders. It includes an Egui interface for manipulating the function with the ability to export its mesh as an .stl file.

![Screenshot of application](screenshot.png)

# Building
You need somewhat recent graphics drivers, shaderc, Rust, and a C++ compiler

## Archlinux
```sh
sudo pacman -S shaderc rust gcc
git clone https://github.com/mihavlic/function-renderer.git
cd function-renderer
cargo run
```

## Windows
install the [Lunarg SDK](https://vulkan.lunarg.com/), [Rust](https://www.rust-lang.org/tools/install), and get MSVC from the [Windows SDK](https://developer.microsoft.com/en-us/windows/downloads/windows-sdk/)
```sh
git clone https://github.com/mihavlic/function-renderer.git
cd function-renderer
cargo run
```
