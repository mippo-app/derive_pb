use proc_macro2::TokenStream; // Punct, Spacing, Span, TokenTree
use quote::quote;
use quote::ToTokens; // TokenStreamExt

use syn::DeriveInput;
use syn::Expr;
use syn::ExprLit;
use syn::Lit;

use crate::utils::get_from_field_quote;
use crate::utils::get_target_info;
use crate::utils::get_to_field_quote;

pub struct StructInfo {
    pub is_enable: bool,
    pub name: syn::Ident,
    pub pb_name: syn::Ident,
    pub module_path: String,
    // pub fields: syn::punctuated::Punctuated<syn::Field, syn::token::Comma>,
    pub field_meta_map: super::FieldMetaMap,
    pub field_attrs: Vec<super::FieldAttr>,
}

impl ToTokens for StructInfo {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        if self.is_enable == true {
            let pb_name = self.pb_name.clone();
            let name = self.name.clone();
            //
            let from_pb_fields = self.get_from_pb_fields();
            let to_pb_fields = self.get_to_pb_fields();
            //
            let a = quote! {
              impl From<#pb_name> for #name {
                  fn from(ssss: #pb_name) -> Self {
                      return #name {
                        #(#from_pb_fields,)*
                      };
                  }
              }
              impl From<#name> for #pb_name {
                  fn from(ssss: #name) -> Self {
                      return #pb_name {
                        #(#to_pb_fields,)*
                      };
                  }
              }
            };
            tokens.extend(a);
        }
        // a.to_tokens(tokens);
        // tokens.append();
    }
}

impl StructInfo {
    pub fn new(ast: &DeriveInput) -> Self {
        let mut fields = vec![];
        if let syn::Data::Struct(syn::DataStruct {
            fields: syn::Fields::Named(syn::FieldsNamed { ref named, .. }),
            ..
        }) = &ast.data
        {
            fields = named.into_iter().collect();
        }

        if let syn::Data::Enum(aaa) = &ast.data {
            let a = aaa.variants.clone();
            for variant in a.into_iter() {
                if let Expr::Lit(ExprLit {
                    lit: Lit::Int(ffff),
                    ..
                }) = variant.discriminant.unwrap().1
                {
                    println!("variant: {:?} | {:?}", variant.ident, ffff);
                }
            }
        }

        println!("{:?}", fields.len());

        let mut is_enable = false;
        if fields.len() > 0 {
            is_enable = true
        }

        let (name, pb_name, module_path, is_pure_enum) = get_target_info(&ast);

        println!("AAA");

        /*
        let fields = if let syn::Data::Struct(syn::DataStruct {
          fields: syn::Fields::Named(syn::FieldsNamed { ref named, .. }),
          ..
        }) = &ast.data
        {
          named
        } else {
          unimplemented!();
        };
        */

        let field_meta_map = super::FieldMetaMap::new(&fields);

        println!("BBB");

        println!("field_meta_map: {:?}", field_meta_map);

        let mut field_attrs = vec![];
        for f in fields.iter() {
            let field_attr = super::FieldAttr::new(&f, &field_meta_map);
            field_attrs.push(field_attr)
        }

        return Self {
            is_enable: is_enable,
            name: name,
            pb_name: pb_name,
            module_path: module_path,
            // fields: fields.clone(),
            field_meta_map: field_meta_map,
            field_attrs: field_attrs,
        };
    }

    pub fn get_from_pb_fields(&self) -> Vec<proc_macro2::TokenStream> {
        return self
            .field_attrs
            .iter()
            .filter_map(|f| {
                println!("get_from_pb_fields: {:?}", f);
                if f.is_skip == false {
                    return Some(get_from_field_quote(
                        &self.module_path,
                        f.get_field_name(),
                        f.type_vec.clone(),
                        None,
                        f.from_pb_func_name.clone(),
                    ));
                } else {
                    None
                }
            })
            .collect();
    }

    pub fn get_to_pb_fields(&self) -> Vec<TokenStream> {
        return self
            .field_attrs
            .iter()
            .filter_map(|f| {
                if f.is_skip == false {
                    return Some(get_to_field_quote(
                        &self.module_path,
                        f.get_field_name(),
                        f.type_vec.clone(),
                        None,
                        f.to_pb_func_name.clone(),
                    ));
                } else {
                    None
                }
            })
            .collect();
    }
}
