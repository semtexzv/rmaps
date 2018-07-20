use ::prelude;

pub fn start(name: &str) {}

pub fn end(name: &str) {}

pub fn frame<A, F: FnOnce() -> A>(name: &str, fun: F) -> A {
    start(name);
    let res = fun();
    end(name);
    res
}