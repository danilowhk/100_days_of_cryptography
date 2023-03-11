use super::field_element::FieldElement;

pub fn xgcd(x: i128, y: i128) -> (FieldElement, FieldElement, FieldElement) {
    let (mut r, mut new_r) = (x, y);
    let (mut s, mut new_s) = (1, 0);
    let (mut t, mut new_t) = (0, 1);

    while new_r != 0 {
        let quotient = r / new_r;
        (t, new_t) = (new_t, t - (quotient * new_t));
        (r, new_r) = (new_r, r - quotient * new_r);
        (s, new_s) = (new_s, s - quotient * new_s);
    }

    assert!(r == 1);

    return (t.into(), r.into(), s.into());
}
