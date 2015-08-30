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

pub fn compose<A, F, G>(a: A, mut f: F, mut g: G) -> Fix<A>
    where F: FnMut(A) -> Fix<A>, G: FnMut(A) -> Fix<A> {
    match f(a) {
        Fix::Fix(b) => g(b),
        fix => fix
    }
}

pub fn fix<A, F: FnMut(A) -> Fix<A>>(mut a: A, mut f: F) -> A {
    loop {
        match f(a) {
            Fix::Fix(b) => return b,
            Fix::Pro(b) => a = b
        }
    }
}

pub fn fix_result<A, E, F: FnMut(A) -> Result<Fix<A>, E>>(mut a: A, mut f: F) -> Result<A, E> {
    loop {
        match f(a) {
            Result::Err(err) => return Result::Err(err),
            Result::Ok(Fix::Fix(b)) => return Result::Ok(b),
            Result::Ok(Fix::Pro(b)) => a = b
        }
    }
}
