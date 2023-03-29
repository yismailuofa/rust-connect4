use yew::prelude::*;
use gloo_net::http::Request;
use client::Leaderboard;
use log::info;
use wasm_bindgen::JsValue;


// #[derive(Clone, PartialEq, Properties)]
// pub struct Props {
//     // pub onsubmit: Callback<()>,

// }

#[function_component]
pub fn LeaderBoard() -> Html {

    let users = use_state(|| vec![]);
    {
        let users = users.clone();
        use_effect_with_deps(move |_| {
            let users = users.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_users: Vec<Leaderboard> = Request::get("//api/leaderboard")
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();

                users.set(fetched_users);
            });
            || ()
        }, ());
    }

    let user_col: Vec<Leaderboard> = users.iter().map(|user| user.clone()).collect();
    let size = users.len();
    
    html! {
        <>
            <h1>{"Leaderboard"}</h1>
            <p>{format!("Top {} users", size)}</p>
            <div>
                <table>
                    <thead>
                        <tr>
                            <th>{"Username"}</th>
                            <th>{"Wins"}</th>
                            <th>{"Losses"}</th>
                        </tr>
                    </thead>
                    <tbody>
                        {
                            for user_col.iter().map(|user| {
                                html! {
                                    <tr>
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