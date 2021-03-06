use crate::model::stats::Stats;
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Ratings {
    pub overall: i32,
    pub totalTurns: i32,
    pub gameTurns: i32,
    pub mvps: i32,
    pub streak: i32,
    pub awards: i32,
}

impl Ratings {
    pub fn load(stat: &Stats) -> Ratings {
        let overall: i32;
        let totalTurns = Self::fromarr(stat.totalTurns, [0, 10, 25, 50, 100]);
        let gameTurns = Self::fromarr(stat.gameTurns, [0, 5, 10, 25, 40]);
        let mvps = Self::fromarr(stat.mvps, [0, 1, 5, 10, 25]);
        let awards = Self::fromarr(stat.awards, [0, 1, 2, 3, 4]);
        let streak = Self::fromarr(stat.streak, [0, 3, 5, 10, 25]);
        let mut numbers = vec![totalTurns, gameTurns, mvps, awards, streak];
        numbers.sort();
        let mid = numbers.len() / 2;
        overall = numbers[mid];
        Ratings {
            overall,
            totalTurns,
            gameTurns,
            mvps,
            streak,
            awards,
        }
    }

    fn fromarr(num: i32, arr: [i32; 5]) -> i32 {
        let mut r = 0;
        for x in &arr {
            if x <= &num {
                r += 1;
            } else {
                r += 0;
                break;
            }
        }
        r
    }
}
