#![recursion_limit = "128"]

mod button;
mod counter;
mod calc_btn;
use counter::{Color, Counter};
use calc_btn::{CalcButton};
use yew::prelude::*;

pub struct Model {
    link: ComponentLink<Self>,
    with_barrier: bool,
    color: Color,
}

pub enum Msg {
    Repaint,
    Toggle,
    ChildClicked(u32),
}


impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Model {
            link,
            with_barrier: false,
            color: Color::Red,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Repaint => {
                self.color = Color::Blue;
                true
            }
            Msg::Toggle => {
                self.with_barrier = !self.with_barrier;
                true
            }
            Msg::ChildClicked(_value) => false,
        }
    }

    fn view(&self) -> Html {
        let btns = vec![
            "C", "(", ")", "/",
            "7", "8", "9", "*",
            "4", "5", "6", "-",
            "1", "2", "3", "+",
            "0", ".", "=", ""
        ];
        let counter = |x| {
            html! {
                <Counter initial=x color=&self.color
                    onclick=self.link.callback(Msg::ChildClicked) />
            }
        };
        let calc_btn = |x| {
            html! {
                <CalcButton>
                    <span>{btns[x]}</span>
                </CalcButton>
            }
        };


        let style_container = {
            vec![
                "max-width: 480px",
                "background: #2a2c2f",
                "margin: 0 auto",
                "color: #ffffff",
            ].join("; ") + ";"
        };
        let style_screen = {
            vec![
                "width: 100%",
                "background: #2a2c2f",
                "font-size: 28px",
                "padding: 0 30px",
                "box-sizing: border-box"
            ].join("; ") + ";"
        };

        let style_btn_list = {
            vec![
                "width: 100%",
                "display: block",
                "overflow: hidden"
            ].join("; ") + ";"
        };
        
        html! {
            <div style=style_container>
                <button onclick=self.link.callback(|_| Msg::Toggle)>{ "Toggle" }</button>
                { self.view_barrier() }
                {for (0..0).map(counter)}
                // <!-- calculator -->

                <div >
                    <div style=style_screen>
                        <span>{"1+(2-3)*4/6"}</span>
                    </div>
                    <div style=style_btn_list>
                        {for (0..btns.len()).map(calc_btn)}
                    </div>
                </div>
            </div>
        }
    }
}

impl Model {
    fn view_barrier(&self) -> Html {
        if self.with_barrier {
            html! {
                <p>{ "No Click \"toggle\"!" }</p>
            }
        } else {
            html! {
                <p>{ "Click \"toggle\"!" }</p>
            }
        }
    }
}
