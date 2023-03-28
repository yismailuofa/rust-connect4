use yew::prelude::*;
use yew_router::prelude::*;

use crate::MainRoute;

#[function_component]
pub fn Navbar() -> Html {
    html! {
        <nav class="navbar is-primary" role="navigation" aria-label="main navigation">
            <div class={classes!("navbar-menu")}>
                <div class="navbar-start">
                    <Link<MainRoute> classes={classes!("navbar-item")} to={MainRoute::Connect4}>
                        { "Connect 4" }
                    </Link<MainRoute>>
                    <Link<MainRoute> classes={classes!("navbar-item")} to={MainRoute::TootOtto}>
                        { "TOOT-OTTO" }
                    </Link<MainRoute>>
                    <Link<MainRoute> classes={classes!("navbar-item")} to={MainRoute::Leaderboard}>
                        { "Leaderboard" }
                    </Link<MainRoute>>
                </div>
            </div>
        </nav>
    }
}
