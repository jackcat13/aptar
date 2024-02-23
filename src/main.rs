mod helpers;
mod components;

use crate::helpers::local_storage::local_storage;
use yew::functional::*;
use yew::prelude::*;
use yew_router::prelude::*;
use crate::components::login::LoginComponent;

const USER_SESSION: &str = "user-session";

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn home() -> Html {
    html! {
        <>
            {header()}

            {footer()}
        </>
    }
}

fn header() -> Html {
    let local_storage = local_storage();
    let user = local_storage
        .get_item(USER_SESSION)
        .expect("Failed to load user from storage");
    //TODO proper profile panel
    html! {
        <>
            <div class="absolute top-0 right-0 m-5">
                {"Welcome "}{user}
            </div>
        </>
    }
}

fn footer() -> Html {
    html! {
        <>
            <div class="sticky bottom-0 w-full">
                <div class="px-8 place-items-center flex justify-center">

                </div>
            </div>
        </>
    }
}

fn not_found() -> Html {
    html! { <h1>{ "404" }</h1> }
}

fn login() -> Html {
    html! {
        <>
            <LoginComponent />
            {footer()}
        </>
    }
}

fn switch(routes: Route) -> Html {
    let local_storage = local_storage();
    let user_session = local_storage.get(USER_SESSION).unwrap();
    match user_session {
        Some(_) => match routes {
            Route::Home => home(),
            Route::NotFound => not_found(),
        },
        None => login(),
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <HashRouter>
                <Switch<Route> render={switch} />
            </HashRouter>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}