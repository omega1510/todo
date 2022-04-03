use yew::prelude::*;

enum Msg {
    Update(String),
    AddTodo,
    DeleteTodo(usize),
    Complete(usize),
}

struct Todo {
    completed: bool,
    text: String,
}
struct App {
    link: ComponentLink<Self>,
    value: String,
    todos: Vec<Todo>,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            value: String::new(),
            todos: Vec::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Update(val) => {
                self.value = val;
                true
            }
            Msg::AddTodo => {
                self.todos.push(Todo {
                    completed: false,
                    text: self.value.to_string(),
                });
                self.value = String::new();
                true
            }
            Msg::DeleteTodo(id) => {
                self.todos.remove(id);
                true
            }
            Msg::Complete(id) => {
                self.todos[id].completed = !self.todos[id].completed;
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="container-fluid bg-light rounded-3">
                <div class="row">
                    <div class="col-md-4">
                        <div class="container-fluid py-5">
                            <h1 class="display-5 fw-bold">{"Todo App"}</h1>
                            <p class="col-md-8 fs-4">
                                <div class="input-group">
                                    <span class="input-group-text" id="input-1">{"+"}</span>
                                    <input
                                    class="form-control"
                                        aria-describedby="input-1"
                                        type="text"
                                        placeholder="Enter a todo."
                                        value=self.value.to_string()
                                        oninput=self.link.callback(|e: InputData|{
                                            Msg::Update(e.value)
                                    })
                                    onkeypress=self.link.batch_callback(|e: KeyboardEvent|{
                                        if e.key() == "Enter" {
                                            Some(Msg::AddTodo)
                                        } else {None}
                                    })/>
                                    </div>
                                    </p>
                                    <button
                                    class="btn btn-primary" onclick=self.link.callback(|_| Msg::AddTodo)>{"Add Todo"}
                                </button>
                            </div>
                        </div>
                    </div>
                    <div class="col-md">
                        <div class="container-fluid py-5">
                            <div class="table-responsive">
                                <table class="table table-hover">
                                    <thead>
                                        <tr>
                                            <th scope="col">{"Status"}</th>
                                            <th scope="col">{"Todo"}</th>
                                            <th scope="col">{"Actions"}</th>
                                        </tr>
                                    </thead>
                                    <tbody>
                                        {
                                            for self.todos.iter().enumerate().map(|todo| self.view_todo(todo))
                                        }
                                    </tbody>
                                </table>
                            </div>
                        </div>
                    </div>
                </div>
        }
    }
}

impl App {
    fn view_todo(&self, (id, todo): (usize, &Todo)) -> Html {
        html! {
            <tr class={if todo.completed {"table-success"} else {""}}>
                <td><input type="checkbox" onclick={self.link.callback(move |_| Msg::Complete(id))} checked={if todo.completed {true} else {false}} /></td>
                <td>{todo.text.as_str()}</td>
                <td>
                    <button class="btn btn-danger" type="button" onclick=self.link.callback(move |_| Msg::DeleteTodo(id))>{"Delete"}</button>
                </td>
            </tr>
        }
    }
}
fn main() {
    yew::start_app::<App>();
}
