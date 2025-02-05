aarch64-unknown-linux-gnu-c++ -c entry/boot_entry_point.S
aarch64-unknown-linux-gnu-c++ -c entry/early_print/uart_pl011.S

cargo build --target aarch64-unknown-linux-gnu --release --manifest-path kernel/Cargo.toml -Zbuild-std

aarch64-unknown-linux-gnu-ld -EL boot_entry_point.o uart_pl011.o kernel/target/aarch64-unknown-linux-gnu/release/libsaturn_kernel.a -T saturn.lds -o saturn.bin
aarch64-unknown-linux-gnu-objcopy -O binary -S saturn.bin saturn
