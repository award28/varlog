use yew::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};

#[derive(Properties, PartialEq)]
pub struct FormInputProps {
    pub label: String,
    pub input_type: String,
    pub onchange: Callback<String>,
}

#[function_component(FormInput)]
pub fn form_input(FormInputProps {
    label,
    input_type,
    onchange,
}: &FormInputProps) -> Html {
    let on_change = onchange.clone();

    html! {
        <>
            <label for={ label.clone() } class="form-label">
                {label}
            </label>
            <input
                type={ input_type.clone() }
                class="form-control"
                id={ label.clone() }
                aria-describedby={format!("{label}Help")}
                onchange={
                    Callback::from(move |e: Event| {
                        // When events are created the target is undefined, it's only
                        // when dispatched does the target get added.
                        let target: Option<EventTarget> = e.target();

                        // Events can bubble so this listener might catch events from child
                        // elements which are not of type HtmlInputElement
                        let input = target.and_then(|t| t.dyn_into::<HtmlInputElement>().ok());

                        if let Some(input) = input {
                            on_change.emit(input.value());
                        }
                    })
                }
            />
        </>
    }
}
