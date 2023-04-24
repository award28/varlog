use gloo_net::http::Request;
use yew::prelude::*;

use gloo_console::log;
use serde::{Deserialize, Serialize};

use crate::{hooks::use_input_state, components::forms::FormInput};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AuthRequest {
    pub paths: Vec<String>,
    pub servers: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AuthResponse {
    pub token: String,
    pub expires: u64,
}

#[derive(Properties, PartialEq)]
pub struct AuthFormProps {
    pub ontoken: Callback<String>,
}

#[function_component(AuthForm)]
pub fn auth_form(AuthFormProps { ontoken }: &AuthFormProps) -> Html {
    let (logs_handle, on_logs_change) = use_input_state(
        String::default(),
    );
    let logs_value = (*logs_handle).clone();

    let (servers_handle, on_servers_change) = use_input_state(
        String::default(),
    );
    let servers_value = (*servers_handle).clone();

    let on_submit = {
        let logs_value = logs_value.clone();
        let servers_value = servers_value.clone();
        let ontoken = ontoken.clone();
        Callback::from(move |_| {
            let ontoken = ontoken.clone();
            let permissions = AuthRequest {
                paths: vec![logs_value.clone()],
                servers: vec![servers_value.clone()],
            };
            log!(format!("{:?}", permissions));

            wasm_bindgen_futures::spawn_local(async move {
                let auth_response: AuthResponse = Request::post("/v1/auth/register")
                    .json(&permissions)
                    .expect("Should be fine?")
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();

                log!(format!("{:?}", auth_response));
                ontoken.emit(auth_response.token.clone());
            });
        })
    };

    html! {
        <>
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
        </>
    }
}
