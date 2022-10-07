use types::{Team, NBATeams};

pub(crate) fn get_teams() -> Vec<Team> {
 let nba = NBATeams {
    teams: vec![
        Team {
            id: 1,
            team: "Lakers".to_string(),
            city: "Los Angeles".to_string(),
        },
        Team {
            id: 2,
            team: "Celtics".to_string(),
            city: "Boston".to_string(),
        },
        Team {
            id: 3,
            team: "Bucks".to_string(),
            city: "Milwaukee".to_string(),
        },
        Team {
            id: 4,
            team: "Raptors".to_string(),
            city: "Toronto".to_string(),
        },
        Team {
            id: 5,
            team: "Nets".to_string(),
            city: "Brooklyn".to_string(),
        },
        Team {
            id: 6,
            team: "76ers".to_string(),
            city: "Philadelphia".to_string(),
        },
        Team {
            id: 7,
            team: "Heat".to_string(),
            city: "Miami".to_string(),
        },
        Team {
            id: 8,
            team: "Pacers".to_string(),
            city: "Indiana".to_string(),
        },
        Team {
            id: 9,
            team: "Magic".to_string(),
            city: "Orlando".to_string(),
        },
        Team {
            id: 10,
            team: "Wizards".to_string(),
            city: "Washington".to_string(),
        },
        Team {
            id: 11,
            team: "Hawks".to_string(),
            city: "Atlanta".to_string(),
        },
        Team {
            id: 12,
            team: "Bulls".to_string(),
            city: "Chicago".to_string(),
        },
        Team {
            id: 13,
            team: "Cavaliers".to_string(),
            city: "Cleveland".to_string(),
        },
        Team {
            id: 14,
            team: "Knicks".to_string(),
            city: "New York".to_string(),
        },
        Team {
            id: 15,
            team: "Pistons".to_string(),
            city: "Detroit".to_string(),
        },
        Team {
            id: 16,
            team: "Hornets".to_string(),
            city: "Charlotte".to_string(),
        },
        Team {
            id: 17,
            team: "Nuggets".to_string(),
            city: "Denver".to_string(),
        },
        Team {
            id: 18,
            team: "Timberwolves".to_string(),
            city: "Minnesota".to_string(),
        },
        Team {
            id: 19,
            team: "Thunder".to_string(),
            city: "Oklahoma City".to_string(),
        },
        Team {
            id: 20,
            team: "Trail Blazers".to_string(),
            city: "Portland".to_string(),
        },
        Team {
            id: 21,
            team: "Jazz".to_string(),
            city: "Utah".to_string(),
        },
        Team {
            id: 22,
            team: "Warriors".to_string(),
            city: "Golden State".to_string(),
        },
        Team {
            id: 23,
            team: "Clippers".to_string(),
            city: "Los Angeles".to_string(),
        },
        Team {
            id: 24,
            team: "Suns".to_string(),
            city: "Phoenix".to_string(),
        },
        Team {
            id: 25,
            team: "Kings".to_string(),
            city: "Sacramento".to_string(),
        },
        Team {
            id: 26,
            team: "Mavericks".to_string(),
            city: "Dallas".to_string(),
        },
        Team {
            id: 27,
            team: "Rockets".to_string(),
            city: "Houston".to_string(),
        },
        Team {
            id: 28,
            team: "Grizzlies".to_string(),
            city: "Memphis".to_string(),
        },
        Team {
            id: 29,
            team: "Pelicans".to_string(),
            city: "New Orleans".to_string(),
        },
        Team {
            id: 30,
            team: "Spurs".to_string(),
            city: "San Antonio".to_string(),
        },
      ]
  };

  nba.teams
}
