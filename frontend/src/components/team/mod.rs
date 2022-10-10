use serde::__private::de;
use yew::{function_component, html, use_effect_with_deps, use_state, Properties, Callback};

use super::loader::Loader;
use crate::utils::Fetch;
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
        let path = format!("/nba/teams/{}", props.name.clone());
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

    
    
    let storage_key = format!("is_favorite-{}",&team.name);
      // Added logic to store favorite items to local storage
       let is_favorite: String = match  LocalStorage::get(format!("is_favorite-{}",&team.name)) {
        Ok(value) => value,
        Err(_) => "".to_owned(),
   };
   
    let onclick = {
        let team = team.clone();
        
        Callback::from(move |_| {
                 LocalStorage::set(storage_key.clone(), &team.name).unwrap();
               let team = team.clone();
            wasm_bindgen_futures::spawn_local(async move  {
                Fetch::post_team("/nba/favorite".to_string(), types::FavTeamPayload { user_name: "user1".to_string(), team_name: team.name.clone() }).await
        });
        })
    };

    let storage_key = format!("is_favorite-{}",&team.name);
   let delete = {
        Callback::from(move |_|  LocalStorage::delete(&storage_key.clone()))
    };


    let favorite_text = if !is_favorite.contains(&team.name) {"Add to favorites"} else {"Added to favorite"};

    log::info!("isfavorite: {:?} {:?} ", is_favorite.contains(&team.name), is_favorite, );
    html! {
      <div class="team">
         <h1>{&team.city}</h1>
         <h2>{&team.name}</h2>
         <img class="team-logo-large" src={team.logo.clone()} alt={format!("Team {}", &team.logo)} />
            <button {onclick} >{favorite_text}</button>
            <button onclick={delete}  >{"Remove from favorites"}</button>
      </div>
    }
}



