pub struct Item {
    pub weight: u32,
    pub value: u32,
}

pub fn maximum_value(max_weight: u32, items: &[Item]) -> u32 {
    let mut knapsack_previous = vec![0u32; max_weight as usize + 1];
    let mut knapsack = vec![0u32; max_weight as usize + 1];

    for item in items {
        for knapsack_weight in 1usize..=(max_weight as usize) {
            if item.weight as usize <= knapsack_weight
                && knapsack_previous[knapsack_weight]
                    <= knapsack_previous[knapsack_weight - item.weight as usize] + item.value
            {
                knapsack[knapsack_weight] =
                    knapsack_previous[knapsack_weight - item.weight as usize] + item.value;
            }
        }
        knapsack_previous = knapsack.clone();
    }

    knapsack[max_weight as usize]
}
