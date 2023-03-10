pub fn run_part_1() {
    let n_field = 101;
    let point_g = Point {
        x: 1,
        y: 2,
        field: n_field,
    };
    let point_2g = point_doubling(point_g.clone());
    let inverted_point_2g = point_inversion(point_2g.clone());
    println!("2G: {:?}", point_2g);
    println!("Inverted 2G: {:?}", inverted_point_2g);
    let point_3g = point_addition(point_g.clone(), point_2g.clone());
    let inverted_point_3g = point_inversion(point_3g.clone());
    println!("3G: {:?}", point_3g);
    println!("Inverted 3G: {:?}", inverted_point_3g);
    let point_4g = point_doubling(point_2g.clone());
    let inverted_point_4g = point_inversion(point_4g.clone());
    println!("4G: {:?}", point_4g);
    println!("Inverted 4G: {:?}", inverted_point_4g);
    let point_5g = point_addition(point_g.clone(), point_4g.clone());
    let inverted_point_5g = point_inversion(point_5g.clone());
    println!("5G: {:?}", point_5g);
    println!("Inverted 5G: {:?}", inverted_point_5g);
    let point_6g = point_addition(point_g.clone(), point_5g.clone());
    let inverted_point_6g = point_inversion(point_6g.clone());
    println!("6G: {:?}", point_6g);
    println!("Inverted 6G: {:?}", inverted_point_6g);
    let point_7g = point_addition(point_g.clone(), point_6g.clone());
    let inverted_point_7g = point_inversion(point_7g.clone());
    println!("7G: {:?}", point_7g);
    println!("Inverted 7G: {:?}", inverted_point_7g);
    let point_8g = point_doubling(point_4g.clone());
    let inverted_point_8g = point_inversion(point_8g.clone());
    println!("8G: {:?}", point_8g);
    println!("Inverted 8G: {:?}", inverted_point_8g);
    let point_9g = point_addition(point_3g.clone(), point_6g.clone());
    let inverted_point_9g = point_inversion(point_9g.clone());
    println!("9G: {:?}", point_9g);
    println!("Inverted 9G: {:?}", inverted_point_9g);
    let point_10g = point_doubling(point_5g.clone());
    let inverted_point_10g = point_inversion(point_10g.clone());
    println!("10G: {:?}", point_10g);
    println!("Inverted 10G: {:?}", inverted_point_10g);
    let point_11g = point_addition(point_4g.clone(), point_7g.clone());
    let inverted_point_11g = point_inversion(point_11g.clone());
    println!("11G: {:?}", point_11g);
    println!("Inverted 11G: {:?}", inverted_point_11g);
    let point_12g = point_addition(point_g.clone(), point_10g.clone());
    let inverted_point_12g = point_inversion(point_12g.clone());
    println!("12G: {:?}", point_12g);
    println!("Inverted 12G: {:?}", inverted_point_12g);
    let point_13g = point_addition(point_g.clone(), point_11g.clone());
    let inverted_point_13g = point_inversion(point_13g.clone());
    println!("13G: {:?}", point_13g);
    println!("Inverted 13G: {:?}", inverted_point_13g);
    let point_14g = point_addition(point_g.clone(), point_13g.clone());
    let inverted_point_14g = point_inversion(point_14g.clone());
    println!("14G: {:?}", point_14g);
    println!("Inverted 14G: {:?}", inverted_point_14g);
    let point_15g = point_addition(point_g.clone(), point_14g.clone());
    let inverted_point_15g = point_inversion(point_15g.clone());
    println!("15G: {:?}", point_15g);
    println!("Inverted 15G: {:?}", inverted_point_15g);
    let point_16g = point_doubling(point_8g.clone());
    let inverted_point_16g = point_inversion(point_16g.clone());
    println!("16G: {:?}", point_16g);
    println!("Inverted 16G: {:?}", inverted_point_16g);
    // Because inverted_point_32G = point_g , this subgroup has order 17
    let point_32g = point_doubling(point_16g.clone());
    let inverted_point_32g = point_inversion(point_32g.clone());
    println!("32G: {:?}", point_32g);
    println!("Inverted 32G: {:?}", inverted_point_32g);
}
#[derive(Debug, Clone)]
struct Point {
    x: i128,
    y: i128,
    field: i128,
}

// Elliptic curve point doubling is an operation that takes a point on an elliptic curve and returns a new point on the curve that is twice the original point.
// This operation is a building block of many elliptic curve algorithms, including scalar multiplication, which is used for public key cryptography.
// The resulting point (x', y') is twice the original point (x, y) on the elliptic curve.
fn point_doubling(point: Point) -> Point {
    let x = point.x;
    let y = point.y;
    // Define M
    // 3*x^2
    let x_2 = modular_exp(x, 2, point.field);
    let x_3 = modular_multiplication(x_2, 3, point.field);
    // 2*y
    let div_1 = modular_multiplication(2, y, point.field);
    // Invert 2*y
    let div_2 = modular_inversion(div_1, point.field).unwrap();
    // m = 3*x^2 / 2*y
    let m = modular_multiplication(x_3, div_2, point.field);
    println!("M: {}", m);
    // // Calculate x2 based on M => x2 = m^2 - 2*x;
    let x2_1 = modular_exp(m, 2, point.field);
    let x2_2 = modular_multiplication(2, x, point.field);
    let x2_3 = modular_subtraction(x2_1, x2_2, point.field);
    // Calculate y2 based on M => m*(3*x -m^2) -y
    let y2_1 = modular_exp(m, 2, point.field);
    let y2_2 = modular_multiplication(3, x, point.field);
    let y2_3 = modular_subtraction(y2_2, y2_1, point.field);
    let y2_4 = modular_multiplication(m, y2_3, point.field);
    let y2_5 = modular_subtraction(y2_4, y, point.field);
    return Point {
        x: x2_3,
        y: y2_5,
        field: point.field,
    };
}

fn point_inversion(point: Point) -> Point {
    return Point {
        x: point.x,
        y: module(-point.y, point.field),
        field: point.field,
    };
}

// With 2 distinc points, P and !, addition is defined as the negation of the point resulting from the intersection of the curve
// and the straight line defined by the points P and Q, giving the point, R
fn point_addition(point1: Point, point2: Point) -> Point {
    // considering (x1, y1) + (x2, y2) = (x3, y3)
    // s = (y2 - y1) / (x2 - x1)
    // x3 = s^2 - x1 - x2
    // y3 = s(x1 - x3) - y1

    // s_1 = (y2 - y1)
    let s_1 = modular_subtraction(point2.y, point1.y, point1.field);
    // s_2 = (x2 - x1)
    let s_2 = modular_subtraction(point2.x, point1.x, point1.field);
    // s_3 = (x2 - x1) ^ (-1)
    let s_3 = modular_inversion(s_2, point1.field).unwrap();
    // s = (y2 - y1) / (x2 - x1)
    let s = modular_multiplication(s_1, s_3, point1.field);
    println!("S: {}", s);

    // x3_1 = s^2
    let x3_1 = modular_exp(s, 2, point1.field);
    // x3_2 = x3_1 - x1;
    let x3_2 = modular_subtraction(x3_1, point1.x, point1.field);
    // x = x3_2 - x2;
    let x3 = modular_subtraction(x3_2, point2.x, point1.field);

    //y3_1 = x1 - x3
    let y3_1 = modular_subtraction(point1.x, x3, point1.field);
    //y3_2 = s * y3_1
    let y3_2 = modular_multiplication(s, y3_1, point1.field);
    //y3= y3_2 - y1
    let y3 = modular_subtraction(y3_2, point1.y, point1.field);

    Point {
        x: x3,
        y: y3,
        field: point1.field,
    }
}

pub fn modular_addition(a: i128, b: i128, n: i128) -> i128 {
    let addition_value = a + b;
    let result = module(addition_value, n);
    result
}

pub fn modular_subtraction(a: i128, b: i128, n: i128) -> i128 {
    let substraction_value = a - b;
    let result = module(substraction_value, n);
    result
}

pub fn modular_multiplication(a: i128, b: i128, n: i128) -> i128 {
    let multiplication_value = a * b;
    let result = module(multiplication_value, n);
    result
}

/// Fermat's little theorem can help us find the modular inverse of an integer a modulo a prime number p.

/// The modular inverse of a modulo p is the integer b such that a * b ≡ 1 (mod p). We can use Fermat's little theorem to find the modular inverse as follows:

/// Compute a^(p-2) mod p using modular_exponentiation(a, p-2, p).
/// The result of a^(p-2) mod p is the modular inverse b of a modulo p.
/// This method works only if p is a prime number and a is not divisible by p. If p is not a prime or a is divisible by p, then this method may not work.
pub fn modular_inversion(a: i128, modulus: i128) -> Option<i128> {
    if modulus <= 1 {
        return None;
    }

    // Calculate a^(p-2) mod p using modular exponentiation
    let b = modular_exp(a, modulus - 2, modulus);

    // Check if a and p are coprime
    if greater_common_divisor(a, modulus) == 1 {
        // If a and p are coprime, return the modular inverse
        Some(b)
    } else {
        // If a and p are not coprime, the modular inverse does not exist
        None
    }
}

/// The function modular_exp(base: i128, exponent: i128, modulus: i128) provides an efficient way to compute the modular exponentiation base^exponent mod modulus.

/// Fermat's little theorem states that for any prime number p and any integer a not divisible by p, a^(p-1) ≡ 1 (mod p).

/// The function modular_exponentiation can be used to prove Fermat's little theorem as follows:

/// Let a be any integer not divisible by p.
/// Compute a^(p-1) mod p using modular_exponentiation(a, p-1, p).
/// According to the function modular_exponentiation, the result of a^(p-1) mod p is equal to the remainder when a^(p-1) is divided by p. Therefore, if the remainder is 1, then a^(p-1) ≡ 1 (mod p), which proves Fermat's little theorem.
/// For example, let's take a=2 and p=5, which is a prime number. According to Fermat's little theorem, 2^4 ≡ 1 (mod 5). We can use the function modular_exponentiation to verify this as follows:
fn modular_exp(base: i128, exponent: i128, modulus: i128) -> i128 {
    if modulus == 1 {
        return 0;
    }
    let mut result = 1;
    let mut base = base % modulus;
    let mut exp = exponent;
    while exp > 0 {
        if exp % 2 == 1 {
            result = (result * base) % modulus;
        }
        exp >>= 1;
        base = (base * base) % modulus;
    }
    result
}

pub fn module(a: i128, n: i128) -> i128 {
    let mut result = a % n;
    if result < 0 {
        result = result + n;
    }
    result
}

pub fn greater_common_divisor(mut a: i128, mut b: i128) -> i128 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

pub fn greater_common_divisor_2(mut a: i128, mut b: i128) -> i128 {
    while a != b {
        if a > b {
            a = a - b;
        } else {
            b = b - a;
        }
    }
    a
}

pub fn extended_euclidean_algorithm(a: i128, b: i128) -> (i128, i128, i128) {
    // b0 = a0 * q + r0
    // b1 = a1 * q + r1 => b1 = a0 , a1 = r0
    // So b1 = a1*q + r1 => a0 = r0*q + r1
    // So nth => bn = an*q + rn => an-1 = rn-1*q + rn
    println!("a: {}, b: {}", a, b);
    if a == 0 {
        return (b, 0, 1);
    }
    // b % a = remainder => rn
    // bn = an-1
    // (b % a , a) => (rn , an-1) => (an , bn)
    let (gcd, x1, y1) = extended_euclidean_algorithm(b % a, a);

    //TODO: explain details
    let x = y1 - (b / a) * x1;
    let y = x1;

    (gcd, x, y)
}

#[test]
fn crypto_hack_modular_arithmetic_gcd_exercise_1() {
    assert_eq!(greater_common_divisor(66528, 52920), 1512);
}

#[test]
fn crypto_hack_extended_euclidean_algorithm_exercise_2() {
    let (gcd, x, y) = extended_euclidean_algorithm(26513, 32321);
    assert_eq!(x, 10245);
    assert_eq!(y, -8404);
}

#[test]
fn crypto_hack_modular_aritchmetic_gcd2_exercise_1() {
    assert_eq!(greater_common_divisor_2(66528, 52920), 1512);
}

#[test]
fn crypto_hack_modular_arithmetic_module_exercise_3() {
    assert_eq!(module(8146798528947, 17), 4);
}

#[test]
fn crypto_hack_modular_exp_exercise_4() {
    assert_eq!(modular_exp(273246787654, 65536, 65537), 1);
}

#[test]
fn crypto_hack_modular_invert_exercise_5() {
    assert_eq!(modular_inversion(3, 13), Some(9));
}
#[test]
fn plonk_by_hand_point_doubling_step_1() {
    let n_field = 101;
    let point_g1 = Point {
        x: 1,
        y: 2,
        field: n_field,
    };
    let point_g2 = point_doubling(point_g1.clone());
    assert_eq!(point_g2.x, 68);
    assert_eq!(point_g2.y, 74);
}

#[test]
fn plonk_by_hand_point_inversion_step_1() {
    let n_field = 101;
    let point_g2 = Point {
        x: 68,
        y: 74,
        field: n_field,
    };
    let inverted_point_g2 = point_inversion(point_g2);
    assert_eq!(inverted_point_g2.x, 68);
    assert_eq!(inverted_point_g2.y, 27);
}
