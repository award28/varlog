use yew::prelude::*;

use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};

use gloo_console::log;
use serde::Deserialize;

#[derive(Clone, PartialEq, Deserialize)]
pub struct UserDetails {
    pub username: String,
    pub password: String,
}

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


#[hook]
fn use_input_state() -> (UseStateHandle<String>, Callback<String>) {
    let handle = use_state(String::default);

    let on_change = {
        let handle = handle.clone();

        Callback::from(move |value| {
            handle.set(value);
        })
    };
    (handle, on_change)
}

#[function_component(App)]
pub fn app() -> Html {
    let (logs_handle, on_logs_change) = use_input_state();
    let logs_value = (*logs_handle).clone();

    let (servers_handle, on_servers_change) = use_input_state();
    let servers_value = (*servers_handle).clone();

    let on_submit = {
        let logs_value = logs_value.clone();
        let servers_value = servers_value.clone();
        Callback::from(move |_| {
            log!(format!("{logs_value}, {servers_value}"));
        })
    };

    html! {
        <main>
            <h1>{"Authorization"}</h1>
            <form action="javascript:void(0);">
                <div class="mb-3">
                    <FormInput label="Logs Access" input_type="text"
                        onchange={on_logs_change.clone()}
                    />
                </div>
                <div class="mb-3">
                    <FormInput label="Servers Access" input_type="text"
                        onchange={on_servers_change}
                    />
                </div>
                <button type="submit" class="btn btn-primary" onclick={on_submit.clone()}>
                    {"Submit"}
                </button>
            </form>
        </main>
    }
}
