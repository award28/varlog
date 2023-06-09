use std::collections::HashMap;

use gloo_console::log;
use gloo_net::http::Request;
use serde::Deserialize;
use serde::de::DeserializeOwned;
use yew::prelude::*;

use crate::components::forms::FormInput;
use crate::hooks::use_input_state;
use crate::molecules::list_group::*;
use crate::organisms::header::Header;

#[derive(Properties, PartialEq)]
pub struct LogRetrieverProps {
    pub token: String,
}

#[derive(Debug, Clone)]
struct GetLogsRequest {
    filename: String,
    servers: Vec<String>,
    pattern: String,
    skip: usize,
    take: usize,
}

#[derive(Debug, Clone)]
struct VarlogClient {
    token: String,
}

#[derive(Deserialize)]
struct ErrorResponse {
    error: String,
}

impl VarlogClient {
    fn new<'a> (token: &'a str) -> Self {
        let token = format!("Bearer {token}");
        Self { token }
    }

    fn request(&self, url: &str) -> Request {
        Request::new(url)
            .header("Authorization", self.token.as_str())
    }

    async fn get_servers(&self) -> Result<Vec<String>, String> {
        let req = self.request(format!("/v1/servers").as_str());
        Self::execute(req).await
    }

    async fn get_logs(&self) -> Result<Vec<String>, String> {
        let req = self.request(format!("/v1/logs").as_str());
        Self::execute(req).await
    }

    async fn get_servers_log(
        &self,
        logs_req: GetLogsRequest,
    ) -> Result<HashMap<String, Vec<String>>, String> {
        let filename = logs_req.filename.as_str();
        let servers = logs_req.servers.clone();
        let q = servers.into_iter()
            .map(|server| ("server", server))
            .collect::<Vec<_>>();
        let req = self.request(format!("/v1/servers/logs/{filename}").as_str())
            .query(q)
            .query([
                ("pattern", logs_req.pattern.to_owned()),
                ("skip", format!("{}", logs_req.skip)),
                ("take", format!("{}", logs_req.take)),
            ]);
        Self::execute(req).await
    }

    async fn execute<'de, T>(req: Request) -> Result<T, String>
        where T: DeserializeOwned
    {
        let snag_msg = String::from("Looks like we hit a snag..");
        let resp = req.send().await
            .map_err(|e| {
                error!("Error executing request: {e}");
                snag_msg.clone()
            })?;

        match resp.json::<T>().await {
            Ok(data) => {return Ok(data);},
            Err(_) => (),
        }

        match resp.json::<ErrorResponse>().await {
            Ok(err_res) => Err(err_res.error),
            Err(e) => {
                error!("Error deserializing response: {e}");
                Err(snag_msg)
            },
        }
    }

}

#[function_component(LogRetriever)]
pub fn log_retriever(LogRetrieverProps { token }: &LogRetrieverProps) -> Html {
    let varlog_client = VarlogClient::new(token.as_str());
    let used_servers = use_state(|| vec![]);
    let used_servers_handle = {
        let used_servers = used_servers.clone();
        Callback::from(move |(checked, server): (bool, String)| {
            let mut servers = (*used_servers).to_owned();
            if checked {
                servers.push(server.to_owned())
            } else if let Some(i) = servers.iter()
                    .position(|x| *x == server) {
                servers.remove(i);
            }
            log!(format!("{:?}", servers));
            used_servers.set(servers);
        })
    };
    let servers = use_state(|| vec![]);
    {
        let servers = servers.clone();
        let varlog_client = varlog_client.clone();
        use_effect_with_deps(move |_| {
            let servers = servers.clone();
            let varlog_client = varlog_client.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_servers = varlog_client.get_servers().await;
                servers.set(fetched_servers.unwrap());
            });
            || ()
        }, ());
    }

    let servers_list_group = html! {
        <ListGroup>
        { 
            for servers.iter()
            .map(|server| { html_nested! {
                <ListGroupItem>
                    <CheckboxItem 
                        value={server.to_owned()}
                        onchange={used_servers_handle.clone()}
                    />
                </ListGroupItem>
            }}) 
        }
        </ListGroup>
    };
    let logfile = use_state(|| None);
    let logfile_handle = {
        let logfile = logfile.clone();
        Callback::from(move |file: String| {
            logfile.set(Some(file));
        })
    };
    let logs = use_state(|| vec![]);
    {
        let logs = logs.clone();
        let varlog_client = varlog_client.clone();
        use_effect_with_deps(move |_| {
            let logs = logs.clone();
            let varlog_client = varlog_client.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_logs = varlog_client.get_logs().await;
                logs.set(fetched_logs.unwrap());
            });
            || ()
        }, ());
    }

    let logs = html! {
        <ListGroup>
        { 
            for logs.iter()
            .map(|value| { html_nested! {
                <ListGroupItem>
                    <RadioItem
                        selected_file={(*logfile).clone()}
                        value={value.to_owned()}
                        onchange={logfile_handle.clone()}
                    />
                </ListGroupItem>
            }}) 
        }
        </ListGroup>
    };


    let error = use_state(|| String::default());

    let on_dismiss = {
        let error = error.clone();
        Callback::from(move |_| error.set(String::default()))
    };
    let (pattern, on_pattern_change) = use_input_state(String::default());
    let (skip, on_skip_change) = use_input_state(0);
    let (take, on_take_change) = use_input_state(10);
    let content = use_state(|| String::from("Logs will show here."));

    let on_skip_change = {
        let on_skip_change = on_skip_change.clone();
        Callback::from(move |value: String| {
            let value = value.parse::<usize>()
                .expect("value to be usize parsable.");
            on_skip_change.emit(value);
        })
    };
    let on_take_change = {
        let on_take_change = on_take_change.clone();
        Callback::from(move |value: String| {
            let value = value.parse::<usize>()
                .expect("value to be usize parsable.");
            on_take_change.emit(value);
        })
    };


    let on_submit = {
        let error = error.clone();
        let servers = used_servers.clone();
        let logfile = logfile.clone();
        let pattern = pattern.clone();
        let skip = skip.clone();
        let take = take.clone();
        let content = content.clone();
        let varlog_client = varlog_client.clone();
        Callback::from(move |_| {
            if logfile.is_none() {
                error.set(String::from("You must select a logfile."));
                return;
            }
            let servers = (*servers).clone();
            if servers.is_empty() {
                error.set(String::from("You must select at least 1 server."));
                return;
            }
            let filename = (*logfile).as_ref().unwrap().clone();
            let pattern: String = (*pattern).clone();
            let req = GetLogsRequest {
                filename,
                servers,
                pattern,
                skip: (*skip).clone(),
                take: (*take).clone(),
            };
            let error = error.clone();
            let varlog_client = varlog_client.clone();
            let content = content.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_content = varlog_client.get_servers_log(req).await;
                let fetched_content = {
                    match fetched_content {
                        Ok(content) => content,
                        Err(e) => {
                            error.set(e);
                            return;
                        },
                    }
                }.into_iter()
                    .map(|(server, logs)| {
                        let hr = String::from("---");
                        [hr.clone(), server, hr, logs.join("\t\n")].join("\n")
                    })
                    .collect::<Vec<_>>();
                content.set(fetched_content.join("\n"));
                error.set("".to_owned());
            });
           
        })
    };

    html! {
        <div class="container-center">
            <div class="row align-items-start">
                <div class="col">
                    <Header title="Available Servers" />
                    { servers_list_group }
                </div>
                <div class="col">
                    <Header title="Available Logs" />
                    { logs }
                </div>
            </div>
            <br />
            <form action="javascript:void(0);" class="row row-cols-lg-auto g-3 align-items-center">
              <div class="col-12">
                <label class="visually-hidden" for="inlineFormInputGroupUsername">{ "Username" }</label>
                <div class="input-group">
                  <div class="input-group-text">{ "Pattern" }</div>
                    <FormInput input_type="text" onchange={on_pattern_change.clone()} />
                </div>
              </div>

              <div class="col-12">
                <label class="visually-hidden" for="inlineFormInputGroupUsername">{ "Username" }</label>
                <div class="input-group">
                  <div class="input-group-text">{ "Skip" }</div>
                    <FormInput input_type="number" onchange={on_skip_change.clone()} />
                </div>
              </div>
           
              <div class="col-12">
                <label class="visually-hidden" for="inlineFormInputGroupUsername">{ "Username" }</label>
                <div class="input-group">
                  <div class="input-group-text">{ "Take" }</div>
                    <FormInput input_type="number" onchange={on_take_change.clone()} />
                </div>
              </div>
           
              <div class="col-12">
                <button type="submit" class="btn btn-primary" onclick={on_submit}>{ "Submit" }</button>
              </div>
            </form>
            <hr />
            if !error.is_empty() {
                <div class="alert alert-danger alert-dismissible fade show" role="alert">
                    <strong>{"Error"}</strong> { format!(" {}", error.as_str()) }
                <button type="button" class="btn-close" onclick={on_dismiss} aria-label="Close"></button>
                </div>
            }
            <pre>
                <code>
                    { (*content).clone() }
                </code>
            </pre>
        </div>
    }
}
