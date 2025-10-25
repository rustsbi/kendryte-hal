# PWM demo

Shows how to configure the PWM peripheral and drive a LED with varying brightness.

This example assumes a K230 board where IO19 is connected to a LED and routed to PWM channel 1.
Adjust pads as needed for your board.

Build this example with:

``` bash
rustup target install riscv64gc-unknown-none-elf
cargo build --target riscv64gc-unknown-none-elf --release -p pwm-demo
```
