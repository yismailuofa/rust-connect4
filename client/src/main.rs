use yew::prelude::*;
use yew_router::history::{AnyHistory, History, MemoryHistory};
use yew_router::prelude::*;

mod components;
use crate::components::login_form::LoginForm;
use crate::components::register_form::RegisterForm;
use crate::components::navbar::Navbar;

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
            </main>
        </BrowserRouter>
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        // Route::Connect4 => {
        //     html! { <Connect4 /> }
        // }
        // Route::TootOtto => {
        //     html! { <TootOtto /> }
        // }
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
