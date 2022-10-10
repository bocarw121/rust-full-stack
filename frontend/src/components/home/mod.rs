use yew::{function_component, html};

#[function_component(Home)]
pub fn home() -> Html {
    html! {
      <div class="home">  
        <h1>{ "Welcome to NBA List" }</h1>
      </div>
    }
}
