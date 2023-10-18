use limit_stream_derive::RmpSer;

#[derive(RmpSer)]
pub struct User {
    pub name: String,
    pub age: limit_stream_runtime::builtin_type::Uint,
    pub description: String,
}

#[test]
fn test() {
    // let src: TokenStream = "".parse().unwrap();
    // derive_ser(src);
}
