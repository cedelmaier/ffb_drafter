extern crate ffb_drafter;

use ffb_drafter::*;

fn main() {
    let nfl_teams = vec!["BAL","BUF","CIN","CLE","DEN","HOU","IND","JAC",
                         "KC","MIA","NE","NYJ","OAK","PIT","SD","TEN",
                         "ARI","ATL","CAR","CHI","DAL","DET","GB","MIN",
                         "NO","NYG","PHI","STL","SF","SEA","TB","WAS"];

    let nfl_teams: Vec<String> = nfl_teams.iter().map(|w| w.to_lowercase()).collect();

    ffb_io::read_raw_players(r"data/qb.dat", ffb_io::Position::QB);
}
