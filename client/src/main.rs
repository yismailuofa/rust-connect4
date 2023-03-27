use yew::prelude::*;

mod pages;
use crate::pages::login_form::LoginForm;

#[function_component]
fn App() -> Html {

    html! {
        <div>
            <LoginForm/>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
