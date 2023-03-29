use yew::prelude::*;
use yew_router::prelude::*;

mod components;
use crate::components::connect4::{Connect4, TootOtto};
use crate::components::leaderboard::LeaderBoard;
use crate::components::login_form::LoginForm;
use crate::components::navbar::Navbar;
use crate::components::register_form::RegisterForm;

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
    let username = use_state(|| Option::<String>::None);
    // let username = use_state(|| Option::<String>::Some("rupin".to_string()));

    let user_clone = username.clone();
    let update_username = Callback::from(move |new_user: Option<String>| user_clone.set(new_user));

    html! {
        <BrowserRouter>
            {
                if let Some(_) = &*username {
                    html! {
                        <>
                            <Navbar/>
                            <Switch<MainRoute> render={switch_main(update_username)} />
                        </>
                    }
                } else {
                    html! {
                        <>
                            <Switch<LoginRoute> render={switch_login(update_username)} />
                        </>
                    }
                }
            }
        </BrowserRouter>
    }
}

fn switch_main(set_user: Callback<Option<String>>) -> impl Fn(MainRoute) -> Html {
    move |routes: MainRoute| match routes {
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
        MainRoute::Logout => {
            set_user.emit(None);

            html! {<Redirect<LoginRoute> to={LoginRoute::Login}/>}
        }
    }
}

fn switch_login(on_login: Callback<Option<String>>) -> impl Fn(LoginRoute) -> Html {
    move |routes: LoginRoute| match routes {
        LoginRoute::Login => {
            html! { <LoginForm set_username={on_login.clone()} /> }
        }
        LoginRoute::Register => {
            html! { <RegisterForm set_username={on_login.clone()}/> }
        }
        LoginRoute::NotFound => {
            html! { "Page not found." }
        }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
