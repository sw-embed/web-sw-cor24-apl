//! Control bar — Reset, Demo dropdown, File upload, Display mode.

use crate::demos::DEMOS;
use crate::prettify::DisplayMode;
use gloo::file::File;
use gloo::file::callbacks::FileReader;
use web_sys::HtmlInputElement;
use yew::prelude::*;

pub enum Msg {
    DemoSelected(usize),
    ImportFileSelected(File),
    ImportFileLoaded(String),
    UploadFileSelected(File),
    UploadFileLoaded(String),
    Reset,
    SetDisplayMode(DisplayMode),
}

#[derive(Properties, PartialEq)]
pub struct ControlBarProps {
    pub on_reset: Callback<()>,
    pub on_demo: Callback<String>,
    /// Import a real APL file (Unicode glyphs → ASCII keywords).
    pub on_import: Callback<String>,
    /// Upload a native .a24 file (already ASCII keywords).
    pub on_upload: Callback<String>,
    pub display_mode: DisplayMode,
    pub on_display_mode: Callback<DisplayMode>,
    pub on_help: Callback<()>,
    pub on_edit: Callback<()>,
    pub editor_visible: bool,
}

pub struct ControlBar {
    _reader: Option<FileReader>,
    selected_demo: Option<usize>,
}

impl Component for ControlBar {
    type Message = Msg;
    type Properties = ControlBarProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            _reader: None,
            selected_demo: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Reset => {
                self.selected_demo = None;
                ctx.props().on_reset.emit(());
                true
            }
            Msg::DemoSelected(index) => {
                if let Some(demo) = DEMOS.get(index) {
                    self.selected_demo = Some(index);
                    ctx.props().on_demo.emit(demo.source.to_string());
                }
                true
            }
            Msg::ImportFileSelected(file) => {
                let link = ctx.link().clone();
                let reader = gloo::file::callbacks::read_as_text(&file, move |result| {
                    if let Ok(text) = result {
                        link.send_message(Msg::ImportFileLoaded(text));
                    }
                });
                self._reader = Some(reader);
                false
            }
            Msg::ImportFileLoaded(text) => {
                self._reader = None;
                ctx.props().on_import.emit(text);
                false
            }
            Msg::UploadFileSelected(file) => {
                let link = ctx.link().clone();
                let reader = gloo::file::callbacks::read_as_text(&file, move |result| {
                    if let Ok(text) = result {
                        link.send_message(Msg::UploadFileLoaded(text));
                    }
                });
                self._reader = Some(reader);
                false
            }
            Msg::UploadFileLoaded(text) => {
                self._reader = None;
                ctx.props().on_upload.emit(text);
                false
            }
            Msg::SetDisplayMode(mode) => {
                ctx.props().on_display_mode.emit(mode);
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();

        let on_reset = link.callback(|_| Msg::Reset);

        let on_demo_change = link.callback(|e: Event| {
            let target: web_sys::HtmlSelectElement = e.target_unchecked_into();
            let idx: usize = target.value().parse().unwrap_or(0);
            Msg::DemoSelected(idx)
        });

        let on_import_change = link.callback(|e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let file = input.files().and_then(|fl| fl.get(0)).unwrap();
            Msg::ImportFileSelected(File::from(file))
        });

        let on_upload_change = link.callback(|e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let file = input.files().and_then(|fl| fl.get(0)).unwrap();
            Msg::UploadFileSelected(File::from(file))
        });

        let current_mode = ctx.props().display_mode;
        let on_mode_repr = link.callback(|_| Msg::SetDisplayMode(DisplayMode::Repr));
        let on_mode_glyph = link.callback(|_| Msg::SetDisplayMode(DisplayMode::Glyph));
        let on_mode_literate = link.callback(|_| Msg::SetDisplayMode(DisplayMode::Literate));

        let mode_class = |mode: DisplayMode| {
            if mode == current_mode {
                "btn btn-mode btn-mode-active"
            } else {
                "btn btn-mode"
            }
        };

        html! {
            <div class="control-bar">
                <button onclick={on_reset} class="btn btn-reset">{"Reset"}</button>
                <select class="demo-select" onchange={on_demo_change}>
                    <option value="" selected={self.selected_demo.is_none()}>
                        {"Demo\u{2026}"}
                    </option>
                    { for DEMOS.iter().enumerate().map(|(i, demo)| {
                        let sel = self.selected_demo == Some(i);
                        html! {
                            <option value={i.to_string()} selected={sel}
                                    title={demo.description}>
                                { &demo.name }
                            </option>
                        }
                    })}
                </select>
                <label class="btn btn-upload" title="Import APL file (converts Unicode glyphs to ASCII)">
                    {"Import .apl"}
                    <input type="file" accept=".apl"
                           onchange={on_import_change}
                           style="display:none" />
                </label>
                <label class="btn btn-upload" title="Load native COR24 APL file">
                    {"Open .a24"}
                    <input type="file" accept=".a24,.txt"
                           onchange={on_upload_change}
                           style="display:none" />
                </label>
                <button onclick={ctx.props().on_edit.reform(|_| ())}
                        class={if ctx.props().editor_visible { "btn btn-edit btn-edit-active" } else { "btn btn-edit" }}
                        title="Open editor">
                    { "Edit" }
                </button>
                <span class="mode-group">
                    <button onclick={on_mode_repr}
                            class={mode_class(DisplayMode::Repr)}
                            title="Show raw ASCII keywords">
                        {"latin"}
                    </button>
                    <button onclick={on_mode_glyph}
                            class={mode_class(DisplayMode::Glyph)}
                            title="Show APL glyphs">
                        {"greek"}
                    </button>
                    <button onclick={on_mode_literate}
                            class={mode_class(DisplayMode::Literate)}
                            title="Show literate names">
                        {"keywords"}
                    </button>
                </span>
                <button onclick={ctx.props().on_help.reform(|_| ())}
                        class="btn btn-help" title="APL Quick Reference">
                    {"?"}
                </button>
                <span class="io-indicator" title="Index origin: iota 5 \u{2192} 0 1 2 3 4">
                    {"\u{2395}IO\u{2190}0"}
                </span>
                <span class="corner-spacer"></span>
            </div>
        }
    }
}
