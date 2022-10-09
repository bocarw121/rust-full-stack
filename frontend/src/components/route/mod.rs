use yew_router::Routable;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at["/"]]
    Home,
    #[at["/teams"]]
    Teams,
    #[at["/teams/:name"]]
    Team { name: String },
    #[at["/favorite"]]
    FavoriteTeams,
}
