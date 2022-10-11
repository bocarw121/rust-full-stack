use yew::{function_component, html};
use yew_router::prelude::Link;

use super::route::Route;

#[function_component(NavBar)]
pub fn nav_bar() -> Html {
    html! {
    <>
    <nav>
        <h1 class="logo"><Link<Route> to={Route::Home}>{"NBA TEAMS"}</Link<Route>></h1>
        <div class="nav-links">
          <Link<Route> to={Route::Teams}>{ "Teams" }</Link<Route>>
          <Link<Route> to={Route::FavoriteTeams}>{ "Favorite Teams" }</Link<Route>>
        </div>
      </nav>
    </>
    }
}
