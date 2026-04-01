//! APL quick-reference help overlay.

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

        html! {
            <div class="help-backdrop" onclick={on_backdrop}>
                <div class="help-dialog" ref={self.dialog_ref.clone()}
                     tabindex="-1" {onkeydown}>
                    <div class="help-header">
                        <span class="help-title">{"APL Quick Reference"}</span>
                        <span class="help-hint">{"Esc to close"}</span>
                    </div>
                    <div class="help-body">
                        <table class="help-table">
                            <tr class="help-section"><td colspan="2">{"Arithmetic"}</td></tr>
                            <tr><td class="help-key">{"+  \u{2212}  \u{00d7}  \u{00f7}"}</td>
                                <td>{"add, subtract, multiply, divide (element-wise)"}</td></tr>
                            <tr><td class="help-key">{"+/  \u{2212}/  \u{00d7}/"}</td>
                                <td>{"reduce: sum, difference, product"}</td></tr>

                            <tr class="help-section"><td colspan="2">{"Arrays"}</td></tr>
                            <tr><td class="help-key">{"iota N"}</td>
                                <td>{"index generator: 1 2 \u{2026} N"}</td></tr>
                            <tr><td class="help-key">{"rho X"}</td>
                                <td>{"shape (monadic)"}</td></tr>
                            <tr><td class="help-key">{"S rho X"}</td>
                                <td>{"reshape (dyadic)"}</td></tr>
                            <tr><td class="help-key">{"N take X"}</td>
                                <td>{"take first N elements"}</td></tr>
                            <tr><td class="help-key">{"N drop X"}</td>
                                <td>{"drop first N elements"}</td></tr>
                            <tr><td class="help-key">{"rev X"}</td>
                                <td>{"reverse"}</td></tr>
                            <tr><td class="help-key">{"A cat B"}</td>
                                <td>{"catenate"}</td></tr>

                            <tr class="help-section"><td colspan="2">{"Bitwise / Logic"}</td></tr>
                            <tr><td class="help-key">{"A and B"}</td>
                                <td>{"bitwise AND"}</td></tr>
                            <tr><td class="help-key">{"A or B"}</td>
                                <td>{"bitwise OR"}</td></tr>
                            <tr><td class="help-key">{"not X"}</td>
                                <td>{"bitwise complement"}</td></tr>

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

                            <tr class="help-section"><td colspan="2">{"Hardware I/O"}</td></tr>
                            <tr><td class="help-key">{"qled"}</td>
                                <td>{"read/write D2 LED"}</td></tr>
                            <tr><td class="help-key">{"qsw"}</td>
                                <td>{"read S2 switch state"}</td></tr>
                            <tr><td class="help-key">{"qsvo 'name'"}</td>
                                <td>{"shared variable (MMIO bridge)"}</td></tr>
                            <tr><td class="help-key">{"MMIO[N]"}</td>
                                <td>{"direct memory-mapped I/O"}</td></tr>

                            <tr class="help-section"><td colspan="2">{"Control Flow"}</td></tr>
                            <tr><td class="help-key">{"goto LABEL"}</td>
                                <td>{"unconditional branch"}</td></tr>
                            <tr><td class="help-key">{"goto (cond)/LABEL"}</td>
                                <td>{"conditional branch"}</td></tr>
                            <tr><td class="help-key">{"LABEL:"}</td>
                                <td>{"label definition"}</td></tr>

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
