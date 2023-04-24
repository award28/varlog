use gloo_console::log;
use gloo_net::http::Request;
use serde::Deserialize;
use serde::de::DeserializeOwned;
use yew::prelude::*;

use crate::molecules::list_group::*;
use crate::organisms::header::Header;

#[derive(Properties, PartialEq)]
pub struct LogRetrieverProps {
    pub token: String,
}

struct GetLogsRequest<'a> {
    filename: &'a str,
    pattern: &'a str,
    skip: usize,
    take: usize,
}

impl<'a> From<GetLogsRequest<'a>> for Vec<(&'a str, String)> {
    fn from(req: GetLogsRequest<'a>) -> Self {
        vec![
            ("pattern", req.pattern.to_owned()),
            ("skip", format!("{}", req.skip)),
            ("take", format!("{}", req.take)),
        ]
    }
}

#[derive(Clone)]
struct VarlogClient {
    token: String,
}

impl VarlogClient {
    fn new<'a> (token: &'a str) -> Self {
        let token = format!("Bearer {token}");
        Self { token }
    }

    fn request(&self, url: &str) -> Request {
        Request::new(url)
            .header(
                "Authorization",
                self.token.as_str(),
            )
    }

    async fn execute<'de, T>(req: Request) -> T 
        where T: DeserializeOwned
    {
            // TODO: Handle error case where file Authorization isn't permitted.
            req.send().await
            .expect("The request to succeed")
            .json().await
            .expect("The request to be a vector of strings")
    }

    async fn get_logs(&self) -> Vec<String> {
        let req = self.request(format!("/v1/logs").as_str());
        Self::execute(req).await
    }

    async fn get_log<'a>(&self, logs_req: GetLogsRequest<'a>) -> Vec<String> {
        let filename = logs_req.filename;
        let req = self.request(format!("/v1/logs/{filename}").as_str())
            .query(Vec::<(&str, String)>::from(logs_req));
        Self::execute(req).await
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
    let servers = html! {
        <ListGroup>
        { 
            // TODO: Replace this list with dynamic data from API
            for vec!["server1", "server2", "server3"].into_iter()
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
                logs.set(fetched_logs);
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

    let pattern = use_state(|| String::default());
    let skip = use_state(|| 0);
    let take = use_state(|| 10);
    let content = use_state(|| String::from("Logs will show here."));

    html! {
        <div class="container-center">
            <div class="row align-items-center">
                <div class="col">
                    <Header title="Available Servers" />
                    { servers }
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
                  <input type="text" class="form-control" value={ (*pattern).clone() } />
                </div>
              </div>

              <div class="col-12">
                <label class="visually-hidden" for="inlineFormInputGroupUsername">{ "Username" }</label>
                <div class="input-group">
                  <div class="input-group-text">{ "Skip" }</div>
                  <input type="number" class="form-control"  value={ format!("{}", *skip) } />
                </div>
              </div>
           
              <div class="col-12">
                <label class="visually-hidden" for="inlineFormInputGroupUsername">{ "Username" }</label>
                <div class="input-group">
                  <div class="input-group-text">{ "Take" }</div>
                  <input type="number" class="form-control" value={ format!("{}", *take) } />
                </div>
              </div>
           
              <div class="col-12">
                <button type="submit" class="btn btn-primary">{ "Submit" }</button>
              </div>
            </form>
            <hr />
            <pre>
                <code>
                    { (*content).clone() }
                </code>
            </pre>
        </div>
    }
}
