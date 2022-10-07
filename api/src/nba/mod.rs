use types::{NBATeams, Team};

// Returns a vec with all the nba teams
pub(crate) fn get_teams() -> Vec<Team> {
    let nba = NBATeams {
        teams: vec![
            Team {
                id: 1,
                name: "Lakers".to_string(),
                city: "Los Angeles".to_string(),
            },
            Team {
                id: 2,
                name: "Celtics".to_string(),
                city: "Boston".to_string(),
            },
            Team {
                id: 3,
                name: "Bucks".to_string(),
                city: "Milwaukee".to_string(),
            },
            Team {
                id: 4,
                name: "Raptors".to_string(),
                city: "Toronto".to_string(),
            },
            Team {
                id: 5,
                name: "Nets".to_string(),
                city: "Brooklyn".to_string(),
            },
            Team {
                id: 6,
                name: "76ers".to_string(),
                city: "Philadelphia".to_string(),
            },
            Team {
                id: 7,
                name: "Heat".to_string(),
                city: "Miami".to_string(),
            },
            Team {
                id: 8,
                name: "Pacers".to_string(),
                city: "Indiana".to_string(),
            },
            Team {
                id: 9,
                name: "Magic".to_string(),
                city: "Orlando".to_string(),
            },
            Team {
                id: 10,
                name: "Wizards".to_string(),
                city: "Washington".to_string(),
            },
            Team {
                id: 11,
                name: "Hawks".to_string(),
                city: "Atlanta".to_string(),
            },
            Team {
                id: 12,
                name: "Bulls".to_string(),
                city: "Chicago".to_string(),
            },
            Team {
                id: 13,
                name: "Cavaliers".to_string(),
                city: "Cleveland".to_string(),
            },
            Team {
                id: 14,
                name: "Knicks".to_string(),
                city: "New York".to_string(),
            },
            Team {
                id: 15,
                name: "Pistons".to_string(),
                city: "Detroit".to_string(),
            },
            Team {
                id: 16,
                name: "Hornets".to_string(),
                city: "Charlotte".to_string(),
            },
            Team {
                id: 17,
                name: "Nuggets".to_string(),
                city: "Denver".to_string(),
            },
            Team {
                id: 18,
                name: "Timberwolves".to_string(),
                city: "Minnesota".to_string(),
            },
            Team {
                id: 19,
                name: "Thunder".to_string(),
                city: "Oklahoma City".to_string(),
            },
            Team {
                id: 20,
                name: "Trail Blazers".to_string(),
                city: "Portland".to_string(),
            },
            Team {
                id: 21,
                name: "Jazz".to_string(),
                city: "Utah".to_string(),
            },
            Team {
                id: 22,
                name: "Warriors".to_string(),
                city: "Golden State".to_string(),
            },
            Team {
                id: 23,
                name: "Clippers".to_string(),
                city: "Los Angeles".to_string(),
            },
            Team {
                id: 24,
                name: "Suns".to_string(),
                city: "Phoenix".to_string(),
            },
            Team {
                id: 25,
                name: "Kings".to_string(),
                city: "Sacramento".to_string(),
            },
            Team {
                id: 26,
                name: "Mavericks".to_string(),
                city: "Dallas".to_string(),
            },
            Team {
                id: 27,
                name: "Rockets".to_string(),
                city: "Houston".to_string(),
            },
            Team {
                id: 28,
                name: "Grizzlies".to_string(),
                city: "Memphis".to_string(),
            },
            Team {
                id: 29,
                name: "Pelicans".to_string(),
                city: "New Orleans".to_string(),
            },
            Team {
                id: 30,
                name: "Spurs".to_string(),
                city: "San Antonio".to_string(),
            },
        ],
    };

    nba.teams
}
