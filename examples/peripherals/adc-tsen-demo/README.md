UART peripheral demo

Build this example with:

```
rustup target install riscv32imac-unknown-none-elf
cargo build --target riscv32imac-unknown-none-elf --release -p adc-tsen-demo
rust-objcopy --binary-architecture=riscv32 --strip-all -O binary ./target/riscv32imac-unknown-none-elf/release/adc-tsen-demo ./target/riscv32imac-unknown-none-elf/release/adc-tsen-demo.bin
```

cargo run --target riscv32imac-unknown-none-elf --release -p adc-tsen-demo -- --port PORT_NAME
