use yew::prelude::*;
use yew_router::prelude::*;

mod components;
use crate::components::connect4::{Connect4, TootOtto};
use crate::components::leaderboard::LeaderBoard;
use crate::components::login_form::LoginForm;
use crate::components::navbar::Navbar;
use crate::components::register_form::RegisterForm;
use client::User;

#[derive(Routable, PartialEq, Eq, Clone, Debug)]
pub enum MainRoute {
    #[at("/")]
    Connect4,
    #[at("/toot-otto")]
    TootOtto,
    #[at("/leaderboard")]
    Leaderboard,
    #[at("/logout")]
    Logout,
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

#[function_component()]
fn App() -> Html {
    let user = use_state(|| {
        Option::<User>::Some({
            User {
                username: "test".to_string(),
                password: "test".to_string(),
            }
        })
    });

    let user_clone = user.clone();
    let update_user = Callback::from(move |new_user: Option<User>| user_clone.set(new_user));

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
                            <Switch<LoginRoute> render={switch_login(update_user)} />
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
        }
        MainRoute::NotFound => {
            html! { "Page not found." }
        }
        MainRoute::Logout => todo!(),
    }
}

fn switch_login(on_login: Callback<Option<User>>) -> impl Fn(LoginRoute) -> Html {
    move |routes: LoginRoute| match routes {
        LoginRoute::Login => {
            html! { <LoginForm set_user={on_login.clone()} /> }
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
