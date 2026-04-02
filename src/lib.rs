pub mod config;
pub mod control_bar;
pub mod demos;
pub mod editor;
pub mod hardware;
pub mod help;
pub mod prettify;
pub mod repl;

use control_bar::ControlBar;
use editor::EditorPanel;
use hardware::HardwarePanel;
use help::HelpOverlay;
use prettify::{DisplayMode, translate_glyph_to_ascii};
use repl::ReplPanel;
use yew::prelude::*;

pub enum Msg {
    Reset,
    LoadDemo(String),
    /// Import a real APL file — convert Unicode glyphs to ASCII keywords.
    ImportAplFile(String),
    /// Upload a native .a24 file (already ASCII keywords).
    UploadProgram(String),
    ToggleS2,
    HwState(bool, Option<u8>, Option<u8>),
    SetDisplayMode(DisplayMode),
    ToggleHelp,
    /// Open editor with given text (from demo or upload).
    OpenEditor(String),
    /// Editor text changed.
    EditorChanged(String),
    /// Run the editor contents in the REPL.
    RunEditor,
    /// New empty editor.
    NewEditor,
    /// Close the editor panel.
    CloseEditor,
}

pub struct App {
    reset_seq: u32,
    feed_seq: u32,
    feed_text: AttrValue,
    s2_on: bool,
    led_on: bool,
    last_tx: Option<u8>,
    last_rx: Option<u8>,
    display_mode: DisplayMode,
    help_visible: bool,
    editor_visible: bool,
    editor_text: String,
    editor_dirty: bool,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            reset_seq: 0,
            feed_seq: 0,
            feed_text: AttrValue::default(),
            s2_on: false,
            led_on: false,
            last_tx: None,
            last_rx: None,
            display_mode: DisplayMode::default(),
            help_visible: false,
            editor_visible: false,
            editor_text: String::new(),
            editor_dirty: false,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Reset => {
                self.reset_seq = self.reset_seq.wrapping_add(1);
                self.s2_on = false;
                self.led_on = false;
                self.last_tx = None;
                self.last_rx = None;
                true
            }
            Msg::ImportAplFile(text) => {
                // Convert Unicode APL glyphs to our ASCII keyword dialect
                self.editor_text = text
                    .lines()
                    .map(translate_glyph_to_ascii)
                    .collect::<Vec<_>>()
                    .join("\n");
                self.editor_dirty = false;
                self.editor_visible = true;
                true
            }
            Msg::LoadDemo(text) | Msg::UploadProgram(text) => {
                self.editor_text = text;
                self.editor_dirty = false;
                self.editor_visible = true;
                true
            }
            Msg::ToggleS2 => {
                self.s2_on = !self.s2_on;
                true
            }
            Msg::HwState(led, tx, rx) => {
                let changed = self.led_on != led || self.last_tx != tx || self.last_rx != rx;
                if changed {
                    self.led_on = led;
                    self.last_tx = tx;
                    self.last_rx = rx;
                }
                changed
            }
            Msg::SetDisplayMode(mode) => {
                if self.display_mode != mode {
                    self.display_mode = mode;
                    true
                } else {
                    false
                }
            }
            Msg::ToggleHelp => {
                self.help_visible = !self.help_visible;
                true
            }
            Msg::OpenEditor(text) => {
                if self.editor_dirty {
                    let win = web_sys::window().unwrap();
                    if !win
                        .confirm_with_message("Discard unsaved changes?")
                        .unwrap_or(false)
                    {
                        return false;
                    }
                }
                self.editor_text = text;
                self.editor_dirty = false;
                self.editor_visible = true;
                true
            }
            Msg::EditorChanged(text) => {
                self.editor_text = text;
                self.editor_dirty = true;
                true
            }
            Msg::RunEditor => {
                self.feed_text = AttrValue::from(self.editor_text.clone());
                self.feed_seq = self.feed_seq.wrapping_add(1);
                self.editor_dirty = false;
                true
            }
            Msg::NewEditor => {
                if self.editor_dirty {
                    let win = web_sys::window().unwrap();
                    if !win
                        .confirm_with_message("Discard unsaved changes?")
                        .unwrap_or(false)
                    {
                        return false;
                    }
                }
                self.editor_text = String::new();
                self.editor_dirty = false;
                self.editor_visible = true;
                true
            }
            Msg::CloseEditor => {
                if self.editor_dirty {
                    let win = web_sys::window().unwrap();
                    if !win
                        .confirm_with_message("Discard unsaved changes?")
                        .unwrap_or(false)
                    {
                        return false;
                    }
                }
                self.editor_visible = false;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();

        let on_reset = link.callback(|()| Msg::Reset);
        let on_demo = link.callback(Msg::LoadDemo);
        let on_import = link.callback(Msg::ImportAplFile);
        let on_upload = link.callback(Msg::UploadProgram);
        let on_s2_toggle = link.callback(|()| Msg::ToggleS2);
        let on_hw_state = link.callback(|(led, tx, rx)| Msg::HwState(led, tx, rx));
        let on_display_mode = link.callback(Msg::SetDisplayMode);
        let on_help = link.callback(|()| Msg::ToggleHelp);
        let on_help_close = link.callback(|()| Msg::ToggleHelp);
        let on_edit = link.callback(|()| Msg::OpenEditor(String::new()));
        let on_editor_change = link.callback(Msg::EditorChanged);
        let on_editor_run = link.callback(|()| Msg::RunEditor);
        let on_editor_new = link.callback(|()| Msg::NewEditor);
        let on_editor_close = link.callback(|()| Msg::CloseEditor);

        html! {
            <>
                // GitHub corner
                <a href="https://github.com/sw-embed/web-sw-cor24-apl" class="github-corner"
                   aria-label="View source on GitHub" target="_blank">
                    <svg width="80" height="80" viewBox="0 0 250 250" aria-hidden="true">
                        <path d="M0,0 L115,115 L130,115 L142,142 L250,250 L250,0 Z" />
                        <path d="M128.3,109.0 C113.8,99.7 119.0,89.6 119.0,89.6 C122.0,82.7 120.5,78.6 \
                            120.5,78.6 C119.2,72.0 123.4,76.3 123.4,76.3 C127.3,80.9 125.5,87.3 125.5,87.3 \
                            C122.9,97.6 130.6,101.9 134.4,103.2" fill="currentColor"
                            style="transform-origin:130px 106px;" class="octo-arm" />
                        <path d="M115.0,115.0 C114.9,115.1 118.7,116.5 119.8,115.4 L133.7,101.6 C136.9,99.2 \
                            139.9,98.4 142.2,98.6 C133.8,88.0 127.5,74.4 143.8,58.0 C148.5,53.4 154.0,51.2 \
                            159.7,51.0 C160.3,49.4 163.2,43.6 171.4,40.1 C171.4,40.1 176.1,42.5 178.8,56.2 \
                            C183.1,58.6 187.2,61.8 190.9,65.4 C194.5,69.0 197.7,73.2 200.1,77.6 C213.8,80.2 \
                            216.3,84.9 216.3,84.9 C212.7,93.1 206.9,96.0 205.4,96.6 C205.1,102.4 203.0,107.8 \
                            198.3,112.5 C181.9,128.9 168.3,122.5 157.7,114.1 C157.9,116.9 156.7,120.9 \
                            152.7,124.9 L141.0,136.5 C139.8,137.7 141.6,141.9 141.8,141.8 Z"
                            fill="currentColor" />
                    </svg>
                </a>
                // Header
                <header>
                    <h1>{"APL"}</h1>
                    <span>{"COR24 Environment"}</span>
                </header>
                // Control bar
                <ControlBar {on_reset} {on_demo} {on_import} {on_upload}
                    display_mode={self.display_mode} {on_display_mode} {on_help}
                    {on_edit} editor_visible={self.editor_visible} />
                // Main content — editor left, REPL right
                <main id="main-content" class={if self.editor_visible { "side-by-side" } else { "" }}>
                    if self.editor_visible {
                        <EditorPanel
                            text={AttrValue::from(self.editor_text.clone())}
                            dirty={self.editor_dirty}
                            display_mode={self.display_mode}
                            on_change={on_editor_change}
                            on_run={on_editor_run}
                            on_new={on_editor_new}
                            on_close={on_editor_close}
                        />
                    }
                    <ReplPanel
                        reset_seq={self.reset_seq}
                        feed_text={self.feed_text.clone()}
                        feed_seq={self.feed_seq}
                        s2_on={self.s2_on}
                        on_hw_state={on_hw_state}
                        display_mode={self.display_mode}
                    />
                    <HardwarePanel
                        led_on={self.led_on}
                        s2_on={self.s2_on}
                        last_tx={self.last_tx}
                        last_rx={self.last_rx}
                        {on_s2_toggle}
                    />
                </main>
                if self.help_visible {
                    <HelpOverlay on_close={on_help_close} />
                }
                // Footer
                <footer>
                    <span>{"MIT License"}</span>
                    <span class="footer-sep">{"\u{00b7}"}</span>
                    <span>{"\u{00a9} 2026 Michael A Wright"}</span>
                    <span class="footer-sep">{"\u{00b7}"}</span>
                    <a href="https://makerlisp.com" target="_blank">{"COR24-TB"}</a>
                    <span class="footer-sep">{"\u{00b7}"}</span>
                    <span>{env!("BUILD_SHA")}</span>
                    <span class="footer-sep">{"\u{00b7}"}</span>
                    <span>{env!("BUILD_HOST")}</span>
                    <span class="footer-sep">{"\u{00b7}"}</span>
                    <span>{env!("BUILD_TIMESTAMP")}</span>
                </footer>
            </>
        }
    }
}
