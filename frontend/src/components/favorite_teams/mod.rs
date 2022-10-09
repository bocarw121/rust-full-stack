use yew::{function_component, html};
use yew_router::prelude::Link;

#[function_component(FavoriteTeams)]
pub fn favorite_teams() -> Html {
    html! {
      <h1>{ "Favorite Teams" }</h1>
    }
}
