use crate::config;
use cor24_emulator::{EmulatorCore, StopReason};
use gloo::timers::callback::Timeout;
use std::collections::VecDeque;
use web_sys::HtmlDivElement;
use yew::prelude::*;

const PROMPT: &str = "      ";

/// Execution batch size per tick (instructions).
const BATCH_SIZE: u64 = 50_000;

/// Tick interval in milliseconds.
const TICK_MS: u32 = 25;

pub enum Msg {
    KeyDown(KeyboardEvent),
    Blink,
    Init,
    Tick,
}

pub struct ReplPanel {
    emulator: EmulatorCore,
    output: Vec<String>,
    input: String,
    cursor_visible: bool,
    running: bool,
    halted: bool,
    uart_rx_queue: VecDeque<u8>,
    /// Partial line buffer for UART output that hasn't ended with '\n' yet.
    partial_line: String,
    _blink_timer: Option<Timeout>,
    _tick_handle: Option<Timeout>,
    output_ref: NodeRef,
}

impl ReplPanel {
    fn schedule_blink(ctx: &Context<Self>) -> Timeout {
        let link = ctx.link().clone();
        Timeout::new(530, move || link.send_message(Msg::Blink))
    }

    fn schedule_tick(ctx: &Context<Self>) -> Timeout {
        let link = ctx.link().clone();
        Timeout::new(TICK_MS, move || link.send_message(Msg::Tick))
    }

    fn scroll_to_bottom(&self) {
        if let Some(el) = self.output_ref.cast::<HtmlDivElement>() {
            el.set_scroll_top(el.scroll_height());
        }
    }

    /// Load the pre-assembled APL binary and start the emulator.
    fn load_apl_binary(&mut self) {
        let binary = config::APL_BINARY;
        self.emulator.hard_reset();
        self.emulator.set_uart_tx_busy_cycles(0);
        self.emulator.load_program(0, binary);
        self.emulator.load_program_extent(binary.len() as u32);
        self.emulator.set_pc(0);
        self.emulator.resume();
        self.running = true;
        self.halted = false;
        self.output.clear();
        self.partial_line.clear();
        self.uart_rx_queue.clear();
    }

    /// Feed bytes from the RX queue into the emulator UART while it's ready.
    fn feed_uart_bytes(&mut self) {
        while !self.uart_rx_queue.is_empty() {
            let status = self.emulator.read_byte(0xFF0101);
            if status & 0x01 != 0 {
                break; // RX buffer full, try again next tick
            }
            if let Some(byte) = self.uart_rx_queue.pop_front() {
                self.emulator.send_uart_byte(byte);
            }
        }
    }

    /// Collect UART output from the emulator and append to the output buffer.
    fn collect_uart(&mut self) {
        let uart = self.emulator.get_uart_output();
        if uart.is_empty() {
            return;
        }
        let uart = uart.to_string();
        self.emulator.clear_uart_output();

        // Split on newlines, handling partial lines across ticks.
        for ch in uart.chars() {
            if ch == '\n' {
                self.output.push(std::mem::take(&mut self.partial_line));
            } else if ch != '\r' {
                self.partial_line.push(ch);
            }
        }
    }
}

impl Component for ReplPanel {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_message(Msg::Init);
        let blink_timer = Self::schedule_blink(ctx);
        let mut emulator = EmulatorCore::new();
        emulator.set_uart_tx_busy_cycles(0);
        Self {
            emulator,
            output: vec!["APL / COR24 Environment".to_string(), String::new()],
            input: String::new(),
            cursor_visible: true,
            running: false,
            halted: false,
            uart_rx_queue: VecDeque::new(),
            partial_line: String::new(),
            _blink_timer: Some(blink_timer),
            _tick_handle: None,
            output_ref: NodeRef::default(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Init => {
                self.load_apl_binary();
                self._tick_handle = Some(Self::schedule_tick(ctx));
                true
            }
            Msg::Tick => {
                if !self.running || self.halted {
                    return false;
                }

                self.feed_uart_bytes();
                let result = self.emulator.run_batch(BATCH_SIZE);
                self.collect_uart();

                if matches!(result.reason, StopReason::Halted) {
                    self.halted = true;
                    self.running = false;
                }

                if self.running && !self.halted {
                    self._tick_handle = Some(Self::schedule_tick(ctx));
                }
                true
            }
            Msg::Blink => {
                self.cursor_visible = !self.cursor_visible;
                self._blink_timer = Some(Self::schedule_blink(ctx));
                true
            }
            Msg::KeyDown(e) => {
                // Let modifier combos (Cmd-C, Ctrl-V, etc.) pass through to browser
                if e.meta_key() || e.ctrl_key() || e.alt_key() {
                    return false;
                }
                let key = e.key();
                match key.as_str() {
                    "Enter" => {
                        e.prevent_default();
                        // Show the input line in output
                        let line = format!("{PROMPT}{}", self.input);
                        self.output.push(line);
                        // Queue input bytes for the emulator's UART
                        for b in self.input.bytes() {
                            self.uart_rx_queue.push_back(b);
                        }
                        self.uart_rx_queue.push_back(b'\n');
                        self.input.clear();
                        true
                    }
                    "Backspace" => {
                        e.prevent_default();
                        self.input.pop();
                        true
                    }
                    _ => {
                        if key.len() == 1 {
                            e.prevent_default();
                            let ch = key.chars().next().unwrap();
                            if ch.is_ascii_graphic() || ch == ' ' {
                                self.input.push(ch);
                                return true;
                            }
                        }
                        false
                    }
                }
            }
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, _first_render: bool) {
        self.scroll_to_bottom();
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onkeydown = ctx.link().callback(Msg::KeyDown);
        let cursor_class = if self.cursor_visible {
            "repl-cursor"
        } else {
            "repl-cursor repl-cursor-hidden"
        };

        html! {
            <div class="repl-panel" tabindex="0" {onkeydown}>
                <div class="repl-output" ref={self.output_ref.clone()}>
                    { for self.output.iter().map(|line| html! {
                        <div class="repl-line">{ line }</div>
                    })}
                    // Show partial line if the emulator is mid-output
                    if !self.partial_line.is_empty() {
                        <div class="repl-line">{ &self.partial_line }</div>
                    }
                    <div class="repl-input-line">
                        <span class="repl-prompt">{ PROMPT }</span>
                        <span class="repl-input-text">{ &self.input }</span>
                        <span class={cursor_class}>{"\u{2588}"}</span>
                    </div>
                </div>
            </div>
        }
    }
}
