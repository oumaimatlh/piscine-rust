// pub fn edit_distance(source: &str, target: &str) -> usize {
//     let s: Vec<char> = source.chars().collect();
//     let t: Vec<char> = target.chars().collect();
//     let mut count: usize = 0;
//     if source.len() > target.len() {
//         count += source.len() - target.len();
//     }else if source.len() < target.len() {
//         count += target.len() - source.len();

//     let mut dp = vec![vec![0; t.len() + 1]; s.len() + 1];

//     for i in 0..=s.len() {
//         dp[i][0] = i;
//     }

//     for j in 0..=t.len() {
//         dp[0][j] = j;
//     }

//     for i in 1..=s.len() {
//         for j in 1..=t.len() {
//             if s[i - 1] == t[j - 1] {
//                 dp[i][j] = dp[i - 1][j - 1];
//             } else {
//                 dp[i][j] = 1 + dp[i - 1][j - 1]
//                     .min(dp[i - 1][j])
//                     .min(dp[i][j - 1]);
//             }
//         }
//     }
//     }
//     dp[s.len()][t.len()]
// }