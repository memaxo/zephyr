use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KnapsackProblem {
    pub capacity: u64,
    pub weights: Vec<u64>,
    pub values: Vec<u64>,
}

impl KnapsackProblem {
    pub fn new(capacity: u64, weights: Vec<u64>, values: Vec<u64>) -> Self {
        KnapsackProblem {
            capacity,
            weights,
            values,
        }
    }

    pub fn solve(&self) -> KnapsackSolution {
        let num_items = self.weights.len();
        let mut dp = vec![vec![0; (self.capacity + 1) as usize]; num_items + 1];

        for i in 1..=num_items {
            for w in 1..=self.capacity {
                if self.weights[i - 1] <= w {
                    dp[i][w as usize] =
                        dp[i][w as usize].max(dp[i - 1][w as usize].max(
                            dp[i - 1][(w - self.weights[i - 1]) as usize] + self.values[i - 1],
                        ));
                } else {
                    dp[i][w as usize] = dp[i - 1][w as usize];
                }
            }
        }

        let mut selected_items = Vec::new();
        let mut i = num_items;
        let mut w = self.capacity;
        while i > 0 && w > 0 {
            if dp[i][w as usize] != dp[i - 1][w as usize] {
                selected_items.push(i - 1);
                w -= self.weights[i - 1];
            }
            i -= 1;
        }

        KnapsackSolution {
            selected_items,
            total_value: dp[num_items][self.capacity as usize],
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KnapsackSolution {
    pub selected_items: Vec<usize>,
    pub total_value: u64,
}

impl KnapsackSolution {
    pub fn new(selected_items: Vec<usize>, total_value: u64) -> Self {
        KnapsackSolution {
            selected_items,
            total_value,
        }
    }
}
