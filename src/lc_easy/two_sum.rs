use std::collections::HashMap;

// -----------------------------------------------------------------------------
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    // key = input vector element
    // value = associated input vector index
    let mut num_map: HashMap<i32, i32> = HashMap::new();

    // .iter().enumerate() yields the current count and the element during iteration
    for (i, &num) in nums.iter().enumerate() {
        // if there exists a KEY in our Hashmap such that it's the difference between
        //  target and current element, then assign the associated map VALUE to diff
        if let Some(&diff) = num_map.get(&(target - num)) {
            // return a vector of the two complimentary indices
            return vec![diff, i as i32];
        } else {
            // otherwise, add the current element and its index to our HashMap
            num_map.insert(num, i as i32);
        }
    }

    // we don't expect a panic based on input criteria of problem
    panic!("two_sum() failed to identify solution pair")
}
