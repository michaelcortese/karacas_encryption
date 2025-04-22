fn quick_rev(s: &mut String) {
    let mut sb =  s.as_bytes().to_vec();
    let l: usize  = s.len() / 2;
    for i in 0..l {
        let t = sb[i];
        sb[i] = sb[sb.len()-(i+1)];
        sb[s.len()-(i+1)] = t;
    }
    if let Ok(a) = String::from_utf8(sb) {
        *s = a;
    }
}

fn letter_swap(s: &mut String) {
    let mut sb = s.as_bytes().to_vec();
    for i in 0..sb.len() {
        match sb[i] {
            b'a' => sb[i] = b'0',
            b'e' => sb[i] = b'1',
            b'i' => sb[i] = b'2',
            b'o' => sb[i] = b'2',
            b'u' => sb[i] = b'3',
            _ => ()
        }
    }
    if let Ok(a) = String::from_utf8(sb) {
        *s = a;
    }
}

fn suffix(s: &mut String) {
    s.push_str("aca");
}

pub fn encrypt(s: &mut String) {
    quick_rev(s);
    letter_swap(s);
    suffix(s);
}