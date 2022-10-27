use std::collections::HashMap;

fn format_row(t: &str, m: &str, w: &str, d: &str, l: &str, p: &str) -> String {
    format!(
        "{:<30} |{:^4}| {:^3}| {:^3}| {:^3}| {:>2}",
        t, m, w, d, l, p
    )
}

#[derive(Clone, Copy)]
struct Results {
    wins: i32,
    draws: i32,
    losses: i32,
}

impl Results {
    fn new() -> Results {
        Results {
            wins: 0,
            draws: 0,
            losses: 0,
        }
    }
    fn played(&self) -> i32 {
        self.wins + self.draws + self.losses
    }
    fn points(&self) -> i32 {
        self.wins * 3 + self.draws
    }
}

pub fn tally(match_results: &str) -> String {
    let mut team_results: HashMap<&str, Results> = HashMap::new();
    for result in match_results.split('\n') {
        match result.split(';').collect::<Vec<&str>>()[..] {
            [winner, loser, "win"] => {
                if !team_results.contains_key(winner) {
                    team_results.insert(winner, Results::new());
                }
                if !team_results.contains_key(loser) {
                    team_results.insert(loser, Results::new());
                }

                let mut winner_results = team_results.get_mut(winner).unwrap();
                winner_results.wins += 1;

                let mut loser_results = team_results.get_mut(loser).unwrap();
                loser_results.losses += 1;
            }
            _ => println!("Not implemented"),
        }
    }

    let mut table = format_row("Team", "MP", "W", "D", "L", "P");
    for (team, r) in team_results {
        table.push('\n');
        table.push_str(&format_row(
            team,
            &r.played().to_string(),
            &r.wins.to_string(),
            &r.draws.to_string(),
            &r.losses.to_string(),
            &r.points().to_string(),
        ));
    }
    table
}
