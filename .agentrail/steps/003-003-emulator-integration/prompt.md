Wire the COR24 emulator into the REPL panel so that APL input is evaluated by the real interpreter running on the emulator.

1. Update build.rs to pre-assemble the APL interpreter (../sw-cor24-apl/build/apl.s) at build time using cor24_emulator::Assembler. Write the binary to OUT_DIR/apl.bin. No label extraction needed (APL binary is a self-contained program).

2. Create src/config.rs with the pre-assembled APL binary (include_bytes) — same pattern as web-sw-cor24-pcode's config.rs but simpler (just the binary, no labels).

3. Modify src/repl.rs to integrate EmulatorCore:
   - Add EmulatorCore, StopReason, VecDeque fields to ReplPanel struct
   - Add Msg::Init to load the APL binary and start running
   - Add Msg::Tick for batch execution (50k instructions per 25ms tick)
   - On Enter: push input bytes + newline into uart_rx_queue instead of echoing
   - Add feed_uart_bytes() to drain rx queue checking UART status register 0xFF0101 bit 0
   - Add collect_uart() to read emulator UART output and append to output buffer
   - The APL interpreter uses getchar/putchar mapped to UART — output appears via UART TX
   - Auto-start running on Init (not paused like the debugger)
   - Keep the blinking cursor

4. Update src/lib.rs to add mod config.

5. Verify trunk build compiles successfully.