use yew::{function_component, html, use_effect_with_deps, use_state, Properties};

use super::loader::Loader;
use crate::utils::Fetch;

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

    log::info!("city: {:?}", &team.city);
    html! {
      <div class="team">
        <h1>{"Team"}</h1>
         <img class="team-logo-large" src={team.logo.clone()} alt={format!("Team {}", &team.logo)} />
        <p>{&team.name}</p>
        <p>{&team.city}</p>
      </div>
    }
}
