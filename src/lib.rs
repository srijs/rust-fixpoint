#[derive (Debug, PartialEq, Eq)]
pub enum Fix<A> {
    Pro(A),
    Fix(A)
}

impl<A> Fix<A> {
    pub fn map<B, F: FnOnce(A) -> B>(self, f: F) -> Fix<B> {
        match self {
            Fix::Pro(a) => Fix::Pro(f(a)),
            Fix::Fix(a) => Fix::Fix(f(a))
        }
    }
}

pub fn fix<A, F: FnMut(A) -> Fix<A>>(a: A, f: F) -> A {
    let mut current = a;
    let mut g = f;
    loop {
        match g(current) {
            Fix::Fix(a) => return a,
            Fix::Pro(a) => current = a
        }
    }
}

pub fn fix_result<A, E, F: FnMut(A) -> Result<Fix<A>, E>>(a: A, f: F) -> Result<A, E> {
    let mut current = a;
    let mut g = f;
    loop {
        match g(current) {
            Result::Err(err) => return Result::Err(err),
            Result::Ok(Fix::Fix(a)) => return Result::Ok(a),
            Result::Ok(Fix::Pro(a)) => current = a
        }
    }
}
