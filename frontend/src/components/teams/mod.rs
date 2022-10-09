use reqwasm::http::Request;
use types::Team;
use yew::{function_component, html, use_effect_with_deps, use_state, Html};

async fn get_teams() -> Vec<Team> {
    let res = match Request::get("/nba/teams").send().await {
        Ok(res) => res,
        Err(_) => panic!("Error fetching data"),
    };

    let teams: Vec<Team> = match res.json().await {
        Ok(teams) => teams,
        Err(_) => panic!("Error fetching data"),
    };

    teams
}

#[function_component(Teams)]
pub fn teams() -> Html {
    let teams = use_state(|| vec![]);
    {
        let teams = teams.clone();
        use_effect_with_deps(
            move |_| {
                let teams = teams.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let fetched_teams = get_teams().await;
                    teams.set(fetched_teams)
                });
                || ()
            },
            (),
        );
    }

  teams.iter().map(|team| {
      html! {
        <div class="team">
          <h3>{&team.city}</h3>
          <p>{&team.name}</p>
        </div>
      }
    }).collect::<Html>()
    


  
}
