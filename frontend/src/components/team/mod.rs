use serde::__private::de;
use yew::{function_component, html, use_effect_with_deps, use_state, Callback, Properties};

use super::loader::Loader;
use crate::utils::{Fetch, User};
use gloo_storage::{LocalStorage, Storage};
#[derive(Properties, PartialEq)]
pub struct Props {
    pub name: String,
}

#[function_component(Team)]
pub fn team(props: &Props) -> Html {
    let team = use_state(|| types::Team::default());

    let props = props.clone();
    let team = team.clone();

    {
        let team = team.clone();
        let path = format!("/nba/teams/{}?user_id={}", props.name.clone(), User::get_user_id());
        use_effect_with_deps(
            move |_| {
                let team = team.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let fetched_team = Fetch::get_team(path.clone()).await;

                    team.set(fetched_team)
                });
                || ()
            },
            (),
        );
    }

    // Show spinner if data hasn't arrived yet
    if team.name == types::Team::default().name {
        return html! {
              <Loader />

        };
    }

    // Added logic to store favorite items to local storage
    let user_id: String = match LocalStorage::get("user_id") {
        Ok(value) => value,
        Err(_) => "".to_owned(),
    };

    let onclick = {
        let team = team.clone();

        Callback::from(move |_| {
            let user_id = user_id.clone();
            let team = team.clone();
            wasm_bindgen_futures::spawn_local(async move {
                Fetch::post_team(
                    "/nba/favorite".to_string(),
                    types::FavTeamPayload {
                        user_id,
                        team_name: team.name.clone(),
                    },
                )
                .await
            });
        })
    };

 
  

    let favorite_text = if team.is_favorite {
        "Add to favorites"
    } else {
        "Added to favorite"
    };

 





    log::info!("isfavorite: {:?} ", team.is_favorite);
    html! {
      <div class="team">
         <h1>{&team.city}</h1>
         <h2>{&team.name}</h2>
         <img class="team-logo-large" src={team.logo.clone()} alt={format!("Team {}", &team.logo)} />
            <button {onclick} disabled={team.is_favorite} >{favorite_text}</button>
      </div>
    }
}
