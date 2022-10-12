use yew::prelude::*;
use yew::{function_component, html};
use yew_router::prelude::*;

pub mod home;
pub mod navbar;
pub mod team;
pub mod teams;

pub mod favorite_teams;
pub mod loader;
pub mod route;
pub mod button;

use favorite_teams::FavoriteTeams;
use home::Home;
use navbar::NavBar;
use route::Route;

use team::Team;
use teams::Teams;

use crate::utils::{Fetch, User};

fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! { <Home/> },
        Route::Teams => html! { <Teams /> },
        Route::FavoriteTeams => html! { <FavoriteTeams /> },
        Route::Team { name } => html! { <Team name={name.clone()}  /> },
    }
}

#[function_component(App)]
pub fn app() -> Html {
    // Initials user with nba teams here
    use_effect_with_deps(
        move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let user_id = User::get_user_id();
                if user_id.is_empty() {
                    User::set_user_id();
                    let user_id = User::get_user_id();
                    Fetch::initialize_teams(user_id.to_string()).await;
                }
            });

            || ()
        },
        (),
    );

    html! {
      <BrowserRouter>
        <NavBar />
        <main>
          <Switch<Route> render={Switch::render(switch)} />
        </main>
      </BrowserRouter>
    }
}
