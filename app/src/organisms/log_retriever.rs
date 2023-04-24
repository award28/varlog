use gloo_console::log;
use yew::prelude::*;

use crate::molecules::list_group::*;
use crate::organisms::header::Header;

#[derive(Properties, PartialEq)]
pub struct LogRetrieverProps {
    pub token: String,
}

#[function_component(LogRetriever)]
pub fn log_retriever(LogRetrieverProps { token }: &LogRetrieverProps) -> Html {
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
    let logs = html! {
        <ListGroup>
        { 
            // TODO: Replace this list with dynamic data from API
            for vec!["daily.out", "weekly.out", "apt/file.log"].into_iter()
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
