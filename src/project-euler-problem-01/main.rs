use std::int;


fn sum_multiples(accum: int, n: int) -> int {
    return match n {
        0               => accum,
        _ if n % 3 == 0 => sum_multiples(accum + n, n - 1),
        _ if n % 5 == 0 => sum_multiples(accum + n, n - 1),
        _               => sum_multiples(accum, n-1),
    };
}


fn main() {
    println(int::to_str(sum_multiples(0, 999)));
}
