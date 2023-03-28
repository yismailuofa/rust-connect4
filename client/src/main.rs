use yew::prelude::*;
mod connect4;
use connect4::Game;

mod pages;
use crate::pages::login_form::LoginForm;

#[function_component]
fn App() -> Html {
    html! {
           <div>
           <LoginForm/>
           <Game player1={"Player 1".to_string()} player2={"Player 2".to_string()} game_type={"otto".to_string()} num_rows={6} num_cols={7} />
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
