use yew::prelude::*;
use yew::{html, ChildrenWithProps, Component, Context, Html, Properties};

#[derive(Properties, Clone, PartialEq)]
pub struct NavTabListProps {
    #[prop_or_default]
    pub children: ChildrenWithProps<NavTab>,
}

pub struct NavTabList;

impl Component for NavTabList {
    type Message = ();
    type Properties = NavTabListProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let tabs = html! {
            <ul class="nav nav-tabs" id="myTab" role="tablist">
                { for ctx.props().children.iter().map(|item| item.props.tab()) }
            </ul>
        };

        let contents = html! {
            <div class="tab-content" id="myTabContent">
                { for ctx.props().children.iter() }
            </div>
        };

        html! {
            <>
                { tabs }
                { contents }
            </>
        }
    }
}


#[derive(Properties, Clone, PartialEq)]
pub struct NavTabProps {
    pub title: String,
    #[prop_or_default]
    pub children: Children,
}

impl NavTabProps {
    pub fn tab(&self) -> Html {
        let title = self.title.clone();
        let lower = title.to_lowercase().replace(" ", "-");
        return html! {
            <li class="nav-item" role="presentation">
                <button
                    class="nav-link"
                    id={format!("{lower}-tab")}
                    data-bs-toggle="tab"
                    data-bs-target={format!("#{lower}-tab-pane")}
                    type="button"
                    role="tab"
                    aria-controls={format!("{lower}-tab-pane")}
                    aria-selected="true"
                >
                    { title }
                </button>
            </li>
        }
    }
}

pub struct NavTab;

impl Component for NavTab {
    type Message = ();
    type Properties = NavTabProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self 
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let lower = ctx.props().title.to_lowercase().replace(" ", "-");
        html! {
            <div 
                class="tab-pane fade show"
                id={format!("{lower}-tab-pane")}
                role="tabpanel"
                aria-labelledby={format!("{lower}-tab")}
                tabindex="0"
            >
                { for ctx.props().children.iter() }
            </div>
        }
    }
}
