use crate::config;
use crate::prettify::{DisplayMode, Segment, prettify_line};
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
    Init,
    Tick,
}

/// Command counter — when the parent bumps the counter, we act.
#[derive(Properties, PartialEq)]
pub struct ReplPanelProps {
    /// Bumped by parent to trigger a reset.
    #[prop_or_default]
    pub reset_seq: u32,
    /// Text to feed as a demo program (set when demo_seq changes).
    #[prop_or_default]
    pub feed_text: AttrValue,
    /// Bumped by parent when feed_text should be consumed.
    #[prop_or_default]
    pub feed_seq: u32,
    /// S2 switch state.
    #[prop_or_default]
    pub s2_on: bool,
    /// Callback to report hardware state each tick: (led_on, last_tx, last_rx).
    #[prop_or_default]
    pub on_hw_state: Callback<(bool, Option<u8>, Option<u8>)>,
    /// Display mode for APL prettification.
    #[prop_or_default]
    pub display_mode: DisplayMode,
}

pub struct ReplPanel {
    emulator: EmulatorCore,
    output: Vec<String>,
    input: String,
    running: bool,
    halted: bool,
    uart_rx_queue: VecDeque<u8>,
    /// Partial line buffer for UART output that hasn't ended with '\n' yet.
    partial_line: String,
    _tick_handle: Option<Timeout>,
    output_ref: NodeRef,
    panel_ref: NodeRef,
    needs_focus: bool,
    /// Track which reset_seq we last processed.
    last_reset_seq: u32,
    /// Track which feed_seq we last processed.
    last_feed_seq: u32,
    /// Last TX byte seen.
    last_tx: Option<u8>,
    /// Last RX byte seen.
    last_rx: Option<u8>,
}

impl ReplPanel {
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
        self.last_tx = None;
        self.last_rx = None;
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
                self.last_rx = Some(byte);
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

        // Track last TX byte
        if let Some(b) = uart.bytes().last() {
            self.last_tx = Some(b);
        }

        // Split on newlines, handling partial lines across ticks.
        for ch in uart.chars() {
            if ch == '\n' {
                self.output.push(std::mem::take(&mut self.partial_line));
            } else if ch != '\r' {
                self.partial_line.push(ch);
            }
        }
    }

    /// Render a single output line, applying prettification to echoed input lines.
    fn render_line(line: &str, mode: DisplayMode) -> Html {
        // Lines starting with the 6-space prompt are echoed user input — prettify those.
        if let Some(content) = line.strip_prefix(PROMPT) {
            let segments = prettify_line(content, mode);
            html! {
                <div class="repl-line">
                    <span class="repl-prompt">{ PROMPT }</span>
                    { Self::render_segments(&segments) }
                </div>
            }
        } else {
            html! { <div class="repl-line">{ line }</div> }
        }
    }

    /// Render prettified segments with keyword highlighting.
    fn render_segments(segments: &[Segment]) -> Html {
        html! {
            { for segments.iter().map(|seg| match seg {
                Segment::Plain(text) => html! { <>{ text }</> },
                Segment::Keyword(text) => html! {
                    <span class="apl-keyword">{ text }</span>
                },
            })}
        }
    }

    /// Queue text to be sent to the interpreter line-by-line.
    fn feed_program_text(&mut self, text: &str) {
        for line in text.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }
            for b in trimmed.bytes() {
                self.uart_rx_queue.push_back(b);
            }
            self.uart_rx_queue.push_back(b'\n');
        }
    }
}

impl Component for ReplPanel {
    type Message = Msg;
    type Properties = ReplPanelProps;

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_message(Msg::Init);
        let mut emulator = EmulatorCore::new();
        emulator.set_uart_tx_busy_cycles(0);
        Self {
            emulator,
            output: vec!["APL / COR24 Environment".to_string(), String::new()],
            input: String::new(),
            running: false,
            halted: false,
            uart_rx_queue: VecDeque::new(),
            partial_line: String::new(),
            _tick_handle: None,
            output_ref: NodeRef::default(),
            panel_ref: NodeRef::default(),
            needs_focus: false,
            last_reset_seq: 0,
            last_feed_seq: 0,
            last_tx: None,
            last_rx: None,
        }
    }

    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        let props = ctx.props();

        // Handle reset command
        if props.reset_seq != self.last_reset_seq {
            self.last_reset_seq = props.reset_seq;
            self.load_apl_binary();
            self.needs_focus = true;
            // Always schedule a fresh tick — the old Timeout may have
            // already fired (e.g. after )OFF halt) but still be Some.
            self._tick_handle = Some(Self::schedule_tick(ctx));
        }

        // Handle feed text command
        if props.feed_seq != self.last_feed_seq {
            self.last_feed_seq = props.feed_seq;
            self.feed_program_text(&props.feed_text);
            self.needs_focus = true;
        }

        // S2 switch — use emulator API (active-low hardware)
        self.emulator.set_button_pressed(props.s2_on);

        true
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
                    // Flush any remaining partial line
                    if !self.partial_line.is_empty() {
                        self.output.push(std::mem::take(&mut self.partial_line));
                    }
                    self.output.push(String::new());
                    self.output
                        .push("[Halted — press Reset to restart]".to_string());
                }

                if self.running && !self.halted {
                    self._tick_handle = Some(Self::schedule_tick(ctx));
                }

                // Report hardware state to parent (LED is active-low)
                let led_on = self.emulator.is_led_on();
                ctx.props()
                    .on_hw_state
                    .emit((led_on, self.last_tx, self.last_rx));

                true
            }
            Msg::KeyDown(e) => {
                // Let modifier combos (Cmd-C, Ctrl-V, etc.) pass through to browser
                if e.meta_key() || e.ctrl_key() || e.alt_key() {
                    return false;
                }
                // Ignore input when the emulator has halted (e.g. after )OFF)
                if self.halted {
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

    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
        self.scroll_to_bottom();
        if (first_render || self.needs_focus)
            && let Some(el) = self.panel_ref.cast::<HtmlDivElement>()
        {
            let _ = el.focus();
            self.needs_focus = false;
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onkeydown = ctx.link().callback(Msg::KeyDown);
        let mode = ctx.props().display_mode;

        html! {
            <div class="repl-panel" tabindex="0" {onkeydown} ref={self.panel_ref.clone()}>
                <div class="repl-output" ref={self.output_ref.clone()}>
                    { for self.output.iter().map(|line| Self::render_line(line, mode)) }
                    // Show partial line if the emulator is mid-output
                    if !self.partial_line.is_empty() {
                        <div class="repl-line">{ &self.partial_line }</div>
                    }
                    <div class="repl-input-line">
                        <span class="repl-prompt">{ PROMPT }</span>
                        <span class="repl-input-text">
                            { Self::render_segments(&prettify_line(&self.input, mode)) }
                        </span>
                        <span class="repl-cursor">{"\u{2588}"}</span>
                    </div>
                </div>
            </div>
        }
    }
}
