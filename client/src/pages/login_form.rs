use yew::prelude::*;


// #[derive(Clone, PartialEq, Properties)]
// pub struct Props {
//     // pub onsubmit: Callback<()>,

// }

#[function_component]
pub fn LoginForm() -> Html {
    html! {
        <form>
            <label for="username">{"Username"}</label>
            <input type="text" name="username" />
            <label for="password">{"Password"}</label>
            <input type="password" name="password" />
            <button type="submit">{"Login"}</button>
        </form>
    }
}