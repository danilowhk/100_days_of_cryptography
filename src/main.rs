fn main() {
    let n_field = 101;
    let point_g1 = Point {x:1 , y:2 , field: n_field};
    let point_g2 = point_doubling(point_g1.clone());    
    let (a , b , c) = extended_euclidean_algorithm(81, 57);
    let invert = modular_inversion(3, 13);
    println!("Invert: {:?}", invert);
    // println!("a: {}, b: {}, c: {}", a, b, c);

    // let test1 = modular_exp(273246787654, 65536, 17);
    // let test2 = modular_exp(5, 17, 17);
    // let test3 = modular_exp(7, 16, 17);

    // println!("test1: {}", test1);
    // println!("test2: {}", test2);
    // println!("test3: {}", test3);

}
#[derive(Debug,Clone)]
struct Point {
    x: i128,
    y: i128,
    field: i128,
}

fn point_doubling(point : Point) -> Point {
    let x = point.x;
    let y = point.y;
    // Define M
    // let m = 3*x^2 / 2*y;
    let x_2 = modular_exp(x , 2 , point.field);
    let x_3 = modular_multiplication(x_2, 3, point.field);

    let div_1 = modular_multiplication(2 , y , point.field);
    println!("x3: {}", x_3);
    println!("div_1: {}", div_1);
    // // Calculate x2 based on M
    // let x2 = m^2 - 2*x;
    // println!("X2: {}", x2);
    // // Calculate y2 based on M
    // let y2 = m*(3*x -m^2) -y;
    // println!("Y2: {}", y2);
    return Point {x: 10, y: 10 , field: point.field}
}

// fn point_inversion(point: Point) -> Point {
//     return Point {x: point.x, y: -point.y}
// }

pub fn modular_addition(a: i128, b: i128, n: i128) -> i128 {
    let addition_value = a + b;
    let result = module(addition_value, n);
    result
}

pub fn modular_subtraction(a: i128, b: i128, n: i128) -> i128 {
    let substraction_value = a-b;
    let result = module(substraction_value, n);
    result
}

pub fn modular_multiplication(a: i128, b: i128, n:i128) -> i128 {
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
    if greater_common_divisor(a, modulus) == 1{
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



pub fn module(a: i128 , n: i128) -> i128 {
    let mut result = a % n;
    if result < 0 {
        result = result + n;
    }
    result
}

pub fn greater_common_divisor(mut a: i128 , mut b: i128) ->  i128{
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

pub fn greater_common_divisor_2( mut a: i128, mut b: i128) -> i128 {
    while a != b {
        if a > b {
            a = a -b;
        } else {
            b = b -a;
        }
    }
    a
}

pub fn extended_euclidean_algorithm(a: i128, b: i128) -> (i128, i128, i128){

    // b0 = a0 * q + r0
    // b1 = a1 * q + r1 => b1 = a0 , a1 = r0
    // So b1 = a1*q + r1 => a0 = r0*q + r1
    // So nth => bn = an*q + rn => an-1 = rn-1*q + rn
    println!("a: {}, b: {}", a, b);
    if a == 0 {
        return (b , 0 , 1)
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
fn crypto_hack_modular_arithmetic_gcd_exercise_1(){
    assert_eq!(greater_common_divisor(66528, 52920), 1512);
}

#[test]
fn crypto_hack_extended_euclidean_algorithm_exercise_2(){
    let (gcd, x , y) = extended_euclidean_algorithm(26513, 32321);
    assert_eq!(x, 10245);
    assert_eq!(y, -8404);
}

#[test]
fn crypto_hack_modular_aritchmetic_gcd2_exercise_1(){
    assert_eq!(greater_common_divisor_2(66528, 52920), 1512);
}

#[test]
fn crypto_hack_modular_arithmetic_module_exercise_3(){
    assert_eq!(module(8146798528947, 17), 4);
}

#[test]
fn crypto_hack_modular_exp_exercise_4(){
    assert_eq!(modular_exp(273246787654, 65536, 65537), 1);
}

#[test]
fn crypto_hack_modular_invert_exercise_5(){
    assert_eq!(modular_inversion(3,13), Some(9));
}

