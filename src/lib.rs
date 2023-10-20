use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(RmpSer)]
pub fn derive_ser(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let struct_name = input.ident.clone();
    match input.data {
        syn::Data::Struct(ds) => {
            let body = ds
                .fields
                .iter()
                .map(|f| {
                    let ident = f.ident.clone().unwrap();
                    quote!(self.#ident.ser(buf)?;)
                })
                .reduce(|mut l, r| {
                    l.extend(vec![r]);
                    l
                })
                .unwrap();
            // println!("{}", body.to_string());
            let r = quote! {
              impl limit_stream_runtime::Ser for #struct_name {
                fn ser(&self, buf: &mut limit_stream_runtime::utils::ByteBuf) -> Result<(), ()> {
                    limit_stream_runtime::utils::ls_write_array_len(buf, 3)?;
                    #body
                    Ok(())
                }
              }
            };
            // println!("{}", r.to_string());
            return r.into();
        }
        syn::Data::Enum(_de) => {
            // de.variants.into_iter().map(|x| x.)
            todo!()
        }
        _ => panic!("not support union"),
    }
}


#[proc_macro_derive(RmpDeSer)]
pub fn derive_deser(item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let struct_name = input.ident.clone();
    match input.data {
        syn::Data::Struct(ds) => {
            let body = ds
                .fields
                .iter()
                .map(|f| {
                    let ident = f.ident.clone().unwrap();
                    quote!(value.#ident = limit_stream_runtime::utils::ls_read_value(buf)?;)
                })
                .reduce(|mut l, r| {
                    l.extend(vec![r]);
                    l
                })
                .unwrap();
            // println!("{}", body.to_string());
            let r = quote! {
                impl limit_stream_runtime::Deser for #struct_name {
                    type Res = Result<Self, ()>;
                    fn deser(buf: &mut limit_stream_runtime::utils::Bytes) -> Result<Self, ()> {
                        if limit_stream_runtime::utils::ls_read_array_len(buf)? != 3 {
                            return Err(());
                        }
                        #[allow(invalid_value)]
                        let mut value = unsafe { core::mem::MaybeUninit::<#struct_name>::uninit().assume_init() };
                        #body
                        Ok(value)
                    }
                }
            };
            // println!("{}", r.to_string());
            return r.into();
        }
        syn::Data::Enum(_de) => {
            // de.variants.into_iter().map(|x| x.)
            todo!()
        }
        _ => panic!("not support union"),
    }
}
