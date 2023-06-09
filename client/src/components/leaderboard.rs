use client::Leaderboard;
use gloo_net::http::Request;
use yew::prelude::*;

#[function_component]
pub fn LeaderBoard() -> Html {
    let to_users = use_state(|| vec![]);
    {
        let to_users = to_users.clone();
        use_effect_with_deps(
            move |_| {
                let to_users = to_users.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let fetched_to_users: Vec<Leaderboard> =
                        Request::get("http://127.0.0.1:8000/leaderboard/tootandotto")
                            .send()
                            .await
                            .unwrap()
                            .json()
                            .await
                            .unwrap();

                    to_users.set(fetched_to_users);
                });
                || ()
            },
            (),
        );
    }

    let c4_users = use_state(|| vec![]);
    {
        let c4_users = c4_users.clone();
        use_effect_with_deps(
            move |_| {
                let c4_users = c4_users.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let fetched_c4_users: Vec<Leaderboard> =
                        Request::get("http://127.0.0.1:8000/leaderboard/connect4")
                            .send()
                            .await
                            .unwrap()
                            .json()
                            .await
                            .unwrap();

                    c4_users.set(fetched_c4_users);
                });
                || ()
            },
            (),
        );
    }

    let to_user_col: Vec<Leaderboard> = to_users
        .iter()
        .map(|user| user.clone())
        .filter(|user| !user.username.contains("CPU") && !user.username.contains("AI"))
        .collect();
    let c4_user_col: Vec<Leaderboard> = c4_users
        .iter()
        .map(|user| user.clone())
        .filter(|user| !user.username.contains("CPU") && !user.username.contains("AI"))
        .collect();

    html! {
        <>
            <h1>{"Leaderboard"}</h1>
            <div class = "leaderboard">
                <div>
                    <h2>{"Toot & Otto"}</h2>
                    <table>
                        <thead>
                            <tr>
                                <th>{"Rank"}</th>
                                <th>{"Username"}</th>
                                <th>{"Wins"}</th>
                                <th>{"Losses"}</th>
                            </tr>
                        </thead>
                        <tbody>
                            {
                                for to_user_col.iter().enumerate().map(|(i, user)| {
                                    html! {
                                        <tr>
                                            {
                                                if i == 0 {
                                                    html! {
                                                        <td>{"🥇"}</td>
                                                    }
                                                } else if i == 1 {
                                                    html! {
                                                        <td>{"🥈"}</td>
                                                    }
                                                } else if i == 2 {
                                                    html! {
                                                        <td>{"🥉"}</td>
                                                    }
                                                } else {
                                                    html! {
                                                        <td>{i+1}</td>
                                                    }
                                                }
                                            }
                                            {
                                                if i == 0 {
                                                    html! {
                                                        <>
                                                        <td id="first">{&user.username}</td>
                                                        <td id="first">{&user.wins}</td>
                                                        <td id="first">{&user.losses}</td>
                                                        </>
                                                    }
                                                } else {
                                                    html! {
                                                        <>
                                                        <td>{&user.username}</td>
                                                        <td>{&user.wins}</td>
                                                        <td>{&user.losses}</td>
                                                        </>
                                                    }
                                                }
                                            }
                                        </tr>
                                    }
                                })
                            }
                        </tbody>
                    </table>

                </div>
                <div>
                    <h2>{"Connect 4"}</h2>
                    <table>
                        <thead>
                            <tr>
                                <th>{"Rank"}</th>
                                <th>{"Username"}</th>
                                <th>{"Wins"}</th>
                                <th>{"Losses"}</th>
                            </tr>
                        </thead>
                        <tbody>
                            {
                                for c4_user_col.iter().enumerate().map(|(i, user)| {
                                    html! {
                                        <tr>
                                            {
                                                if i == 0 {
                                                    html! {
                                                        <td>{"🥇"}</td>
                                                    }
                                                } else if i == 1 {
                                                    html! {
                                                        <td>{"🥈"}</td>
                                                    }
                                                } else if i == 2 {
                                                    html! {
                                                        <td>{"🥉"}</td>
                                                    }
                                                } else {
                                                    html! {
                                                        <td>{i+1}</td>
                                                    }
                                                }
                                            }
                                            {
                                                if i == 0 {
                                                    html! {
                                                        <>
                                                        <td id="first">{&user.username}</td>
                                                        <td id="first">{&user.wins}</td>
                                                        <td id="first">{&user.losses}</td>
                                                        </>
                                                    }
                                                } else {
                                                    html! {
                                                        <>
                                                        <td>{&user.username}</td>
                                                        <td>{&user.wins}</td>
                                                        <td>{&user.losses}</td>
                                                        </>
                                                    }
                                                }
                                            }
                                        </tr>
                                    }
                                })
                            }
                        </tbody>
                    </table>

                </div>
            </div>
        </>
    }
}
