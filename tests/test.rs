use limit_stream_derive::{RmpDeSer, RmpSer};
use limit_stream_runtime::builtin_type::Uint;
use limit_stream_runtime::{Deser, Ser};

#[derive(RmpSer, RmpDeSer)]
pub struct User {
    pub name: String,
    pub age: Uint,
    pub description: String,
}

#[derive(RmpSer, RmpDeSer)]
pub enum UserForm {
    User(User),
    Id(Uint),
}

#[test]
fn test() {
    UserForm::Id(114);
    // let src: TokenStream = "".parse().unwrap();
    // derive_ser(src);
}
