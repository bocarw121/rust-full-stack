use yew::{function_component, html, use_effect_with_deps, use_state, Html};
use yew_router::prelude::Link;

use super::{loader::Loader, route::Route};

use crate::utils::Fetch;

#[function_component(Teams)]
pub fn teams() -> Html {
    let teams = use_state(|| vec![]);
    {
        let teams = teams.clone();
        use_effect_with_deps(
            move |_| {
                let teams = teams.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let fetched_teams = Fetch::get_teams("/nba/teams".to_string()).await;
                    teams.set(fetched_teams)
                });
                || ()
            },
            (),
        );
    }

    let teams_html = teams.iter().map(|team| {
      html! {
        <Link<Route> to={Route::Team { name: team.name.clone().to_lowercase() }} >
        <div class="teams-item">
          <h3>{&team.city}</h3>
          <p>{&team.name}</p>
          <img class="team-logo" src={team.logo.clone()} alt={format!("Team {}", &team.logo)} />
          </div>

        </Link<Route>>
      }
    }).collect::<Html>();

    if teams.len() == 0 {
        return html! {
             <Loader />
        };
    }

    html! {
      <div class="teams-display">
        { teams_html }
      </div>
    }
}
