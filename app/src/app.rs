use yew::prelude::*;

use crate::organisms::{auth_form::AuthForm, header::Header, log_retriever::LogRetriever};

#[function_component(App)]
pub fn app() -> Html {
    let token = use_state(|| None);

    let ontoken = {
        let token = token.clone();
        Callback::from(move |token_str: String| {
            token.set(Some(token_str))
        })
    };

    let token = token.as_ref().to_owned();

    html! {
        <div class="col-lg-8 mx-auto p-4 py-md-5">
        <Header title="VARLOG" />
        <main>
        if let Some(token) = token {
            <LogRetriever token={token.to_owned()} />
        } else {
            <AuthForm ontoken={ontoken} />
        }
        </main>
        </div>
    }
}
