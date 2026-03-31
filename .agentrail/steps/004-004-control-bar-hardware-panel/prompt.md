Add control bar (Reset button, demo dropdown, file upload) and floating hardware panel (S2 switch, D2 LED, UART TX/RX display) to the APL environment.

1. Create src/control_bar.rs — a Yew Component with:
   - Reset button: sends a callback to reset the emulator
   - Demo dropdown: populated with embedded demo .apl programs
   - File upload button: hidden input[type=file] + styled button for .apl/.txt files
   - Properties: callbacks for on_reset, on_demo(String), on_upload(String)
   
2. Create src/hardware.rs — a floating Yew Component with:
   - S2 switch toggle (visual, click to toggle, maps to emulator switch)
   - D2 LED indicator (reflects emulator LED register 0xFF0000, green=on, dark=off)
   - UART TX/RX last-byte display (shows -- initially)
   - Properties: led_on: bool, s2_on: bool, last_tx: Option<u8>, last_rx: Option<u8>, on_s2_toggle callback

3. Create src/demos.rs — demo registry:
   - Each demo is a name + description + APL source text (not pre-assembled, since APL programs are fed as text via UART to the running interpreter)
   - Embed demo .apl files converted to ASCII syntax (the COR24 APL uses rho/iota/take/drop not Unicode glyphs)
   - Include 4-6 demos from sw-cor24-apl/samples/ converted to ASCII syntax

4. Update src/repl.rs:
   - Add Msg variants: Reset, LoadDemo(String), UploadProgram(String), ToggleS2
   - Reset: call load_apl_binary() to restart
   - LoadDemo/UploadProgram: feed text line-by-line into uart_rx_queue
   - ToggleS2: toggle the S2 switch state
   - Track led_on by reading 0xFF0000 each tick
   - Track last_tx/last_rx bytes

5. Update src/lib.rs:
   - Add mod control_bar, mod hardware, mod demos
   - Restructure App to include ControlBar above ReplPanel
   - Add HardwarePanel floating inside main content area
   - Wire callbacks between components using Yew properties/callbacks

6. Update src/app.css with:
   - Control bar styling (flex row, gap, mantle background, border-bottom)
   - Button styles (.btn, .btn-reset, .btn-upload)
   - Demo select dropdown styling
   - Hardware panel floating positioning (bottom-right, semi-transparent mantle bg)
   - LED indicator (circle, green/dark)
   - S2 switch toggle styling
   - UART display fields

7. Add web-sys features needed: HtmlSelectElement, FileReader, File, FileList, Blob

8. Verify trunk build compiles successfully.