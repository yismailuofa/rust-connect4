use yew::prelude::*;


// #[derive(Clone, PartialEq, Properties)]
// pub struct Props {
//     // pub onsubmit: Callback<()>,

// }

#[function_component]
pub fn LoginForm() -> Html {
    html! {
        <form>
            <div>
                <label for="username">{"Username"}</label>
                <input type="text" name="username" />
            </div>
            <div>
                <label for="password">{"Password"}</label>
                <input type="password" name="password" />
            </div>
            <div>
                <button type="submit">{"Login"}</button>
            </div>
        </form>
    }
}