use proc_macro2::TokenStream; // Punct, Spacing, Span, TokenTree
use quote::quote;
use quote::ToTokens; // TokenStreamExt

use syn::DeriveInput;
use syn::Expr;
use syn::ExprLit;
use syn::Lit;

use crate::utils::get_target_info;

pub struct EnumInfo {
    pub is_enable: bool,
    pub is_pure_enum: bool,
    pub name: syn::Ident,
    pub pb_name: syn::Ident,
    pub module_path: String,
    pub variant_meta_map: super::FieldMetaMap,
    pub variant_attrs: Vec<super::VariantAttr>,
}

impl ToTokens for EnumInfo {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        if self.is_enable == true {
            let pb_name = self.pb_name.clone();
            let name = self.name.clone();
            let from_pb_fields: Vec<proc_macro2::TokenStream> = self
                .variant_attrs
                .iter()
                .filter_map(|f| {
                    if f.is_skip == false {
                        if self.is_pure_enum == true {
                            //  || (self.is_pure_enum == false && )
                            let pb_statement: syn::Type = syn::parse_str(&format!(
                                "{}::{}",
                                pb_name.to_string().as_str(),
                                f.field.ident.to_string().as_str()
                            ))
                            .unwrap();
                            let pure_statement: syn::Type = syn::parse_str(&format!(
                                "{}::{}",
                                name.to_string().as_str(),
                                f.field.ident.to_string().as_str()
                            ))
                            .unwrap();

                            let a = quote! {
                              #pb_statement => #pure_statement
                            };

                            return Some(a);
                        } else if f.type_vec.len() == 0 {
                            let pb_statement: syn::Type = syn::parse_str(&format!(
                                "pb_ValueOf::{}",
                                f.field.ident.to_string().as_str()
                            ))
                            .unwrap();
                            let pure_statement: syn::Type = syn::parse_str(&format!(
                                "{}::{}",
                                name.to_string().as_str(),
                                f.field.ident.to_string().as_str()
                            ))
                            .unwrap();

                            let a = quote! {
                              #pb_statement(value) => #pure_statement(value.into())
                            };
                            return Some(a);
                        } else {
                            let pb_statement: syn::Type = syn::parse_str(&format!(
                                "pb_ValueOf::{}",
                                f.field.ident.to_string().as_str()
                            ))
                            .unwrap();
                            let pure_statement: syn::Type = syn::parse_str(&format!(
                                "{}::{}",
                                name.to_string().as_str(),
                                f.field.ident.to_string().as_str()
                            ))
                            .unwrap();

                            let a = quote! {
                              #pb_statement(value) => #pure_statement(value.into())
                            };

                            return Some(a);
                        }
                    } else {
                        None
                    }
                })
                .collect();
            let to_pb_fields: Vec<proc_macro2::TokenStream> = self
                .variant_attrs
                .iter()
                .filter_map(|f| {
                    if f.is_skip == false {
                        if self.is_pure_enum == true {
                            let aa: syn::Type = syn::parse_str(&format!(
                                "{}::{}",
                                name.to_string().as_str(),
                                f.field.ident.to_string().as_str()
                            ))
                            .unwrap();

                            let bb: syn::Type = syn::parse_str(&format!(
                                "{}::{}",
                                pb_name.to_string().as_str(),
                                f.field.ident.to_string().as_str()
                            ))
                            .unwrap();

                            let a = quote! {
                              #aa => #bb
                            };
                            return Some(a);
                        } else if f.type_vec.len() == 0 {
                            let aa: syn::Type = syn::parse_str(&format!(
                                "{}::{}",
                                name.to_string().as_str(),
                                f.field.ident.to_string().as_str()
                            ))
                            .unwrap();

                            let bb: syn::Type =
                                syn::parse_str(&format!("{}", pb_name.to_string().as_str(),))
                                    .unwrap();

                            let cc: syn::Type = syn::parse_str(&format!(
                                "pb_ValueOf::{}",
                                f.field.ident.to_string().as_str()
                            ))
                            .unwrap();

                            let a = quote! {
                              #aa(value) => #cc(value.into())
                            };

                            return Some(a);
                        } else {
                            let aa: syn::Type = syn::parse_str(&format!(
                                "{}::{}",
                                name.to_string().as_str(),
                                f.field.ident.to_string().as_str()
                            ))
                            .unwrap();

                            let bb: syn::Type =
                                syn::parse_str(&format!("{}", pb_name.to_string().as_str(),))
                                    .unwrap();

                            let cc: syn::Type = syn::parse_str(&format!(
                                "pb_ValueOf::{}",
                                f.field.ident.to_string().as_str()
                            ))
                            .unwrap();

                            let a = quote! {
                              #aa(value) => Some(#cc(value.into()))
                            };

                            return Some(a);
                        }
                    } else {
                        None
                    }
                })
                .collect();
            //
            if self.is_pure_enum == true {
                let i_from_fields: Vec<proc_macro2::TokenStream> = self
                    .variant_attrs
                    .iter()
                    .filter_map(|f| {
                        if f.is_skip == false {
                            let field = f.field.clone();
                            if let Expr::Lit(ExprLit {
                                lit: Lit::Int(ffff),
                                ..
                            }) = field.discriminant.unwrap().1
                            {
                                println!("variant: {:?} | {:?}", field.ident, ffff);
                                let aaa = format!(
                                    "{}::{}",
                                    name.to_string().as_str(),
                                    f.field.ident.to_string().as_str()
                                );

                                let aa: syn::Type = syn::parse_str(&aaa).unwrap();

                                return Some(quote! {
                                  #ffff => #aa
                                });
                            }
                            return None;
                        } else {
                            return None;
                        }
                    })
                    .collect();

                let i_into_fields: Vec<proc_macro2::TokenStream> = self
                    .variant_attrs
                    .iter()
                    .filter_map(|f| {
                        if f.is_skip == false {
                            let field = f.field.clone();
                            if let Expr::Lit(ExprLit {
                                lit: Lit::Int(ffff),
                                ..
                            }) = field.discriminant.unwrap().1
                            {
                                println!("variant: {:?} | {:?}", field.ident, ffff);
                                let aaa = format!(
                                    "{}::{}",
                                    name.to_string().as_str(),
                                    f.field.ident.to_string().as_str()
                                );

                                let aa: syn::Type = syn::parse_str(&aaa).unwrap();

                                return Some(quote! {
                                    #aa => #ffff
                                });
                            }
                            return None;
                        } else {
                            return None;
                        }
                    })
                    .collect();

                let a = quote! {
                  impl From<i32> for #name {
                      fn from(ssss: i32) -> Self {
                          return match ssss {
                            #(#i_from_fields,)*
                            _ => panic!("{:?}", ssss),
                          };
                      }
                  }

                  impl Into<i32> for #name {
                    fn into(self) -> i32 {
                        return match self {
                          #(#i_into_fields,)*
                        };
                    }
                }
                };
                tokens.extend(a);
            }

            if self.is_pure_enum == true {
                let a = quote! {
                  impl From<#pb_name> for #name {
                      fn from(ssss: #pb_name) -> Self {
                          return match ssss {
                            #(#from_pb_fields,)*
                          };
                      }
                  }
                  impl From<#name> for #pb_name {
                      fn from(ssss: #name) -> Self {
                          return match ssss {
                            #(#to_pb_fields,)*
                          };
                      }
                  }
                };
                tokens.extend(a);
            } else {
                let a = quote! {
                  impl From<#pb_name> for #name {
                      fn from(ssss: #pb_name) -> Self {
                          return match ssss {
                            #(#from_pb_fields,)*
                          };
                      }
                  }
                  impl From<#name> for #pb_name {
                      fn from(ssss: #name) -> Self {
                          return match ssss {
                            #(#to_pb_fields,)*
                          };
                      }
                  }
                };
                tokens.extend(a);
            }
        }
        // a.to_tokens(tokens);
        // tokens.append();
    }
}

impl EnumInfo {
    pub fn new(ast: &DeriveInput) -> Self {
        let (name, pb_name, module_path, is_pure_enum) = get_target_info(&ast);

        println!("new: {:?}|{:?}|{:?}", name, pb_name, module_path);

        let mut variants = vec![];
        if let syn::Data::Enum(syn::DataEnum { variants: v, .. }) = &ast.data {
            println!("dataenum: {:?}", v);
            variants = v.into_iter().collect();
        }

        let mut is_enable = false;
        if variants.len() > 0 {
            is_enable = true
        }

        /*let variants = if let syn::Data::Enum(syn::DataEnum { variants, .. }) = &ast.data {
          variants
        } else {
          unimplemented!();
        };*/

        let variant_meta_map = super::FieldMetaMap::new_from(&variants);
        println!("{:?}", variant_meta_map);

        let mut variant_attrs = vec![];
        for f in variants.iter() {
            let variant_attr = super::VariantAttr::new(&f, &variant_meta_map);

            variant_attrs.push(variant_attr)
        }

        return Self {
            is_enable: is_enable,
            is_pure_enum: is_pure_enum,
            name: name,
            pb_name: pb_name,
            module_path: module_path,
            // fields: fields.clone(),
            variant_meta_map: variant_meta_map,
            variant_attrs: variant_attrs,
        };
    }
}
