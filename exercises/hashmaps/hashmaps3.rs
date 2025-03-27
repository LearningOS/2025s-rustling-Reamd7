// hashmaps3.rs
//
// 给定一场足球比赛的比分列表（每行一个）。每行的格式为：
// "<队伍1名称>,<队伍2名称>,<队伍1进球数>,<队伍2进球数>"
// 例如：England,France,4,2（英格兰进4球，法国进2球）。
//
// 你需要构建一个比分表，包含队伍名称、队伍进球数和失球数。构建比分表的一种方法是使用
// Hashmap。解决方案部分使用了Hashmap，请完成它以通过测试。
//
// 让我通过测试！
//
// 执行 `rustlings hint hashmaps3` 或使用 `hint` watch 子命令获取提示。

// 我还没有完成

use std::collections::HashMap;

// 一个用于存储队伍进球详情的结构体。
struct Team {
    goals_scored: u8,
    goals_conceded: u8,
}

fn build_scores_table(results: String) -> HashMap<String, Team> {
    // 队伍名称是键，其对应的结构体是值。
    let mut scores: HashMap<String, Team> = HashMap::new();

    for r in results.lines() {
        let v: Vec<&str> = r.split(',').collect();
        let team_1_name = v[0].to_string();
        let team_1_score: u8 = v[2].parse().unwrap();
        let team_2_name = v[1].to_string();
        let team_2_score: u8 = v[3].parse().unwrap();
        // TODO: 使用从当前行提取的详细信息填充比分表。请记住，队伍1的进球数
        // 将是队伍2的失球数，同样，队伍2的进球数将是队伍1的失球数。
        let team_1 = Team {
            goals_scored: team_1_score,
            goals_conceded: team_2_score,
        };
        let team_2 = Team {
            goals_scored: team_2_score,
            goals_conceded: team_1_score,
        };
        if !scores.contains_key(&team_1_name) {
            scores.insert(team_1_name, team_1);
        } else {
            let team = scores.get_mut(&team_1_name).unwrap();
            team.goals_scored += team_1_score;
            team.goals_conceded += team_2_score;
        }
        if !scores.contains_key(&team_2_name) {
            scores.insert(team_2_name, team_2);
        } else {
            let team = scores.get_mut(&team_2_name).unwrap();
            team.goals_scored += team_2_score;
            team.goals_conceded += team_1_score;
        }
    }
    scores
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_results() -> String {
        let results = "".to_string()
            + "England,France,4,2\n"
            + "France,Italy,3,1\n"
            + "Poland,Spain,2,0\n"
            + "Germany,England,2,1\n";
        results
    }

    #[test]
    fn build_scores() {
        let scores = build_scores_table(get_results());

        let mut keys: Vec<&String> = scores.keys().collect();
        keys.sort();
        assert_eq!(
            keys,
            vec!["England", "France", "Germany", "Italy", "Poland", "Spain"]
        );
    }

    #[test]
    fn validate_team_score_1() {
        let scores = build_scores_table(get_results());
        let team = scores.get("England").unwrap();
        assert_eq!(team.goals_scored, 5);
        assert_eq!(team.goals_conceded, 4);
    }

    #[test]
    fn validate_team_score_2() {
        let scores = build_scores_table(get_results());
        let team = scores.get("Spain").unwrap();
        assert_eq!(team.goals_scored, 0);
        assert_eq!(team.goals_conceded, 2);
    }
}
