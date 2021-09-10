use yew::prelude::*;
use yew::{html, Component, Html};

enum Msg {
    Add,
    Input(String),
    Toggle(usize),
}

struct Model {
    link: ComponentLink<Self>,
    input_value: String,
    todos: Vec<Entry>,
}

struct Entry {
    name: String,
    completed: bool,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            input_value: String::from(""),
            todos: vec![],
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Add => {
                self.todos.push(Entry {
                    name: self.input_value.clone(),
                    completed: false,
                });
                self.input_value = String::from("");
                true
            }
            Msg::Input(input_value) => {
                self.input_value = input_value;
                true
            }
            Msg::Toggle(index) => {
                let todo = self.todos.iter_mut().nth(index).unwrap();
                todo.completed = !todo.completed;
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <label for="new-todo">{"New TODO"} </label>
                <input
                    id="new-todo"
                    type="text"
                    value=self.input_value.clone()
                    oninput=self.link.callback(|e: InputData| Msg::Input(e.value))
                />
                <button onclick=self.link.callback(|_| Msg::Add) type="button">{"Add"}</button>
                <ul>
                { for self.todos.iter().enumerate().map(|(index, todo)| html! {
                    <li>
                        <label>
                            <input type="checkbox" checked=todo.completed onclick=self.link.callback(move |_| Msg::Toggle(index))/>
                            {
                                match todo.completed {
                                    false => html! {&todo.name},
                                    true => html! {<s>{&todo.name}</s>}
                                }
                            }
                        </label>
                    </li>
                }) }
                </ul>
            </div>
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<Model>();
}
