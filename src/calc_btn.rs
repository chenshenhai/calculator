use yew::prelude::*;

pub struct CalcButton {
    props: Props,
}

pub enum Msg {
    Clicked,
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
}

impl Component for CalcButton {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        CalcButton {
            props
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {

        let style_btn_item = {
            vec![
                "width: 100%",
                "flex: 1",
                "height: 80px"
            ].join("; ") + ";"
        };
        html! {
            <button style=style_btn_item>
                { self.props.children.render() }
            </button>
        }
    }
}
