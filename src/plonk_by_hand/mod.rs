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

    // Calculate Embedding Degree
    // The degree of the extension field of characteristic p that contains every point of order r on the elliptic curve is the embedding degree.
    // The embedding degree is the smallest integer k such that the field extension F_p^k contains every point of order r.
    // In our case the embedding degree = 2
    let embedding_degree = 2;

    // To compute an elliptic curve pairing in order to check our Kate polynomial commitments, we need we need a pair of points on an elliptic curve
    // which have the same order anr return an element of field Fp^k.
    // The points (x, y) and (x', y') are called a pair of points on the elliptic curve if they satisfy the equation y^2 = x^3 + ax + b.
    // The points (x, y) and (x', y') are called a pair of points of order r on the elliptic curve if they satisfy the equation y^2 = x^3 + ax + b and r(x, y) = (x', y').

    // For our second subgroup since 17 is prime, if we find just one of them we can be sure that it is a generator of the second subgroup.
    // For the second subgroup we have (36,31u)
    // TODO: How to define "u" ?
    let point2_g = Point {
        x: 36,
        y: 31,
        field: 101,
    };

    // Generating SRS
    // First we will generate a random secret number "s" which is part of the toxic waste.
    // For our example we will use s = 2
    let s = 2;

    // For the first group we will have n+3 elements, where n is the number of gates in our case 4 as we will see further ahead
    // (2^0)*G1, (2^1)*G1, (2^2)*G1, (2^3)*G1, (2^4)*G1, (2^5)*G1, (2^6)*G1
    let s_g1_0 = modular_exp(2, 0, 101); // 1
    let s_g1_1 = modular_exp(2, 1, 101); // 2
    let s_g1_2 = modular_exp(2, 2, 101); // 4
    let s_g1_3 = modular_exp(2, 3, 101); // 8
    let s_g1_4 = modular_exp(2, 4, 101); // 16
    let s_g1_5 = modular_exp(2, 5, 101); // 15
    let s_g1_6 = modular_exp(2, 6, 101); // 13

    let srs_g1_0 = point_g;
    let srs_g1_1 = point_2g;
    let srs_g1_2 = point_4g;
    let srs_g1_3 = point_8g;
    let srs_g1_4 = point_16g;
    let srs_g1_5 = point_15g;
    let srs_g1_6 = point_13g;

    // For the second group the Generator = (36,31u)

    // TODO: fix on "u" based calculation
    let srs_g2_0 = point2_g.clone(); // => (36,31u)
    let srs_g2_1 = point_doubling(point2_g.clone()); // => (90,82u)

    // Following the example on Plonk by Hand, we will try to generate a proof a verifier for the Pythagorean Theorem equation (a^2 + b^2 = c^2)
    // For this we will generate the following polynomial commitments, given that for each gate on a plonk, there can be 1 addition(2 if you consider the constant) and 1 multiplication:
    // 1. x1^x1 = x2  ---- (a^2)
    // 2. x3^x3 = x4  ---- (b^2)
    // 3. x5^x5 = x6  ---- (c^2)
    // 4. x2 + x4 = x6

    // Plonk gate formula: (ql)*a + (qr)*b + (q0)*c + (qm)*a*b + qc = 0

    // Given the polynomial commitments and the Plonk gate formula we can tell that
    // x1^x1 = x2 => a = x1 and b = x1 and c = x2 => 0*a + 0*b + (-1)*c + 1*a*b + 0 = 0
    // x3^x3 = x4 => a = x3 and b = x3 and c = x4 => 0*a + 0*b + (-1)*c + 1*a*b + 0 = 0
    // x5^x5 = x6 => a = x5 and b = x5 and c = x6 => 0*a + 0*b + (-1)*c + 1*a*b + 0 = 0
    // x2 + x4 = x6 => a = x2 and b = x4 and c = x6 => 1*a + 1*b + (-1)*c + 0*a*b + 0 = 0

    // Given (3,4,5), these formulas, we can infer:
    // ql, qr, q0, qm and qc are called "selectors" and a,b and c are called "assignments"
    let ql = [0, 0, 0, 1];
    let qr = [0, 0, 0, 1];
    let q0 = [-1, -1, -1, -1];
    let qm = [1, 1, 0, 0];
    let qc = [0, 0, 0, 0];
    let a = [3, 4, 5, 9];
    let b = [3, 4, 5, 16];
    let c = [9, 16, 25, 25];

    // These constraints are not enough because x2 from x1^x1 = x2 and x4 from x3^x3 = x4 are not linked with x2 + x4 = x6 (last constraint)
    // For this we will need to add the "copy constraints"
    // Copy constraints are used to link the assignments of different gates together.

    // Copy constraints:
    // a1 = b1
    // a2 = b2
    // a3 = b3
    // c1 = a4
    // c2 = b4
    // c3 = c4

    // Interpolating our vectors (ql, qr, q0, qm, qc, a, b, c):
    // Given a vector, vector [5,10,15] , just by adding index, we can transform them into "points" => will use index (1,2,3) => (1,5) , (2,10) , (3,15)
    // With this we would have 3 points and with Lagrenge interpolation, we would be able to generate a (n-1) => (3 -1) = 2 degree polynomial that would pass through all the points
    // To transform vectors into "points" , we can use any domain, and for Plonk "roots of unity" are used
    // The nth roots of unity of a field are the field elements x that satisfy x^n=1
    // The vectors for our circuit and assignment are all of length four, so the domain of our polynomials must have at least four elements.

    // Finding 4 roots of unity in F17

    let root_0 = 1; // 1^4 = 1 (mod 17)
    let root_1 = 4; // 4^4 = 1 (mod 17)
    let root_2 = 16; // 16^4 = 1 (mod 17)
    let root_3 = 13; // 13^4 = 1 (mod 17)

    // No we need to label 12 of the values in our assigments( a, b ,c) with different fiel elements. //TODO: Why have to be different?
    // So we chose cosets by multiplying H => (root_0, root_1, root_2, root_3) with k1 and k2
    // Where k1 is chosen not an element of H and k2 is chosen that is neither an element from H and k1H

    let k1 = 2;
    let k2 = 3;

    let H = [root_0, root_1, root_2, root_3];
    let k1H = [k1 * root_0, k1 * root_1, k1 * root_2, k1 * root_3];
    let k2H = [k2 * root_0, k2 * root_1, k2 * root_2, k2 * root_3];

    // Now we will interpolate using the Roots of Unity
    // We will start with a = (3, 4, 5, 9)
    // The interpolated polynomial will be of degree 3 => 4 - 1 = 3
    // fa(x) = q0 + q1*x + q2*x^2 + q3*x^3
    // We want fa(1) = 3 , fa(4) = 4, fa(16) = 5, fa(13) = 9

    // fa(1) = 3 => q0 + q1*1 + q2*1^2 + q3*1^3 = 3
    // fa(4) = 4 => q0 + q1*4 + q2*4^2 + q3*4^3 = 4
    // fa(16) = 5 => q0 + q1*16 + q2*16^2 + q3*16^3 = 5
    // fa(13) = 9 => q0 + q1*13 + q2*13^2 + q3*13^3 = 9

    // This can be rewritten in a matrix:
    // [1 1 1 1] * [q0] = [3]
    // [4 4 4 4]   [q1] = [4]
    // [16 16 16 16] [q2] = [5]
    // [13 13 13 13] [q3] = [9]

    let a_vec = [3, 4, 5, 9];

    let a_interpolation_result = interpolate_lagrange(&H, &a_vec);
    println!("Interpolation Result: {:?}", a_interpolation_result);
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
pub fn interpolate_lagrange(x: &[i32], y: &[i32]) -> Vec<i32> {
    // Ensure that the input arrays have the same length
    assert_eq!(x.len(), y.len());

    let degree = x.len() - 1;
    let modulus = 17;

    // Initialize the coefficients of the interpolating polynomial
    let mut c = vec![0; degree + 1];

    // Loop over the degrees of the polynomial
    for k in 0..=degree {
        // Compute the k-th coefficient using the Lagrange formula
        let mut coeff = 0;
        for j in 0..=degree {
            if j != k {
                let numerator = (modulus - x[j]) % modulus;
                let denominator = (x[k] - x[j] + modulus) % modulus;
                coeff += numerator
                    * modular_inversion(denominator as i128, modulus as i128).unwrap() as i32
                    % modulus;
                coeff %= modulus;
            }
        }
        c[k] = (y[k] * coeff) % modulus;
    }

    c
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
