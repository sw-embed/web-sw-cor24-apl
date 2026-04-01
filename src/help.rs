//! APL quick-reference help overlay with tabbed interface.

use crate::prettify::KEYWORDS;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;
use yew::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub enum Tab {
    QuickReference,
    Tutorial,
}

pub enum Msg {
    KeyDown(KeyboardEvent),
    BackdropClick(MouseEvent),
    SwitchTab(Tab),
}

#[derive(Properties, PartialEq)]
pub struct HelpOverlayProps {
    pub on_close: Callback<()>,
}

pub struct HelpOverlay {
    dialog_ref: NodeRef,
    active_tab: Tab,
}

impl Component for HelpOverlay {
    type Message = Msg;
    type Properties = HelpOverlayProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            dialog_ref: NodeRef::default(),
            active_tab: Tab::QuickReference,
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
            Msg::SwitchTab(tab) => {
                if self.active_tab != tab {
                    self.active_tab = tab;
                    true
                } else {
                    false
                }
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

        let tab_ref = {
            let link = ctx.link().clone();
            move |tab: Tab| link.callback(move |_: MouseEvent| Msg::SwitchTab(tab))
        };

        let qr_class = if self.active_tab == Tab::QuickReference {
            "help-tab help-tab-active"
        } else {
            "help-tab"
        };
        let tut_class = if self.active_tab == Tab::Tutorial {
            "help-tab help-tab-active"
        } else {
            "help-tab"
        };

        let body = match self.active_tab {
            Tab::QuickReference => self.view_quick_reference(),
            Tab::Tutorial => Self::view_tutorial(),
        };

        html! {
            <div class="help-backdrop" onclick={on_backdrop}>
                <div class="help-dialog" ref={self.dialog_ref.clone()}
                     tabindex="-1" {onkeydown}>
                    <div class="help-header">
                        <div class="help-tabs">
                            <span class={qr_class}
                                  onclick={tab_ref(Tab::QuickReference)}>
                                {"Quick Reference"}
                            </span>
                            <span class={tut_class}
                                  onclick={tab_ref(Tab::Tutorial)}>
                                {"Tutorial"}
                            </span>
                        </div>
                        <span class="help-hint">{"Esc to close"}</span>
                    </div>
                    <div class="help-body">
                        { body }
                    </div>
                </div>
            </div>
        }
    }
}

impl HelpOverlay {
    fn view_quick_reference(&self) -> Html {
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
            <>
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
            </>
        }
    }

    fn view_tutorial() -> Html {
        html! {
            <div class="tutorial">
                { Self::tutorial_section(
                    "1. Scalars & Arithmetic",
                    "APL works as a calculator. Arithmetic operators are +, -, *, and /. \
                     Negative numbers are written with an underscore prefix. \
                     Expressions evaluate right-to-left, not left-to-right.",
                    &[
                        ("3 + 4", "7"),
                        ("2 * 3 + 1", "8  (3+1 first, then *2)"),
                        ("(2 * 3) + 1", "7  (parens override)"),
                        ("10 - _3", "13  (_3 means negative 3)"),
                    ],
                )}
                { Self::tutorial_section(
                    "2. Variables",
                    "Assign values with the arrow (<-). Variable names must be UPPERCASE. \
                     Use )VARS to list defined variables and )CLEAR to reset the workspace.",
                    &[
                        ("X <- 42", ""),
                        ("X", "42"),
                        ("X + 8", "50"),
                        (")VARS", "X"),
                    ],
                )}
                { Self::tutorial_section(
                    "3. Vectors",
                    "Type multiple numbers separated by spaces to create a vector. \
                     Arithmetic applies element-wise. A scalar extends to match a vector.",
                    &[
                        ("1 2 3 4 5", "1 2 3 4 5"),
                        ("1 2 3 + 10 20 30", "11 22 33"),
                        ("2 * 1 2 3", "2 4 6  (scalar extension)"),
                    ],
                )}
                { Self::tutorial_section(
                    "4. Iota & Reduce",
                    "iota N generates integers 1 through N. \
                     The reduce operator (/) inserts a function between every element.",
                    &[
                        ("iota 5", "1 2 3 4 5"),
                        ("+/ 1 2 3 4 5", "15  (sum)"),
                        ("*/ iota 5", "120  (factorial of 5)"),
                    ],
                )}
                { Self::tutorial_section(
                    "5. Reshape & Shape",
                    "rho has two uses: monadic rho returns the shape (dimensions) of an array; \
                     dyadic rho reshapes data into a given shape.",
                    &[
                        ("rho 10 20 30", "3"),
                        ("2 3 rho iota 6", "1 2 3 / 4 5 6  (2\u{00d7}3 matrix)"),
                        ("rho 2 3 rho iota 6", "2 3"),
                    ],
                )}
                { Self::tutorial_section(
                    "6. Take & Drop",
                    "take selects the first N elements; drop removes them. \
                     Negative arguments work from the end.",
                    &[
                        ("3 take iota 5", "1 2 3"),
                        ("2 drop iota 5", "3 4 5"),
                        ("_2 take iota 5", "4 5  (last two)"),
                        ("_1 drop iota 5", "1 2 3 4  (drop last)"),
                    ],
                )}
                { Self::tutorial_section(
                    "7. Reverse & Catenate",
                    "rev reverses an array. cat joins two arrays together. \
                     Ravel (,) turns any array into a vector.",
                    &[
                        ("rev iota 5", "5 4 3 2 1"),
                        ("1 2 3 cat 4 5", "1 2 3 4 5"),
                    ],
                )}
                { Self::tutorial_section(
                    "8. Bracket Indexing",
                    "Use square brackets to select or update elements. \
                     Indices start at 1 (index origin).",
                    &[
                        ("V <- 10 20 30 40 50", ""),
                        ("V[3]", "30"),
                        ("V[2 4]", "20 40"),
                        ("V[1] <- 99", ""),
                        ("V", "99 20 30 40 50"),
                    ],
                )}
                { Self::tutorial_section(
                    "9. Matrices",
                    "Reshape creates matrices. Arithmetic works element-wise on matrices too.",
                    &[
                        ("M <- 2 3 rho 1 2 3 4 5 6", ""),
                        ("M + 10", "11 12 13 / 14 15 16"),
                        ("rho M", "2 3"),
                    ],
                )}
                { Self::tutorial_section(
                    "10. Comparison Operators",
                    "Comparisons return 1 (true) or 0 (false), element-wise on arrays. \
                     Not-equal is written as <>.",
                    &[
                        ("3 > 2", "1"),
                        ("1 2 3 4 5 >= 3", "0 0 1 1 1"),
                        ("5 <> 5", "0"),
                    ],
                )}
                { Self::tutorial_section(
                    "11. Control Flow",
                    "In multiline programs, labels mark jump targets. \
                     goto branches unconditionally; a conditional branch uses \
                     an expression that evaluates to a label name or empty.",
                    &[
                        ("[1]  X <- 1", ""),
                        ("[2]  LOOP: X <- X + 1", ""),
                        ("[3]  goto (X < 5) / 'LOOP'", ""),
                        ("[4]  X", "5"),
                    ],
                )}
                { Self::tutorial_section(
                    "12. System Commands",
                    "System commands start with ) and manage the workspace.",
                    &[
                        (")VARS", "list all variables"),
                        (")CLEAR", "erase all variables"),
                        (")LIST", "show program lines"),
                        (")RUN", "execute the program"),
                        (")ERASE", "delete program lines"),
                    ],
                )}
                { Self::tutorial_section(
                    "13. Multiline Programs",
                    "Enter program lines with [N] prefix. Lines are stored and run with )RUN. \
                     Use )LIST to review and )ERASE to delete.",
                    &[
                        ("[1]  [] <- 'HELLO'", ""),
                        ("[2]  [] <- 2 + 2", ""),
                        (")LIST", "[1] []<-'HELLO' / [2] []<-2+2"),
                        (")RUN", "HELLO / 4"),
                    ],
                )}
                { Self::tutorial_section(
                    "14. User-Defined Functions",
                    "Define functions with del (nabla). The header names the function \
                     and its parameters.",
                    &[
                        ("del R <- DOUBLE X", ""),
                        ("[1]  R <- 2 * X", ""),
                        ("del", "(closes definition)"),
                        ("DOUBLE 21", "42"),
                    ],
                )}
                { Self::tutorial_section(
                    "15. Hardware I/O",
                    "The COR24 has hardware registers exposed as quad-variables. \
                     qled controls the D2 LED; qsw reads the S2 switch.",
                    &[
                        ("qled <- 1", "turn on the D2 LED"),
                        ("qsw", "read switch (0 or 1)"),
                    ],
                )}
                { Self::tutorial_section(
                    "16. Shared Variables",
                    "qsvo couples a variable to an auxiliary processor (AP). \
                     AP 242 provides memory-mapped I/O to COR24 hardware.",
                    &[
                        ("'X' qsvo 242", "couple X to AP 242"),
                        ("X <- 100", "write 100 to MMIO"),
                        ("X", "read from MMIO"),
                    ],
                )}
                { Self::tutorial_section(
                    "17. Bitwise Operations",
                    "and, or, and not work bitwise on integers.",
                    &[
                        ("7 and 3", "3  (binary: 111 AND 011)"),
                        ("5 or 2", "7  (binary: 101 OR 010)"),
                        ("not 0", "65535  (16-bit complement)"),
                    ],
                )}
                { Self::tutorial_section(
                    "18. Edge Cases & Errors",
                    "APL reports errors clearly. Empty vectors are valid. \
                     Division by zero and domain errors are caught.",
                    &[
                        ("iota 0", "(empty vector)"),
                        ("rho iota 0", "0"),
                        ("1 / 0", "DOMAIN ERROR"),
                        ("1 2 3 + 1 2", "LENGTH ERROR"),
                    ],
                )}
            </div>
        }
    }

    fn tutorial_section(title: &str, description: &str, examples: &[(&str, &str)]) -> Html {
        let example_rows: Html = examples
            .iter()
            .map(|(input, output)| {
                if output.is_empty() {
                    html! {
                        <div class="tut-example">
                            <span class="tut-input">{"      "}{*input}</span>
                        </div>
                    }
                } else {
                    html! {
                        <div class="tut-example">
                            <span class="tut-input">{"      "}{*input}</span>
                            <span class="tut-output">{*output}</span>
                        </div>
                    }
                }
            })
            .collect();

        html! {
            <div class="tut-section">
                <h3 class="tut-title">{ title }</h3>
                <p class="tut-desc">{ description }</p>
                <div class="tut-examples">
                    { example_rows }
                </div>
            </div>
        }
    }
}
