use types::{NBATeams, Team};

// Returns a vec with all the nba
pub(crate) fn get_teams(_id: String) -> Vec<Team> {
    let nba = NBATeams {
        _id,
        teams: vec![
            Team {
                _id: 1,
                name: "Lakers".to_string(),
                city: "Los Angeles".to_string(),
                logo: "http://i.cdn.turner.com/nba/nba/.element/img/1.0/teamsites/logos/teamlogos_500x500/lal.png".to_string(),
                is_favorite: false
            },
            Team {
                _id: 2,
                name: "Celtics".to_string(),
                city: "Boston".to_string(),
                logo: "http://i.cdn.turner.com/nba/nba/.element/img/1.0/teamsites/logos/teamlogos_500x500/bos.png".to_string(),
                is_favorite: false
            },
            Team {
                _id: 3,
                name: "Bucks".to_string(),
                city: "Milwaukee".to_string(),
                logo: "http://i.cdn.turner.com/nba/nba/.element/img/1.0/teamsites/logos/teamlogos_500x500/mil.png".to_string(),
                is_favorite: false
            },
            Team {
                _id: 4,
                name: "Raptors".to_string(),
                city: "Toronto".to_string(),
                logo: "http://i.cdn.turner.com/nba/nba/.element/img/1.0/teamsites/logos/teamlogos_500x500/tor.png".to_string(),
                is_favorite: false
            },
            Team {
                _id: 5,
                name: "Nets".to_string(),
                city: "Brooklyn".to_string(),
                logo: "http://i.cdn.turner.com/nba/nba/.element/img/1.0/teamsites/logos/teamlogos_500x500/bkn.png".to_string(),
                is_favorite: false
            },
            Team {
                _id: 6,
                name: "76ers".to_string(),
                city: "Philadelphia".to_string(),
                logo: "http://i.cdn.turner.com/nba/nba/.element/img/1.0/teamsites/logos/teamlogos_500x500/phi.png".to_string(),
                is_favorite: false
            },
            Team {
                _id: 7,
                name: "Heat".to_string(),
                city: "Miami".to_string(),
                logo: "http://i.cdn.turner.com/nba/nba/.element/img/1.0/teamsites/logos/teamlogos_500x500/mia.png".to_string(),
                is_favorite: false
            },
            Team {
                _id: 8,
                name: "Pacers".to_string(),
                city: "Indiana".to_string(),
                logo: "http://i.cdn.turner.com/nba/nba/.element/img/1.0/teamsites/logos/teamlogos_500x500/ind.png".to_string(),
                is_favorite: false
            },
            Team {
                _id: 9,
                name: "Magic".to_string(),
                city: "Orlando".to_string(),
                logo: "http://i.cdn.turner.com/nba/nba/.element/img/1.0/teamsites/logos/teamlogos_500x500/orl.png".to_string(),
                is_favorite: false
            },
            Team {
                _id: 10,
                name: "Wizards".to_string(),
                city: "Washington".to_string(),
                logo: "http://i.cdn.turner.com/nba/nba/.element/img/1.0/teamsites/logos/teamlogos_500x500/was.png".to_string(),
                is_favorite: false
            },
            Team {
                _id: 11,
                name: "Hawks".to_string(),
                city: "Atlanta".to_string(),
                logo: "http://i.cdn.turner.com/nba/nba/.element/img/1.0/teamsites/logos/teamlogos_500x500/atl.png".to_string(),
                is_favorite: false
            },
            Team {
                _id: 12,
                name: "Bulls".to_string(),
                city: "Chicago".to_string(),
                logo: "http://i.cdn.turner.com/nba/nba/.element/img/1.0/teamsites/logos/teamlogos_500x500/chi.png".to_string(),
                is_favorite: false
            },
            Team {
                _id: 13,
                name: "Cavaliers".to_string(),
                city: "Cleveland".to_string(),
                logo: "http://i.cdn.turner.com/nba/nba/.element/img/1.0/teamsites/logos/teamlogos_500x500/cle.png".to_string(),
                is_favorite: false
            },
            Team {
                _id: 14,
                name: "Knicks".to_string(),
                city: "New York".to_string(),
                logo: "http://i.cdn.turner.com/nba/nba/.element/img/1.0/teamsites/logos/teamlogos_500x500/nyk.png".to_string(),
                is_favorite: false
            },
            Team {
                _id: 15,
                name: "Pistons".to_string(),
                city: "Detroit".to_string(),
                logo: "http://i.cdn.turner.com/nba/nba/.element/img/1.0/teamsites/logos/teamlogos_500x500/det.png".to_string(),
                is_favorite: false
            },
            Team {
                _id: 16,
                name: "Hornets".to_string(),
                city: "Charlotte".to_string(),
                logo: "http://i.cdn.turner.com/nba/nba/.element/img/1.0/teamsites/logos/teamlogos_500x500/cha.png".to_string(),
                is_favorite: false
            },
            Team {
                _id: 17,
                name: "Nuggets".to_string(),
                city: "Denver".to_string(),
                logo: "http://i.cdn.turner.com/nba/nba/.element/img/1.0/teamsites/logos/teamlogos_500x500/den.png".to_string(),
                is_favorite: false
            },
            Team {
                _id: 18,
                name: "Timberwolves".to_string(),
                city: "Minnesota".to_string(),
                logo: "http://i.cdn.turner.com/nba/nba/.element/img/1.0/teamsites/logos/teamlogos_500x500/min.png".to_string(),
                is_favorite: false
            },
            Team {
                _id: 19,
                name: "Thunder".to_string(),
                city: "Oklahoma City".to_string(),
                logo: "http://i.cdn.turner.com/nba/nba/.element/img/1.0/teamsites/logos/teamlogos_500x500/okc.png".to_string(),
                is_favorite: false
            },
            Team {
                _id: 20,
                name: "Trail Blazers".to_string(),
                city: "Portland".to_string(),
                logo: "http://i.cdn.turner.com/nba/nba/.element/img/1.0/teamsites/logos/teamlogos_500x500/por.png".to_string(),
                is_favorite: false
            },
            Team {
                _id: 21,
                name: "Jazz".to_string(),
                city: "Utah".to_string(),
                logo: "http://i.cdn.turner.com/nba/nba/.element/img/1.0/teamsites/logos/teamlogos_500x500/uta.png".to_string(),
                is_favorite: false
            },
            Team {
                _id: 22,
                name: "Warriors".to_string(),
                city: "Golden State".to_string(),
                logo: "http://i.cdn.turner.com/nba/nba/.element/img/1.0/teamsites/logos/teamlogos_500x500/gsw.png".to_string(),
                is_favorite: false
            },
            Team {
                _id: 23,
                name: "Clippers".to_string(),
                city: "Los Angeles".to_string(),
                logo: "http://i.cdn.turner.com/nba/nba/.element/img/1.0/teamsites/logos/teamlogos_500x500/lac.png".to_string(),
                is_favorite: false
            },
            Team {
                _id: 24,
                name: "Suns".to_string(),
                city: "Phoenix".to_string(),
                logo: "http://i.cdn.turner.com/nba/nba/.element/img/1.0/teamsites/logos/teamlogos_500x500/phx.png".to_string(),
                is_favorite: false
            },
            Team {
                _id: 25,
                name: "Kings".to_string(),
                city: "Sacramento".to_string(),
                logo: "http://i.cdn.turner.com/nba/nba/.element/img/1.0/teamsites/logos/teamlogos_500x500/sac.png".to_string(),
                is_favorite: false
            },
            Team {
                _id: 26,
                name: "Mavericks".to_string(),
                city: "Dallas".to_string(),
                logo: "http://i.cdn.turner.com/nba/nba/.element/img/1.0/teamsites/logos/teamlogos_500x500/dal.png".to_string(),
                is_favorite: false
            },
            Team {
                _id: 27,
                name: "Rockets".to_string(),
                city: "Houston".to_string(),
                logo: "http://i.cdn.turner.com/nba/nba/.element/img/1.0/teamsites/logos/teamlogos_500x500/hou.png".to_string(),
                is_favorite: false
            },
            Team {
                _id: 28,
                name: "Grizzlies".to_string(),
                city: "Memphis".to_string(),
                logo: "http://i.cdn.turner.com/nba/nba/.element/img/1.0/teamsites/logos/teamlogos_500x500/mem.png".to_string(),
                is_favorite: false
            },
            Team {
                _id: 29,
                name: "Pelicans".to_string(),
                city: "New Orleans".to_string(),
                logo: "http://i.cdn.turner.com/nba/nba/.element/img/1.0/teamsites/logos/teamlogos_500x500/nop.png".to_string(),
                is_favorite: false
            },
            Team {
                _id: 30,
                name: "Spurs".to_string(),
                city: "San Antonio".to_string(),
                logo: "http://i.cdn.turner.com/nba/nba/.element/img/1.0/teamsites/logos/teamlogos_500x500/sas.png".to_string(),
                is_favorite: false
            },
        ],
    };

    nba.teams
}
