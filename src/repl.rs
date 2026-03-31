use gloo::timers::callback::Timeout;
use web_sys::HtmlDivElement;
use yew::prelude::*;

const PROMPT: &str = "      ";

pub enum Msg {
    KeyDown(KeyboardEvent),
    Blink,
}

pub struct ReplPanel {
    output: Vec<String>,
    input: String,
    cursor_visible: bool,
    _blink_timer: Option<Timeout>,
    output_ref: NodeRef,
}

impl ReplPanel {
    fn schedule_blink(ctx: &Context<Self>) -> Timeout {
        let link = ctx.link().clone();
        Timeout::new(530, move || link.send_message(Msg::Blink))
    }

    fn scroll_to_bottom(&self) {
        if let Some(el) = self.output_ref.cast::<HtmlDivElement>() {
            el.set_scroll_top(el.scroll_height());
        }
    }
}

impl Component for ReplPanel {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let timer = Self::schedule_blink(ctx);
        Self {
            output: vec!["APL / COR24 Environment".to_string(), String::new()],
            input: String::new(),
            cursor_visible: true,
            _blink_timer: Some(timer),
            output_ref: NodeRef::default(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Blink => {
                self.cursor_visible = !self.cursor_visible;
                self._blink_timer = Some(Self::schedule_blink(ctx));
                true
            }
            Msg::KeyDown(e) => {
                let key = e.key();
                match key.as_str() {
                    "Enter" => {
                        e.prevent_default();
                        let line = format!("{PROMPT}{}", self.input);
                        self.output.push(line);
                        // Echo back for now (no emulator yet)
                        if !self.input.is_empty() {
                            self.output.push(format!("      {}", self.input));
                        }
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
