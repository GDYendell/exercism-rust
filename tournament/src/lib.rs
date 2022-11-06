use std::cmp;
use std::collections::HashMap;

fn format_row(t: &str, m: &str, w: &str, d: &str, l: &str, p: &str) -> String {
    format!(
        "{:<30} | {:^3}| {:^3}| {:^3}| {:^3}| {:>2}",
        t, m, w, d, l, p
    )
}

#[derive(Clone, Eq)]
struct TeamResult {
    name: String,
    wins: i32,
    draws: i32,
    losses: i32,
}

impl TeamResult {
    fn new(name: &str) -> TeamResult {
        TeamResult {
            name: name.to_string(),
            wins: 0,
            draws: 0,
            losses: 0,
        }
    }
    fn add_win(&mut self) {
        self.wins += 1;
    }
    fn add_draw(&mut self) {
        self.draws += 1;
    }
    fn add_loss(&mut self) {
        self.losses += 1;
    }
    fn played(&self) -> i32 {
        self.wins + self.draws + self.losses
    }
    fn points(&self) -> i32 {
        self.wins * 3 + self.draws
    }
    fn to_string(&self) -> String {
        format_row(
            &self.name,
            &self.played().to_string(),
            &self.wins.to_string(),
            &self.draws.to_string(),
            &self.losses.to_string(),
            &self.points().to_string(),
        )
    }
}

impl PartialOrd for TeamResult {
    fn partial_cmp(&self, other: &TeamResult) -> Option<cmp::Ordering> {
        Some(other.cmp(self))
    }
}

impl Ord for TeamResult {
    fn cmp(&self, other: &TeamResult) -> cmp::Ordering {
        if self.eq(other) {
            other.name.cmp(&self.name)
        } else {
            self.points().cmp(&other.points())
        }
    }
}

impl PartialEq for TeamResult {
    fn eq(&self, other: &TeamResult) -> bool {
        self.points() == other.points()
    }
}

struct LeagueResult {
    teams: HashMap<String, TeamResult>,
}

impl LeagueResult {
    fn new() -> LeagueResult {
        LeagueResult {
            teams: HashMap::new(),
        }
    }
    fn add_team(&mut self, team: &str) {
        if !self.teams.contains_key(team) {
            self.teams.insert(team.to_string(), TeamResult::new(team));
        }
    }

    fn record_win<'a>(&mut self, winner_name: &'a str, loser_name: &'a str) {
        self.add_team(winner_name);
        self.add_team(loser_name);

        self.teams.get_mut(winner_name).unwrap().add_win();
        self.teams.get_mut(loser_name).unwrap().add_loss();
    }

    fn record_draw<'a>(&mut self, team1_name: &'a str, team2_name: &'a str) {
        self.add_team(team1_name);
        self.add_team(team2_name);

        self.teams.get_mut(team1_name).unwrap().add_draw();
        self.teams.get_mut(team2_name).unwrap().add_draw();
    }
    fn sorted_teams(&self) -> Vec<&TeamResult> {
        let mut teams: Vec<&TeamResult> = self.teams.values().to_owned().collect();
        teams.sort();
        teams
    }
}

pub fn tally(match_results: &str) -> String {
    let mut league_result = LeagueResult::new();
    for result in match_results.split('\n') {
        match result.split(';').collect::<Vec<&str>>()[..] {
            [winner, loser, "win"] => league_result.record_win(winner, loser),
            [loser, winner, "loss"] => league_result.record_win(winner, loser),
            [team1, team2, "draw"] => league_result.record_draw(team1, team2),
            _ => println!("Not implemented"),
        }
    }

    let mut table = format_row("Team", "MP", "W", "D", "L", "P");
    for team_result in league_result.sorted_teams() {
        table.push('\n');
        table.push_str(team_result.to_string().as_str());
    }
    table
}
