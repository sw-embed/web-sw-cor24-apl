//! APL quick-reference help overlay.

use crate::prettify::KEYWORDS;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;
use yew::prelude::*;

pub enum Msg {
    KeyDown(KeyboardEvent),
    BackdropClick(MouseEvent),
}

#[derive(Properties, PartialEq)]
pub struct HelpOverlayProps {
    pub on_close: Callback<()>,
}

pub struct HelpOverlay {
    dialog_ref: NodeRef,
}

impl Component for HelpOverlay {
    type Message = Msg;
    type Properties = HelpOverlayProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            dialog_ref: NodeRef::default(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::KeyDown(e) => {
                if e.key() == "Escape" {
                    e.prevent_default();
                    ctx.props().on_close.emit(());
                }
                false
            }
            Msg::BackdropClick(e) => {
                // Close only if the click target is the backdrop itself.
                if let Some(dialog) = self.dialog_ref.cast::<HtmlElement>()
                    && let Some(target) = e.target()
                    && let Ok(target_el) = target.dyn_into::<HtmlElement>()
                    && !dialog.contains(Some(&target_el))
                {
                    ctx.props().on_close.emit(());
                }
                false
            }
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, _first_render: bool) {
        if let Some(el) = self.dialog_ref.cast::<HtmlElement>() {
            let _ = el.focus();
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onkeydown = ctx.link().callback(Msg::KeyDown);
        let on_backdrop = ctx.link().callback(Msg::BackdropClick);

        // Build keyword rows from the KEYWORDS table.
        let keyword_rows: Html = KEYWORDS
            .iter()
            .map(|kw| {
                let meaning = if let Some(dyadic) = kw.literate_dyadic {
                    format!("{} / {}", kw.literate_monadic, dyadic)
                } else {
                    kw.literate_monadic.to_string()
                };
                let aliases = if let Some(dyadic) = kw.literate_dyadic {
                    if dyadic != kw.ascii && kw.literate_monadic != kw.ascii {
                        format!("{}, {}", kw.literate_monadic, dyadic)
                    } else if kw.literate_monadic != kw.ascii {
                        kw.literate_monadic.to_string()
                    } else {
                        String::new()
                    }
                } else if kw.literate_monadic != kw.ascii {
                    kw.literate_monadic.to_string()
                } else {
                    String::new()
                };
                html! {
                    <tr>
                        <td class="help-key">{ kw.ascii }</td>
                        <td class="help-glyph">{ kw.glyph }</td>
                        <td class="help-meaning">{ &meaning }</td>
                        <td class="help-alias">{ &aliases }</td>
                    </tr>
                }
            })
            .collect();

        html! {
            <div class="help-backdrop" onclick={on_backdrop}>
                <div class="help-dialog" ref={self.dialog_ref.clone()}
                     tabindex="-1" {onkeydown}>
                    <div class="help-header">
                        <span class="help-title">{"APL Quick Reference"}</span>
                        <span class="help-hint">{"Esc to close"}</span>
                    </div>
                    <div class="help-body">
                        <p class="help-intro">
                            {"The "}
                            <span class="help-em">{"latin"}</span>
                            {" / "}
                            <span class="help-em">{"greek"}</span>
                            {" / "}
                            <span class="help-em">{"keywords"}</span>
                            {" buttons change how output is displayed. \
                              You can type either the ASCII keyword or its literate alias."}
                        </p>

                        // — Keyword reference (4-column) —
                        <table class="help-table help-kw-table">
                            <tr class="help-col-header">
                                <td>{"Type"}</td>
                                <td>{"APL"}</td>
                                <td>{"Meaning"}</td>
                                <td>{"Alias"}</td>
                            </tr>
                            { keyword_rows }
                        </table>

                        // — Remaining sections (2-column) —
                        <table class="help-table">
                            <tr class="help-section"><td colspan="2">{"Arithmetic"}</td></tr>
                            <tr><td class="help-key">{"+  \u{2212}  \u{00d7}  \u{00f7}"}</td>
                                <td>{"add, subtract, multiply, divide (element-wise)"}</td></tr>
                            <tr><td class="help-key">{"+/  \u{2212}/  \u{00d7}/"}</td>
                                <td>{"reduce: sum, difference, product"}</td></tr>

                            <tr class="help-section"><td colspan="2">{"Variables & Indexing"}</td></tr>
                            <tr><td class="help-key">{"NAME \u{2190} expr"}</td>
                                <td>{"assign (NAME must be uppercase)"}</td></tr>
                            <tr><td class="help-key">{"V[N]"}</td>
                                <td>{"bracket index"}</td></tr>
                            <tr><td class="help-key">{"V[N] \u{2190} expr"}</td>
                                <td>{"indexed assign"}</td></tr>

                            <tr class="help-section"><td colspan="2">{"Output"}</td></tr>
                            <tr><td class="help-key">{"[] \u{2190} expr"}</td>
                                <td>{"print to output"}</td></tr>

                            <tr class="help-section"><td colspan="2">{"System Commands"}</td></tr>
                            <tr><td class="help-key">{")VARS"}</td>
                                <td>{"list variables"}</td></tr>
                            <tr><td class="help-key">{")CLEAR"}</td>
                                <td>{"clear workspace"}</td></tr>
                            <tr><td class="help-key">{")OFF"}</td>
                                <td>{"halt interpreter"}</td></tr>

                            <tr class="help-section"><td colspan="2">{"Syntax Notes"}</td></tr>
                            <tr><td class="help-key">{"_N"}</td>
                                <td>{"negative literal (e.g. _3 = \u{207b}3)"}</td></tr>
                            <tr><td class="help-key">{"<-"}</td>
                                <td>{"assignment arrow (\u{2190})"}</td></tr>
                            <tr><td colspan="2" class="help-note">
                                {"Evaluation is right-to-left; use parens to override."}
                            </td></tr>
                        </table>
                    </div>
                </div>
            </div>
        }
    }
}
