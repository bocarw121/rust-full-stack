use yew::{function_component, Properties, use_state, Callback, html};

use crate::utils::Fetch;


#[derive(Properties, PartialEq)]
pub struct ButtonProps {
   pub team: types::Team,
}

#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
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
                    
                    Fetch::delete_favorite_team(team.name).await
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
