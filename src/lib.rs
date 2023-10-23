use std::{fs::File, io::Read};

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};
use limit_stream::codegen::{idl2rust, rust::Rust};

#[proc_macro]
pub fn include_idl(ident: TokenStream) -> TokenStream {
    let ident = ident.to_string();
    println!("{}", ident);
    let mut f = File::open(ident).unwrap();
    let mut src = String::new();
    f.read_to_string(&mut src).unwrap();
    let code = idl2rust(&src, &mut Rust {
        tab_size: 0,
        indent: 0,
        enum_id: Default::default(),
        codegen_regester: Default::default(),
    });
    code.parse().unwrap()
}

#[proc_macro]
pub fn inline_idl(ident: TokenStream) -> TokenStream {
    let src = ident.to_string();
    println!("{}", src);
    let code = idl2rust(&src, &mut Rust {
        tab_size: 0,
        indent: 0,
        enum_id: Default::default(),
        codegen_regester: Default::default(),
    });
    code.parse().unwrap()
}

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
            let len = ds.fields.len();
            let r = quote! {
              impl limit_stream_runtime::Ser for #struct_name {
                fn ser(&self, buf: &mut limit_stream_runtime::utils::ByteBuf) -> Result<(), ()> {
                    limit_stream_runtime::utils::ls_write_array_len(buf, #len as u32)?;
                    #body
                    Ok(())
                }
              }
            };
            r.into()
        }
        syn::Data::Enum(de) => {
            let body = de
                .variants
                .iter()
                .map(|f| {
                    let ident = &f.ident;
                    let ident_str = ident.to_string();
                    quote!(#struct_name::#ident(v) => {
                        limit_stream_runtime::utils::ls_write_str(buf, #ident_str)?;
                        v.ser(buf)?;
                    })
                })
                .reduce(|mut l, r| {
                    l.extend(vec![r]);
                    l
                })
                .unwrap();
            let r = quote! {
                impl limit_stream_runtime::Ser for #struct_name {
                    fn ser(&self, buf: &mut limit_stream_runtime::utils::ByteBuf) -> Result<(), ()> {
                        limit_stream_runtime::utils::ls_write_array_len(buf, 2)?;
                        match self {
                            #body
                        }
                        Ok(())
                    }
                }
            };
            r.into()
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
            let len = ds.fields.len();
            let r = quote! {
                impl limit_stream_runtime::Deser for #struct_name {
                    type Res = Result<Self, ()>;
                    fn deser(buf: &mut limit_stream_runtime::utils::Bytes) -> Result<Self, ()> {
                        if limit_stream_runtime::utils::ls_read_array_len(buf)? != #len as u32 {
                            return Err(());
                        }
                        #[allow(invalid_value)]
                        let mut value = unsafe { core::mem::MaybeUninit::<#struct_name>::uninit().assume_init() };
                        #body
                        Ok(value)
                    }
                }
            };
            r.into()
        }
        syn::Data::Enum(de) => {
            let body = de
                .variants
                .iter()
                .map(|f| {
                    let ident = &f.ident;
                    let ident_str = ident.to_string();
                    quote!(#ident_str => Ok(#struct_name::#ident(limit_stream_runtime::utils::ls_read_value(buf)?)),)
                })
                .reduce(|mut l, r| {
                    l.extend(vec![r]);
                    l
                })
                .unwrap();
            let r = quote! {
                impl limit_stream_runtime::Deser for #struct_name {
                    type Res = Result<Self, ()>;
                    fn deser(buf: &mut limit_stream_runtime::utils::Bytes) -> Result<Self, ()> {
                        if limit_stream_runtime::utils::ls_read_array_len(buf)? != 2 {
                            return Err(());
                        }
                        match limit_stream_runtime::utils::ls_read_str(buf)?.as_str() {
                            #body
                            _ => return Err(()),
                        }
                    }
                }
            };
            r.into()
        }
        _ => panic!("not support union"),
    }
}
