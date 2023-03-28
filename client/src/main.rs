use yew::prelude::*;
use yew_router::prelude::*;

mod components;
use crate::components::connect4::{Connect4, TootOtto};
use crate::components::login_form::LoginForm;
use crate::components::navbar::Navbar;
use crate::components::register_form::RegisterForm;

#[derive(Routable, PartialEq, Eq, Clone, Debug)]
pub enum Route {
    #[at("/connect4")]
    Connect4,
    #[at("/toot-otto")]
    TootOtto,
    #[at("/leaderboard")]
    Leaderboard,
    #[at("/user/login")]
    Login,
    #[at("/user/register")]
    Register,
    #[at("/")]
    Home,
    // #[not_found]
    // #[at("/404")]
    // NotFound,
}

#[function_component]
fn App() -> Html {
    html! {
           <BrowserRouter>
            <Navbar />

            <main>
                <Switch<Route> render={switch} />
                // <Game player1={"Player 1".to_string()} player2={"Player 2".to_string()} game_type={"Connect 4".to_string()} num_rows={100} num_cols={100} />
           </main>
        </BrowserRouter>
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Connect4 => {
            html! { <Connect4 /> }
        }
        Route::TootOtto => {
            html! { <TootOtto /> }
        }
        // Route::Leaderboard => {
        //     html! { <Leaderboard /> }
        // }
        Route::Login => {
            html! { <LoginForm /> }
        }
        Route::Register => {
            html! { <RegisterForm /> }
        }
        _ => todo!(),
        // Route::Home => {
        //     html! { <Home /> }
        // }
        // Route::NotFound => {
        //     html! { <PageNotFound /> }
        // }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
