struct Expression<'a> {
    terms: &'a Vec<Term>
}

struct Term {
    previous: u8,
    base: u8,
    exponent: u8,
    next: u8
}