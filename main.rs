#![feature(box_syntax, box_patterns)]

struct RustRepo<'s> {
    id: u8,
    proj: &'s str,
}

impl<'s> RustRepo<'s> {
    fn new(id: u8, proj: &'s str) -> Self {
        Self { id, proj }
    }
}

fn main() {
    println!("Rust codebase masquerade.");
    let rr = RustRepo::new(101, "rust-debrief");
    println!("proj: {}, id: {}", rr.proj, rr.id);
}
