use yew::prelude::*;
mod connect4;
use connect4::Game;

#[function_component]
fn App() -> Html {
    
    html! {
       <div>
            <Game player1={"Player 1".to_string()} player2={"Player 2".to_string()} game_type={"Connect 4".to_string()} num_rows={100} num_cols={100} />
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
