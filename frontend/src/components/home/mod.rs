use yew::{function_component, html, use_effect, use_state, Callback};

#[function_component(Home)]
pub fn home() -> Html {
    let counter = use_state(|| 0);

    let counter_one = counter.clone();
    use_effect(move || {
        // Make a call to DOM API after component is rendered
        log::info!("{}", &format!("You clicked {} times", *counter_one));

        // Perform the cleanup
        || log::info!("{}", &format!("You clicked 0 times"))
    });

    let onclick = {
        let counter = counter.clone();
        Callback::from(move |_| counter.set(*counter + 1))
    };

    html! {
        <button {onclick}>{ format!("Increment to {}", *counter) }</button>
    }
}
