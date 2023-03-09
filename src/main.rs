fn main() {
    let n_field = 101;
    let point_g1 = Point {x:1 , y:2 , field: n_field};
    let point_g2 = point_doubling(point_g1.clone());    
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

pub fn modular_division(a: i128, b: i128, n: i128) -> i128 {
    0
}

pub fn modular_exp(a:i128, exp: i128, n: i128) -> i128 {
    let mut result = 0;
    for i in 0..exp {
        result = modular_multiplication(a , a, n);
    };
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
    todo!
}

#[test]
fn crypto_hack_modular_arithmetic_gcd_exercise_1(){
    assert_eq!(greater_common_divisor(66528, 52920), 1512);
}

#[test]
fn crypto_hack_modular_aritchmetic_gcd2_exercise_1(){
    assert_eq!(greater_common_divisor_2(66528, 52920), 1512);
}

#[test]
fn crypto_hack_modular_arithmetic_module_exercise_3(){
    assert_eq!(module(8146798528947, 17), 4);
}


