use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;

#[function_component]
pub fn Navbar() -> Html {
    let navbar_active = use_state_eq(|| false);


    let active_class = if !*navbar_active { "is-active" } else { "" };

    html! {
        <nav class="navbar is-primary" role="navigation" aria-label="main navigation">
            <div class={classes!("navbar-menu", active_class)}>
                <div class="navbar-start">
                    <Link<Route> classes={classes!("navbar-item")} to={Route::Connect4}>
                        { "Connect 4" }
                    </Link<Route>>
                    <Link<Route> classes={classes!("navbar-item")} to={Route::TootOtto}>
                        { "TOOT-OTTO" }
                    </Link<Route>>
                    <Link<Route> classes={classes!("navbar-item")} to={Route::Leaderboard}>
                        { "Leaderboard" }
                    </Link<Route>>
                    <Link<Route> classes={classes!("navbar-item")} to={Route::Login}>
                        { "Login" }
                    </Link<Route>>
                    <Link<Route> classes={classes!("navbar-item")} to={Route::Register}>
                        { "Sign Up" }
                    </Link<Route>>
                </div>
            </div>
        </nav>
    }
}