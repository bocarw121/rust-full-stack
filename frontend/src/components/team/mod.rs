
use yew::{
    function_component, html, use_effect_with_deps, use_state, use_state_eq, Callback,
    Properties
};

use super::loader::Loader;
use crate::utils::{Fetch, User};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub name: String,
}

#[function_component(Team)]
pub fn team(props: &Props) -> Html {
    let team = use_state_eq::<types::Team, _>(|| types::Team::default());

    let props = props.clone();
    let team = team.clone();

    {
        let team = team.clone();
        let path = format!(
            "/nba/teams/{}?user_id={}",
            props.name.clone(),
            User::get_user_id()
        );
        use_effect_with_deps(
            move |_| {
                let team = team.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let fetched_team = Fetch::get_team(path.clone()).await;

                    team.set(fetched_team.clone());
                });
                || ()
            },
            (),
        );
    }

    // Show spinner if data hasn't arrived yet
    if team.logo == types::Team::default().logo {
        return html! {
              <Loader />
        };
    }


    html! {
      <div class="team">
         <h1>{&team.city}</h1>
         <h2>{&team.name}</h2>
         <img class="team-logo-large" src={team.logo.clone()} alt={format!("Team {}", &team.logo)} />
            <Button team={(*team).clone()} />
      </div>
    }
}

#[derive(Properties, PartialEq)]
struct ButtonProps {
    team: types::Team,
}

#[function_component(Button)]
fn button(props: &ButtonProps) -> Html {
    let team = props.team.clone();
    // Initialize state with the team.is_favorite boolean
    let is_favorite = use_state(|| team.is_favorite);
    log::info!(
        "is_favorite{} {}",
        *is_favorite.clone(),
        props.team.is_favorite.clone()
    );

    let onclick = {
        let team = props.team.clone();
        let is_favorite = is_favorite.clone();

        Callback::from(move |_| {
            let team = team.clone();
            // Toggle the state of is_favorite with each click
            is_favorite.set(!*is_favorite);
            let is_favorite = *is_favorite.clone();
            
            wasm_bindgen_futures::spawn_local(async move {
                // toggles the favorite property for the team
                // in the server
                Fetch::update_favorite_team(team.name.clone()).await;
                
                // Team should only be posted to favorites list
                // if it is false
                if is_favorite == false {
                    // Add to favorites list
                    log::info!("Its false");
                    Fetch::post_fav_team(team.name).await
                } else {
                    // delete the favorite team
                }
            });
        })
    };

    let favorite_text = if *is_favorite == false {
        "Add to favorites"
    } else {
        "Remove from favorites"
    };

    html! {
    <button {onclick}  >{favorite_text}</button>
    }
}
