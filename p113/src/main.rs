

use num_bigint::{BigUint, ToBigUint};
use num_traits::{One, Zero};

// Функція для обчислення комбінацій C(n, k)
fn combinations(n: u32, k: u32) -> BigUint {
    if k > n {
        return Zero::zero();
    }
    if k == 0 || k == n {
        return One::one();
    }
    if k > n / 2 {
        return combinations(n, n - k);
    }

    let mut res: BigUint = One::one();
    for i in 0..k {
        res *= ToBigUint::to_biguint(&(n - i)).unwrap();
        res /= ToBigUint::to_biguint(&(i + 1)).unwrap();
    }
    res
}

// Функція для обчислення кількості нестрибучих чисел до 10^n
fn solve(n: u32) -> BigUint {
    let n_big = ToBigUint::to_biguint(&n).unwrap();

    // 1. Кількість зростаючих чисел (non-decreasing).
    // Формула C(n + 9, 9) враховує всі числа з неспадною послідовністю цифр.
    // Це включає однозначні числа.
    let increasing_count = combinations(n + 9, 9);
    
    // 2. Кількість спадних чисел (non-increasing).
    // Формула C(n + 10, 10) враховує всі числа з незростаючою послідовністю.
    // Ми віднімаємо 1, щоб виключити число 0.
    let decreasing_count = combinations(n + 10, 10) - 1.to_biguint().unwrap();

    // 3. Віднімаємо перетин: числа з однаковими цифрами.
    // Ми порахували їх двічі (як зростаючі, так і спадні).
    // Є 9 таких чисел для кожної довжини (1, 11, 111,... до 9, 99, 999...).
    let overlaps = n_big * 9.to_biguint().unwrap();
    
    // Остаточний підрахунок
    (increasing_count + decreasing_count - overlaps)
}

fn main() {
    let n = 100;
    println!("Проблема 113 Ейлера (для 10^{}):", n);
    println!("Кількість нестрибучих чисел: {}", solve(n));
}
