use client::Leaderboard;
use gloo_net::http::Request;
use log::info;
use wasm_bindgen::JsValue;
use yew::prelude::*;

// #[derive(Clone, PartialEq, Properties)]
// pub struct Props {
//     // pub onsubmit: Callback<()>,

// }

#[function_component]
pub fn LeaderBoard() -> Html {

    let to_users = use_state(|| vec![]);
    {
        let to_users = to_users.clone();
        use_effect_with_deps(move |_| {
            let to_users = to_users.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_to_users: Vec<Leaderboard> = Request::get("http://127.0.0.1:8000/leaderboard/tootandotto")
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();

                    to_users.set(fetched_to_users);
            });
            || ()
        }, ());
    }

    let c4_users = use_state(|| vec![]);
    {
        let c4_users = c4_users.clone();
        use_effect_with_deps(move |_| {
            let c4_users = c4_users.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_c4_users: Vec<Leaderboard> = Request::get("http://127.0.0.1:8000/leaderboard/connect4")
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();

                    c4_users.set(fetched_c4_users);
            });
            || ()
        }, ());
    }

    let to_user_col: Vec<Leaderboard> = to_users.iter().map(|user| user.clone()).collect();
    let c4_user_col: Vec<Leaderboard> = c4_users.iter().map(|user| user.clone()).collect();

    html! {
        <>
            <h1>{"Leaderboard"}</h1>
            <p>{"Toot & Otto"}</p>
            <div>
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
                                        <td>{i+1}</td>
                                        <td>{&user.username}</td>
                                        <td>{&user.wins}</td>
                                        <td>{&user.losses}</td>
                                    </tr>
                                }
                            })
                        }
                    </tbody>
                </table>
                
            </div>
            <p>{"Connect 4"}</p>
            <div>
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
                                        <td>{i+1}</td>
                                        <td>{&user.username}</td>
                                        <td>{&user.wins}</td>
                                        <td>{&user.losses}</td>
                                    </tr>
                                }
                            })
                        }
                    </tbody>
                </table>
                
            </div>
        </>
    }
}
