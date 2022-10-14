use super::{route::Route};
use crate::utils::Fetch;
use yew::{function_component, html, use_effect_with_deps, use_state, Html};
use yew_router::prelude::Link;

#[function_component(FavoriteTeams)]
pub fn favorite_teams() -> Html {
    let fav_teams = use_state(|| vec![]);
    {
        let fav_teams = fav_teams.clone();
        use_effect_with_deps(
            move |_| {
                let fav_teams = fav_teams.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let fetched_team = Fetch::get_fav_teams().await;

                    fav_teams.set(fetched_team.data.to_owned())
                });
                || ()
            },
            (),
        );
    }


    // Turn this into a reusable component
    let fav_html = fav_teams
        .iter()
        .map(|fav_teams| {
             fav_teams.team.iter().map(|team| {
                   html! {
              <Link<Route> to={Route::Team { name: team.name.clone().to_lowercase() }} >

            <div class="teams-item">
              <h3>{&team.city}</h3>
              <p>{&team.name}</p>
              <img class="team-logo" src={team.logo.clone()} alt={format!("Team {}", &team.logo)} />
              </div>

            </Link<Route>>
            }
             }).collect::<Html>()   
        }).collect::<Html>()  
        ;



    html! {
      <>
      <h1 class="center">{"Favorite Teams"}</h1>
       <div class="teams-display">
        {fav_html}
      </div>

      </>

    }
}


