use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput, Ident};

#[proc_macro_derive(RmpSer)]
pub fn derive_ser(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    // println!("name: {}", input.ident);
    match input.data {
        syn::Data::Struct(ds) => {
            let r: Vec<Ident> = ds.fields.iter().map(|f| f.ident.clone().unwrap()).collect();

            todo!()
        }
        syn::Data::Enum(de) => {
            // de.variants.into_iter().map(|x| x.)
            todo!()
        }
        _ => panic!("not support union"),
    }
}
