// -----------------------------------------------------------------------------
pub fn is_palindrome(x: i32) -> bool {
    // palindromes must be positive integers (no negative sign)
    if x < 0 {
        return false;
    }

    // build the 'reverse' integer to check against input x
    let mut reverse = 0;

    // need to mutate x, hence assigning it to 'temp'
    let mut temp = x;
    while temp > 0 {
        // (= reverse * 10) will add a 0 digit to right end of reverse
        // (+ temp % 10) will change that 0 digit to the last digit of temp
        reverse = reverse * 10 + temp % 10;
        // remove the right most digit from temp
        temp /= 10;
    }

    x == reverse
}

/*
// this solution checks if first HALF of input is equivalent to second HALF
//  thus, only iterates over half of the input integer
// key difference is not using a 'temp' variable, instead mutating the input x
// -----------------------------------------------------------------------------
bool is_palindrome (x: i32) -> bool {
    // if x is negative or if x is a (non-zero) multiple of 10
    if (x < 0 || (x != 0 && x % 10 == 0)) { return false; }

    let mut reverse = 0;

    // while removing last digit of x with each iteration, the condition (x > reverse), the
    //  loop breaks when:
    //  - if x has ODD num digits: reverse will have the 'middle' digit and thus be greater
    //      - need to remove this 'middle' digit below when determining equality
    //  - if x has EVEN num digits:
    //      - if x is palindrome: loop breaks when x == reverse
    //      - if x is NOT palindrome: reverse will have one more digit than x (failing equality)
    let mut x = x;      // input parameter x is immutable
    while (x > reverse) {
        reverse = reverse * 10 + x % 10;
        x /= 10;
    }

    // if palindrome is even number of digits, then x == reverse
    // if palindrome is odd number of digits, then x == reverse / 10. (remove the 'middle' digit)
    x == reverse || x == reverse / 10
}
 */
