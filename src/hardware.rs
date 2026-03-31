//! Floating hardware panel — S2 switch, D2 LED, UART TX/RX display.

use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct HardwarePanelProps {
    pub led_on: bool,
    pub s2_on: bool,
    pub last_tx: Option<u8>,
    pub last_rx: Option<u8>,
    pub on_s2_toggle: Callback<()>,
}

#[function_component(HardwarePanel)]
pub fn hardware_panel(props: &HardwarePanelProps) -> Html {
    let on_s2 = {
        let cb = props.on_s2_toggle.clone();
        Callback::from(move |_: MouseEvent| cb.emit(()))
    };

    let led_class = if props.led_on {
        "hw-led hw-led-on"
    } else {
        "hw-led hw-led-off"
    };

    let switch_class = if props.s2_on {
        "hw-switch hw-switch-on"
    } else {
        "hw-switch hw-switch-off"
    };

    let tx_text = match props.last_tx {
        Some(b) => format!("{:02X}", b),
        None => "--".to_string(),
    };
    let rx_text = match props.last_rx {
        Some(b) => format!("{:02X}", b),
        None => "--".to_string(),
    };

    html! {
        <div class="hw-panel">
            <div class="hw-title">{"Hardware"}</div>
            <div class="hw-row">
                <span class="hw-label">{"S2"}</span>
                <span class={switch_class} onclick={on_s2}>
                    { if props.s2_on { "ON" } else { "OFF" } }
                </span>
            </div>
            <div class="hw-row">
                <span class="hw-label">{"D2"}</span>
                <span class={led_class}></span>
            </div>
            <div class="hw-row">
                <span class="hw-label">{"TX"}</span>
                <span class="hw-byte">{ tx_text }</span>
                <span class="hw-label hw-label-rx">{"RX"}</span>
                <span class="hw-byte">{ rx_text }</span>
            </div>
        </div>
    }
}
