extern crate proc_macro;

mod a_enum;
mod a_struct;
mod field_attr;
mod field_meta_map;
mod utils;

use field_attr::{FieldAttr, VariantAttr};
use field_meta_map::FieldMetaMap;

use proc_macro::TokenStream;

// use proc_macro2::{Ident, Span};

use quote::quote;
use syn::{parse_macro_input, DeriveInput};

// HashMap<String, Vec<Seed>>
/*
{"name": "HashMap", "args": [
  {"name": "String", "args": []},
  {"name": "Vec", "args": [
      {"name": "Seed", "args": []}
  ]}
]}
*/
/*
fn get_path_args(segment: syn::PathSegment) -> (syn::Ident, syn::PathSegment) {
    //
    let mut r_name;
    let mut r_args;
    if segment.ident == "HashMap" {
        if let syn::PathArguments::AngleBracketed(args) = segment.arguments {
            // key
            if let syn::GenericArgument::Type(syn::Type::Path(bbb)) = &args.args[0] {
                let k = &bbb.path.segments[0].ident;
                eprintln!("key: {:?}", k);
                r_name = k;
            }
            // value
            if let syn::GenericArgument::Type(syn::Type::Path(bbb)) = &args.args[1] {
                let v = &bbb.path.segments[0].ident;
                let v_args = &bbb.path.segments[0].arguments;
                eprintln!("value: {:?}", v);
                eprintln!("value: {:?}", v_args);
            }


            /*
            for aaa in args.args.iter() {
                if let syn::GenericArgument::Type(syn::Type::Path(bbb)) = aaa {
                    // eprintln!("ddd: {:?}", bbb);
                    for segment in bbb.path.segments.iter() {
                        eprintln!("ddd: {:?}", segment);
                    }
                }
            }*/
        }
    }

    return r_name, r_args
}
*/

#[proc_macro_derive(Pb, attributes(pb))]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    if let syn::Data::Struct(syn::DataStruct {
        fields: syn::Fields::Named(syn::FieldsNamed { ref named, .. }),
        ..
    }) = &ast.data
    {
        println!("struct");
        let struct_info = a_struct::StructInfo::new(&ast);

        let a = quote! {
            #struct_info
        };

        // eprintln!("{}", a);

        return TokenStream::from(a);
    } else if let syn::Data::Enum(aaa) = &ast.data {
        println!("Enum");
        let enum_info = a_enum::EnumInfo::new(&ast);

        let a = quote! {
            #enum_info
        };

        // eprintln!("{}", a);

        return TokenStream::from(a);
    }

    return TokenStream::from(quote! {});
}
