struct Foo {
    x: &int
}

pub fn main() {
    let f = Foo { x: @3 };
    assert *f.x == 3;
}

