use limit_stream_derive::{RmpSer, RmpDeSer};
use limit_stream_runtime::builtin_type::Uint;

#[derive(RmpSer, RmpDeSer)]
pub struct User {
    pub name: String,
    pub age: Uint,
    pub description: String,
}

#[test]
fn test() {
    // let src: TokenStream = "".parse().unwrap();
    // derive_ser(src);
}
