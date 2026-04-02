//! Editor panel — edit APL programs before running them.

use crate::prettify::{DisplayMode, prettify_line, translate_literate_to_ascii};
use web_sys::HtmlTextAreaElement;
use yew::prelude::*;

pub enum Msg {
    TextChanged(String),
    Run,
    Download,
    New,
    Close,
}

#[derive(Properties, PartialEq)]
pub struct EditorPanelProps {
    /// Current editor text.
    pub text: AttrValue,
    /// Whether the editor has unsaved changes.
    pub dirty: bool,
    /// Display mode for prettification preview.
    pub display_mode: DisplayMode,
    /// Called when the user edits the text.
    pub on_change: Callback<String>,
    /// Called when the user clicks Run.
    pub on_run: Callback<()>,
    /// Called when the user clicks New.
    pub on_new: Callback<()>,
    /// Called when the user clicks Close.
    pub on_close: Callback<()>,
    /// Optional inline style (e.g. for drag-resized width).
    #[prop_or_default]
    pub style: AttrValue,
}

pub struct EditorPanel {
    textarea_ref: NodeRef,
}

impl EditorPanel {
    /// Trigger a file download of the given text as a .a24 file.
    fn download_a24(text: &str) {
        use wasm_bindgen::JsCast;
        let doc = web_sys::window().unwrap().document().unwrap();
        let a = doc.create_element("a").unwrap();
        let opts = web_sys::BlobPropertyBag::new();
        opts.set_type("text/plain");
        let blob = web_sys::Blob::new_with_str_sequence_and_options(
            &js_sys::Array::of1(&wasm_bindgen::JsValue::from_str(text)),
            &opts,
        )
        .unwrap();
        let url = web_sys::Url::create_object_url_with_blob(&blob).unwrap();
        a.set_attribute("href", &url).unwrap();
        a.set_attribute("download", "program.a24").unwrap();
        a.unchecked_ref::<web_sys::HtmlElement>().click();
        web_sys::Url::revoke_object_url(&url).unwrap();
    }

    /// Render a prettification preview of the editor contents.
    fn render_preview(text: &str, mode: DisplayMode) -> Html {
        // In Repr mode, no preview needed (textarea already shows ASCII)
        if mode == DisplayMode::Repr {
            return html! {};
        }
        html! {
            <div class="editor-preview">
                <div class="editor-preview-label">
                    { if mode == DisplayMode::Glyph { "Glyph preview" } else { "Literate preview" } }
                </div>
                { for text.lines().map(|line| {
                    let segments = prettify_line(line, mode);
                    html! {
                        <div class="editor-preview-line">
                            { for segments.iter().map(|seg| match seg {
                                crate::prettify::Segment::Plain(t) => html! { <>{ t }</> },
                                crate::prettify::Segment::Keyword(t) => html! {
                                    <span class="apl-keyword">{ t }</span>
                                },
                            })}
                        </div>
                    }
                })}
            </div>
        }
    }
}

impl Component for EditorPanel {
    type Message = Msg;
    type Properties = EditorPanelProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            textarea_ref: NodeRef::default(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::TextChanged(text) => {
                ctx.props().on_change.emit(text);
                false // parent re-renders us with new props
            }
            Msg::Run => {
                ctx.props().on_run.emit(());
                false
            }
            Msg::Download => {
                // Translate literate input to ASCII before download
                let text = ctx.props().text.to_string();
                let translated = text
                    .lines()
                    .map(translate_literate_to_ascii)
                    .collect::<Vec<_>>()
                    .join("\n");
                Self::download_a24(&translated);
                false
            }
            Msg::New => {
                ctx.props().on_new.emit(());
                false
            }
            Msg::Close => {
                ctx.props().on_close.emit(());
                false
            }
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
        if first_render && let Some(el) = self.textarea_ref.cast::<HtmlTextAreaElement>() {
            let _ = el.focus();
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let props = ctx.props();

        let on_input = link.callback(|e: InputEvent| {
            let target: HtmlTextAreaElement = e.target_unchecked_into();
            Msg::TextChanged(target.value())
        });

        let on_run = link.callback(|_| Msg::Run);
        let on_download = link.callback(|_| Msg::Download);
        let on_new = link.callback(|_| Msg::New);
        let on_close = link.callback(|_| Msg::Close);

        let dirty_marker = if props.dirty { " *" } else { "" };

        html! {
            <div class="editor-panel" style={props.style.clone()}>
                <div class="editor-toolbar">
                    <span class="editor-title">
                        { format!("Editor{dirty_marker}") }
                    </span>
                    <button onclick={on_run} class="btn btn-editor-run"
                        title="Send to REPL (run program)">
                        { "\u{25B6} Run" }
                    </button>
                    <button onclick={on_download} class="btn btn-editor-dl"
                        title="Download as .a24 file">
                        { "\u{2913} Save" }
                    </button>
                    <button onclick={on_new} class="btn btn-editor-new"
                        title="New empty program">
                        { "New" }
                    </button>
                    <span class="editor-spacer"></span>
                    <button onclick={on_close} class="btn btn-editor-close"
                        title="Close editor">
                        { "\u{00D7}" }
                    </button>
                </div>
                <div class="editor-body">
                    <textarea
                        class="editor-textarea"
                        ref={self.textarea_ref.clone()}
                        value={props.text.clone()}
                        oninput={on_input}
                        spellcheck="false"
                        autocomplete="off"
                        placeholder="Write APL code here\u{2026}"
                    />
                    { Self::render_preview(&props.text, props.display_mode) }
                </div>
            </div>
        }
    }
}
