use yew::prelude::*;
use yew_router::prelude::*;

mod components;
use crate::components::connect4::{Connect4, TootOtto};
use crate::components::login_form::LoginForm;
use crate::components::navbar::Navbar;
use crate::components::register_form::RegisterForm;
use crate::components::leaderboard::LeaderBoard;
use client::User;

#[derive(Routable, PartialEq, Eq, Clone, Debug)]
pub enum MainRoute {
    #[at("/")]
    Connect4,
    #[at("/toot-otto")]
    TootOtto,
    #[at("/leaderboard")]
    Leaderboard,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[derive(Routable, PartialEq, Eq, Clone, Debug)]
pub enum LoginRoute {
    #[at("/")]
    Login,
    #[at("/register")]
    Register,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[function_component]
fn App() -> Html {
    let user = use_state(|| {
        Option::<User>::Some(User {
            username: "".to_string(),
            password: "".to_string(),
        })
    });

    html! {
        <BrowserRouter>
            {
                if let Some(_) = &*user {
                    html! {
                        <>
                            <Navbar/>
                            <Switch<MainRoute> render={switch_main} />
                        </>
                    }
                } else {
                    html! {
                        <>
                            <Switch<LoginRoute> render={switch_login} />
                        </>
                    }
                }
            }
        </BrowserRouter>
    }
}

fn switch_main(routes: MainRoute) -> Html {
    match routes {
        MainRoute::Connect4 => {
            html! { <div class="game-container">
                        <Connect4 />
                        </div>
                    }
        }
        MainRoute::TootOtto => {
            html! { <div class="game-container">
                        <TootOtto />
                    </div>
                }
        }
        MainRoute::Leaderboard => {
            html! { <div class="game-container">
                        <LeaderBoard />
                    </div>
                }
        },
        MainRoute::NotFound => {
            html! { "Page not found." }
        }
    }
}

fn switch_login(routes: LoginRoute) -> Html {
    match routes {
        LoginRoute::Login => {
            html! { <LoginForm /> }
        }
        LoginRoute::Register => {
            html! { <RegisterForm /> }
        }
        LoginRoute::NotFound => {
            html! { "Page not found." }
        }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
