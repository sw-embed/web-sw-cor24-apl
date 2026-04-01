//! Control bar — Reset, Demo dropdown, File upload, Display mode.

use crate::demos::DEMOS;
use crate::prettify::DisplayMode;
use gloo::file::File;
use gloo::file::callbacks::FileReader;
use web_sys::HtmlInputElement;
use yew::prelude::*;

pub enum Msg {
    DemoSelected(usize),
    FileChanged(File),
    FileLoaded(String),
    Reset,
    SetDisplayMode(DisplayMode),
}

#[derive(Properties, PartialEq)]
pub struct ControlBarProps {
    pub on_reset: Callback<()>,
    pub on_demo: Callback<String>,
    pub on_upload: Callback<String>,
    pub display_mode: DisplayMode,
    pub on_display_mode: Callback<DisplayMode>,
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
            Msg::FileChanged(file) => {
                let link = ctx.link().clone();
                let reader = gloo::file::callbacks::read_as_text(&file, move |result| {
                    if let Ok(text) = result {
                        link.send_message(Msg::FileLoaded(text));
                    }
                });
                self._reader = Some(reader);
                false
            }
            Msg::FileLoaded(text) => {
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

        let on_file_change = link.callback(|e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let file = input.files().and_then(|fl| fl.get(0)).unwrap();
            Msg::FileChanged(File::from(file))
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
                <label class="btn btn-upload">
                    {"Upload .apl"}
                    <input type="file" accept=".apl,.txt"
                           onchange={on_file_change}
                           style="display:none" />
                </label>
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
            </div>
        }
    }
}
