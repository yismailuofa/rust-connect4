use yew::prelude::*;
use yew_router::prelude::*;

use crate::MainRoute;

#[function_component]
pub fn Navbar() -> Html {
    html! {
        <nav class="navbar">
            <div>
                <div class="navbar-links">
                    <Link<MainRoute> classes={classes!("navbar-item")} to={MainRoute::Connect4}>
                        { "Connect 4" }
                    </Link<MainRoute>>
                    <Link<MainRoute> classes={classes!("navbar-item")} to={MainRoute::TootOtto}>
                        { "Toot & Otto" }
                    </Link<MainRoute>>
                    <Link<MainRoute> classes={classes!("navbar-item")} to={MainRoute::Leaderboard}>
                        { "Leaderboard" }
                    </Link<MainRoute>>
                    <Link<MainRoute> classes={classes!("navbar-item")} to={MainRoute::Logout}>
                        { "Logout" }
                    </Link<MainRoute>>
                </div>
            </div>
        </nav>
    }
}
