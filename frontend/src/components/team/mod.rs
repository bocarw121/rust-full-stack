
use yew::{
    function_component, html, use_effect_with_deps, use_state_eq,
    Properties
};

use super::{loader::Loader, };
use crate::utils::{Fetch, User};
use crate::components::button::Button;

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

                    team.set(fetched_team.data.clone());
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
