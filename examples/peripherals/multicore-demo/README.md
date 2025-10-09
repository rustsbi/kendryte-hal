# multicore-demo (K230)

Minimal example that brings up the secondary core (hart1) on Kendryte K230 using `kendryte-rt`.

This is the simplified version: single launch sequence, no multi-attempt logic, no instruction dump.

## What it Shows

Hart1 liveness is demonstrated via two atomic "mailboxes":

* `HART1_FLAG`: cycles through four distinct magic values `[0xCAFE_BABE, 0xDEAD_BEEF, 0x1234_5678, 0x0BAD_F00D]`
* `HART1_TICKS`: periodic tick counter updated by hart1

Hart0 periodically reads and prints both. A `*` after a field indicates it changed since the previous print.

A configurable observation window (default 10 seconds with a progress bar) lets you attach a serial console and view initial values before hart1 is released.

## File Overview (`src/main.rs`)

* Constants: reset vector / control register addresses, startup delay, magic sequence
* `hart1_reset_trap`: naked assembly trampoline + private stack
* `hart1_main`: loops updating the tick and the rotating magic flag
* `start_hart1()`: one-shot reset vector program + reset toggle + fences
* Hart0 main loop: throttled logging of its own counter plus hart1 mailboxes

Removed compared to earlier diagnostic version: multi-attempt strategy, first 4 instruction word dump, repeated control register snapshots.

## Concurrency & Safety

* Shared state uses `AtomicU32` to avoid undefined data races
* After writing the reset vector and releasing hart1, `fence.i` and `fence iorw, iorw` ensure visibility and instruction fetch coherency
* Inline `asm!` is confined to `unsafe` blocks per 2024 edition requirements

## Build

```bash
cargo build -p multicore-demo --target riscv64gc-unknown-none-elf --release
```

## Generate Image (using the workspace xtask helpers)

```bash
# Direct: ELF -> image
cargo xtask elf2img -i target/riscv64gc-unknown-none-elf/release/multicore-demo -o target/multicore-demo.img

# Or: ELF -> raw bin -> image
cargo objcopy -p multicore-demo --release --target riscv64gc-unknown-none-elf -- -O binary target/riscv64gc-unknown-none-elf/release/multicore-demo.bin
cargo xtask gen-image -i target/riscv64gc-unknown-none-elf/release/multicore-demo.bin -o target/multicore-demo.img
```

## Example UART Output

```text
=== multicore-demo (K230) ===
hart0: starting bring-up sequence
pre-start mailbox: hart1_flag=0x00000000 hart1_ticks=0
observing 10s before launching hart1 (progress 0%->100%)
[01/10s] [#...................]  10%
... (progress lines elided) ...
[10/10s] [####################] 100%
startup window complete -> launching hart1
launching hart1 rstvec=0x80301234
hart1 initial flag=0xcafebabe
hart0 cnt=100000 hart1_flag=0xcafebabe* hart1_ticks=50000*
hart0 cnt=200000 hart1_flag=0xdeadbeef* hart1_ticks=100000*
hart0 cnt=300000 hart1_flag=0x12345678* hart1_ticks=150000*
hart0 cnt=400000 hart1_flag=0x0badf00d* hart1_ticks=200000*
...
```

Exact addresses and cadence depend on clock configuration and linker layout.

## Tuning

* Startup wait length: adjust `STARTUP_DELAY_SECS`
* Hart0 print interval: change the `counter % 100_000` condition
* Hart1 magic / tick cadence: modify `HART1_MAGIC` or the `tick % 50_000` threshold
* Remove the progress bar: delete the pre-launch loop
* Slow output further: increase the busy-wait delay or add additional `delay()` calls

## Possible Extensions

* Add a ready/handshake barrier before hart0 proceeds with more init
* Replace busy-wait nops with a low-power `wfi` strategy
* Implement a simple inter-hart message ring buffer
* Switch from magic pattern to a strictly monotonic counter for symmetry

## Disclaimer

Register addresses and the reset sequence are based on reference material and experiments. Consult the official K230 TRM before relying on this for production.
