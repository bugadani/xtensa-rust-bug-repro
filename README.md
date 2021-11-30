This repository contains a reproduction for [this issue](https://github.com/esp-rs/rust/issues/95)

# Instructions
build using:

`cargo +esp rustc --target=xtensa-esp32-espidf -Z build-std=std,panic_abort -Zbuild-std-features=panic_immediate_abort`

Expected output: project compiles fine.
Actual output:

```
❯ cargo +esp rustc --target=xtensa-esp32-espidf -Z build-std=std,panic_abort -Zbuild-std-features=panic_immediate_abort 
   Compiling foolib v0.1.0 (C:\_OpenSource\idftest\foolib)
LLVM ERROR: Not supported instr: <MCInst 271 <MCOperand Reg:42>>
error: could not compile `foolib`
```

## Additional information:
 - Offending code needs to be in foolib. Moving it to `src/lib.rs` results in a successful build.
 - Issue doesn't reproduce using `opt-level="z"`
 - Placing `#[inline(never)]` on `Enum::foo` results in a successful build.
