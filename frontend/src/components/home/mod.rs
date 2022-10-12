use yew::{function_component, html};

#[function_component(Home)]
pub fn home() -> Html {
    html! {
      <div class="home">
        <h1>{ "Pick your favorite nba teams" }</h1>
      </div>
    }
}
