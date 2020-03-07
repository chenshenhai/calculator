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
                "width: 25%",
                "float: left",
                "height: 80px",
                "line-height: 80px",
                "text-align: center",
                "font-size: 28px",
                "cursor: pointer",
                "border: 1px solid #272d2d",
                "box-sizing: border-box",
                "background: #5f5f62"
            ].join("; ") + ";"
        };
        html! {
            <div style=style_btn_item>
                { self.props.children.render() }
            </div>
        }
    }
}
