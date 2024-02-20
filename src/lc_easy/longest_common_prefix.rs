// https://doc.rust-lang.org/std/string/struct.String.html#deref-methods-str
// -----------------------------------------------------------------------------
pub fn longest_common_prefix(strs: Vec<String>) -> String {
    // pick any of the strings in strs to use for comparison (since prefix is common to all)
    let mut result = strs[0].clone();

    for s in strs.iter() {
        // if the current string doesn't start with result, pop the last character off of result
        //  eventually left with longest common prefix between the two being compared
        while !s.starts_with(&result) {
            result.pop();
        }
    }

    result.to_string()
}
