#![recursion_limit = "128"]

mod calculate;

use yew::prelude::*;
use yew::services::ConsoleService;
use calculate::calc_expression;

pub struct Model {
    link: ComponentLink<Self>,
    expression: String,
    prev_char: char,
    console: ConsoleService,
}

pub enum Msg {
    ButtonClicked(char),
}


impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Model {
            link,
            expression: "".to_string(),
            prev_char: ' ',
            console: ConsoleService::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ButtonClicked(value) => {
                if value.is_digit(10) == true {
                    let char_str = value.to_string();
                    self.expression.push_str(&char_str);
                    self.prev_char = value;
                } else if value == '(' || value == ')'{
                    let char_str = value.to_string();
                    self.expression.push_str(&char_str);
                    self.prev_char = value;
                } else if value == 'C' {
                    self.expression.clear();
                    self.prev_char = ' ';
                } else if self.prev_char.is_digit(10) == true {
                     if value == '+' || value == '-' || value == '*' || value == '/' {
                        let char_str = value.to_string();
                        self.expression.push_str(&char_str);
                        self.prev_char = value;
                    }
                } else if value == '=' && (self.prev_char.is_digit(10) == true || self.prev_char == ')') {
                    self.console.log("do calculate!");
                    let result = calc_expression(&self.expression);
                    self.expression.clear();
                    self.expression.push_str(&result);
                    self.prev_char = ' ';
                }
                self.console.log(&self.expression);
                true
            },
        }
    }

    fn view(&self) -> Html {
        let style_btn_item = {
            vec![
                "width: 25%",
                "float: left",
                "height: 60px",
                "line-height: 60px",
                "text-align: center",
                "font-size: 28px",
                "cursor: pointer",
                "border: 1px solid #272d2d",
                "box-sizing: border-box",
                "background: #5f5f62",
                "user-select: none"
            ].join("; ") + ";"
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
                "min-height: 80px",
                "padding: 20px 30px",
                "box-sizing: border-box",
                "text-align: right"
            ].join("; ") + ";"
        };

        let style_btn_list = {
            vec![
                "width: 100%",
                "display: block",
                "overflow: hidden"
            ].join("; ") + ";"
        };
        let btns: Vec<char> = vec![
            'C', '(', ')', '/',
            '7', '8', '9', '*',
            '4', '5', '6', '-',
            '1', '2', '3', '+',
            '0', '=', ' ', ' '
        ];
        let calc_btn = |x| {
            let _char = btns[x];
            html! {
                <div style=style_btn_item 
                    onclick=self.link.callback(move|_| Msg::ButtonClicked(_char))
                >
                    {_char}
                </div>
            }
        };

        html! {
            <div style=style_container>
                <div style=style_screen>
                    <span>{&self.expression}</span>
                </div>
                <div style=style_btn_list>
                    {for (0..btns.len()).map(calc_btn)}
                </div>
            </div>
        }
    }
}

// impl Model {
// }
