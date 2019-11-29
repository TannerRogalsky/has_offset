use has_offset::Offsets;

#[repr(C, packed)]
#[derive(Offsets)]
struct Test {
    a: f32,
    b: u64,
}

#[test]
fn it_works() {
    let t = Test { a: 0.0, b: 0 };
    assert_eq!(Test::get_a_offset(), memoffset::offset_of!(Test, a));
    assert_eq!(Test::get_b_offset(), memoffset::offset_of!(Test, b));
}
