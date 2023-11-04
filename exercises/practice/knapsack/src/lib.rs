pub struct Item {
    pub weight: u32,
    pub value: u32,
}

/// Pseudo polynomial solution exploiting dynamic programming.
///
/// [see on Wikipedia](https://en.wikipedia.org/wiki/Knapsack_problem#0-1_knapsack_problem)
pub fn maximum_value(max_weight: u32, items: &[Item]) -> u32 {
    // Data structures that store the partial computations results.
    // In particular, at location `i` will be stored the maximum value that can be obtained with a
    // maximum weight of `i` and taking only the items that comes before `item` (`item` included in
    // `knapsack`, excluded in `knapsack_previous`).
    let buffer_size = max_weight as usize + 1;
    let mut knapsack_previous = vec![0; buffer_size];
    let mut knapsack = vec![0; buffer_size];

    for item in items {
        let item_weight = item.weight as usize;
        for knapsack_weight in item_weight..buffer_size {
            if knapsack_previous[knapsack_weight]
                <= knapsack_previous[knapsack_weight - item_weight] + item.value
            {
                knapsack[knapsack_weight] =
                    knapsack_previous[knapsack_weight - item_weight] + item.value;
            }
        }
        knapsack_previous.copy_from_slice(&knapsack);
    }

    knapsack[max_weight as usize]
}
