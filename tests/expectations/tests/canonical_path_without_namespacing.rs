/* automatically generated by rust-bindgen */


#![allow(non_snake_case)]


#[repr(C)]
#[derive(Debug, Default, Copy)]
pub struct Bar {
    pub _address: u8,
}
#[test]
fn bindgen_test_layout_Bar() {
    assert_eq!(::std::mem::size_of::<Bar>() , 1usize , concat ! (
               "Size of: " , stringify ! ( Bar ) ));
    assert_eq! (::std::mem::align_of::<Bar>() , 1usize , concat ! (
                "Alignment of " , stringify ! ( Bar ) ));
}
impl Clone for Bar {
    fn clone(&self) -> Self { *self }
}
extern "C" {
    #[link_name = "_Z3bazPN3foo3BarE"]
    pub fn baz(arg1: *mut Bar);
}
