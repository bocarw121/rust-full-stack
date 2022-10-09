use yew::prelude::*;
use yew::{function_component, html};
use yew_router::prelude::*;

pub mod home;
pub mod navbar;
pub mod team;
pub mod teams;

pub mod favorite_teams;
pub mod route;

use favorite_teams::FavoriteTeams;
use home::Home;
use navbar::NavBar;
use route::Route;

use team::Team;
use teams::Teams;

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
    html! {
      <BrowserRouter>
      
        <NavBar />
        <main>
          <Switch<Route> render={Switch::render(switch)} />
        </main>
      </BrowserRouter>
    }
}
