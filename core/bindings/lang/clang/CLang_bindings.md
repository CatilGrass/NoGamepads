# No Gamepads - C Bindings

​	To run `NoGamepads` code on the `C/C++` platform, ensure your working environment has the Rust toolchain installed. After cloning the project locally, execute the following commands in the root directory:

```bash
# Compile Development Environment Runtime Library
cargo dev_build
cargo dev

# Compile Release Environment Runtime Library
cargo release_build
cargo release
```

​	After a period of time, Cargo will automatically compile the runtime libraries and header files, and open the output folder. (If it does not open, navigate to *./export/dev/* in the root directory to view the output.)

# Example Project

​	To view example code demonstrating the usage of the `NoGamepads` library, navigate to the folder *./core/bindings/lang/clang/binding_example/*.