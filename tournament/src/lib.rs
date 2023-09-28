use std::cmp;
use std::collections::HashMap;
use std::fmt;

fn format_row(t: &str, m: &str, w: &str, d: &str, l: &str, p: &str) -> String {
    format!(
        "{:<30} | {:^3}| {:^3}| {:^3}| {:^3}| {:>2}",
        t, m, w, d, l, p
    )
}

#[derive(Clone, Eq, Debug)]
struct Team {
    name: String,
    wins: i32,
    draws: i32,
    losses: i32,
}

impl Team {
    fn new(name: &str) -> Team {
        Team {
            name: name.to_string(),
            wins: 0,
            draws: 0,
            losses: 0,
        }
    }

    fn record_win(&mut self) {
        self.wins += 1;
    }

    fn record_draw(&mut self) {
        self.draws += 1;
    }

    fn record_loss(&mut self) {
        self.losses += 1;
    }
    fn played(&self) -> i32 {
        self.wins + self.draws + self.losses
    }

    fn points(&self) -> i32 {
        self.wins * 3 + self.draws
    }
}

impl fmt::Display for Team {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            format_row(
                &self.name,
                &self.played().to_string(),
                &self.wins.to_string(),
                &self.draws.to_string(),
                &self.losses.to_string(),
                &self.points().to_string(),
            )
        )
    }
}

impl Ord for Team {
    fn cmp(&self, other: &Team) -> cmp::Ordering {
        self.points()
            .cmp(&other.points())
            .reverse() // Higher points come first
            .then_with(|| self.name.cmp(&other.name)) // "Higher" letter comes last
    }
}

impl PartialOrd for Team {
    fn partial_cmp(&self, other: &Team) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Team {
    fn eq(&self, other: &Team) -> bool {
        self.points() == other.points()
    }
}

struct LeagueTable {
    teams: HashMap<String, Team>,
}

impl LeagueTable {
    fn new() -> LeagueTable {
        LeagueTable {
            teams: HashMap::new(),
        }
    }

    fn get_mut_team_result(&mut self, team: &str) -> &mut Team {
        self.teams
            .entry(team.to_string())
            .or_insert_with(|| Team::new(team))
    }

    fn record_win(&mut self, winner_name: &str, loser_name: &str) {
        self.get_mut_team_result(winner_name).record_win();
        self.get_mut_team_result(loser_name).record_loss();
    }

    fn record_draw(&mut self, team1_name: &str, team2_name: &str) {
        self.get_mut_team_result(team1_name).record_draw();
        self.get_mut_team_result(team2_name).record_draw();
    }

    fn rows(&self) -> Vec<String> {
        let mut teams: Vec<&Team> = self.teams.values().to_owned().collect();
        teams.sort();
        teams.into_iter().map(|team| team.to_string()).collect()
    }
}

pub fn tally(match_results: &str) -> String {
    let mut league_table = LeagueTable::new();
    for result in match_results.split('\n') {
        match result.split(';').collect::<Vec<&str>>()[..] {
            [winner, loser, "win"] => league_table.record_win(winner, loser),
            [loser, winner, "loss"] => league_table.record_win(winner, loser),
            [team1, team2, "draw"] => league_table.record_draw(team1, team2),
            _ => println!("Not implemented"),
        }
    }

    vec![format_row("Team", "MP", "W", "D", "L", "P")] // Table header
        .into_iter()
        .chain(league_table.rows()) // Table rows
        .collect::<Vec<String>>()
        .join("\n")
}
