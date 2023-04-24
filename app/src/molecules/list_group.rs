use gloo_console::log;
use yew::html::ChildrenRenderer;
use yew::prelude::*;
use yew::virtual_dom::VChild;
use yew::{html, Children, Component, Context, Html, Properties};

#[derive(Properties, Clone, PartialEq)]
pub struct ListGroupProps {
    #[prop_or_default]
    pub children: ChildrenRenderer<ListGroupItem>,
}

pub struct ListGroup;
impl Component for ListGroup {
    type Message = ();
    type Properties = ListGroupProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <ul class="list-group">
                { for ctx.props().children.iter() }
            </ul>
        }
    }
}


#[derive(Properties, Clone, PartialEq)]
pub struct ListGroupItemProps {
    #[prop_or_default]
    pub children: Children,
}

#[derive(Clone, PartialEq)]
pub struct ListGroupItem {
    props: ListGroupItemProps,
}

impl Component for ListGroupItem {
    type Message = ();
    type Properties = ListGroupItemProps;

    fn create(ctx: &Context<Self>) -> Self {
        let props = ctx.props().clone();
        Self {
            props
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        self.clone().into()
    }
}

impl From<VChild<ListGroupItem>> for ListGroupItem {
    fn from(child: VChild<ListGroupItem>) -> Self {
        let props = (*child.props.as_ref()).clone();
        Self {
            props,
        }
    }
}

impl Into<Html> for ListGroupItem {
    fn into(self) -> Html {
        html! {
            <li class="list-group-item">
                { for self.props.children.iter() }
            </li>
        }
    }
}

#[derive(Properties, Clone, PartialEq)]
pub struct CheckboxItemProps {
    pub value: String,
    pub onchange: Callback<(bool, String)>,
}

#[function_component(CheckboxItem)]
pub fn checkbox_item(CheckboxItemProps { value, onchange }: &CheckboxItemProps) -> Html {
    let value = value.clone();
    let id = value.to_lowercase().replace(" ", "-");
    let checked = use_state(|| false);

    let oncheck = {
        let checked = checked.clone();
        let onchange = onchange.clone();
        let value = value.clone();
        Callback::from(move |_| {
            checked.set(!*checked);
            onchange.emit((!*checked, value.clone()));
        })
    };
    
    html! {
        <>
        <input
            class="form-check-input me-1"
            type="checkbox"
            value=""
            id={id.clone()}
            onchange={oncheck}
        />
        <label class="form-check-label" for={ id }> { value }</label>
        </>
    }
}

#[derive(Properties, Clone, PartialEq)]
pub struct RadioItemProps {
    pub value: String,
    pub selected_file: Option<String>,
    pub onchange: Callback<String>,
}


#[function_component(RadioItem)]
pub fn radio_item(RadioItemProps { value, selected_file, onchange }: &RadioItemProps) -> Html {
    let value = value.clone();
    let id = value.to_lowercase().replace(" ", "-");
    let checked = use_state(|| false);

    let oncheck = {
        let checked = checked.clone();
        let onchange = onchange.clone();
        let value = value.clone();
        Callback::from(move |_| {
            checked.set(!*checked);
            onchange.emit(value.clone());
        })
    };
    log!(format!("{selected_file:?}, {value}"));
    html! {
        <>
        <input
            class="form-check-input me-1"
            type="radio"
            value=""
            id={id.clone()}
            onchange={oncheck}
            checked={ (*selected_file).clone().unwrap_or(String::default()) == value }
        />
        <label class="form-check-label" for={ id }> { value }</label>
        </>
    }
}
