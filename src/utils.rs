use proc_macro2::TokenStream; // Punct, Spacing, Span, TokenTree
use quote::quote;
use quote::ToTokens; // TokenStreamExt

use syn::DeriveInput;
use syn::Expr;
use syn::ExprLit;
use syn::Lit;

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
                        } else {
                            let pb_statement: syn::Type = syn::parse_str(&format!(
                                "pb_Valueof::{}",
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
                                "pb_Valueof::{}",
                                f.field.ident.to_string().as_str()
                            ))
                            .unwrap();

                            let a = quote! {
                              #aa(value) => #bb { valueof: Some(#cc(value.into()))}
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
                          return match ssss.valueof.unwrap() {
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

        println!("{:?} {:?} {:?}", name, pb_name, module_path);

        let mut variants = vec![];
        if let syn::Data::Enum(syn::DataEnum { variants: v, .. }) = &ast.data {
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

pub fn get_target_info(
    ast: &DeriveInput,
) -> (proc_macro2::Ident, proc_macro2::Ident, String, bool) {
    let name = &ast.ident;
    let mut pb_name = String::from("");
    let mut module_path = String::from("helper");
    let mut is_pure_enum = false;

    // eprintln!("{:?}", name);
    for attr in ast.attrs.iter() {
        println!("attr: {:?}", attr);

        if let Ok(hhh) = super::field_meta_map::meta_to_hashmap(attr) {
            eprintln!("hashmap: {:?}", hhh);
            let kkk = String::from("pb_name");
            if let Some(vv) = hhh.get(&kkk) {
                // pb_name = Ident::new(vv.clone().as_str(), Span::call_site());
                pb_name = vv.clone();
            }

            let iii = String::from("module_path");
            if let Some(vv) = hhh.get(&iii) {
                module_path = vv.clone();
            }
        } else {
            is_pure_enum = true;
        }
    }
    println!("A");

    let pb_name_ = syn::Ident::new(&pb_name, name.span()); //.as_str();

    return (name.clone(), pb_name_, module_path, is_pure_enum);
}

fn get_to_field_quote(
    module_path: &String,
    name: syn::Ident,
    type_vec: Vec<syn::Ident>,
    parent_type: Option<syn::Ident>,
    func_name: Option<String>,
) -> proc_macro2::TokenStream {
    // println!("get_field_quote: {:?}", type_vec);
    if let Some(func_name) = func_name {
        let f_name = get_custom_func_ident(func_name);

        let a = quote! {
            #name: #f_name(ssss.#name)
        };

        return a;
    }

    if name == "any"
        || name == "anys"
        || vec!["String", "i32", "u8", "u32", "bool", "f32", "i64", "f64"]
            .contains(&type_vec[0].to_string().as_str())
            == true
    {
        return quote! {
            #name: ssss.#name
        };
    } else if type_vec[0] == "Option" || type_vec[0] == "Vec" {
        return get_to_field_quote(
            module_path,
            name,
            get_next_type_vec(type_vec.clone()),
            Some(type_vec[0].clone()),
            func_name,
        );
    } else if type_vec[0] == "HashMap" {
        return get_to_func_name_quote(
            name,
            format!("{}::conv_map_one_to_map_pb", module_path.as_str()),
            None,
            None,
        );
    }
    /* else if type_vec[0] == "Uuid" {
        return get_to_func_name_quote(
            name,
            format!("{}::conv_uuid_one_to_uuid_pb", module_path.as_str()),
            None,
            None,
        );
    } */
    else {
        if let Some(aaa) = parent_type {
            if aaa == "Option" {
                return get_to_func_name_quote(
                    name,
                    format!("{}::conv_option_one_to_option_pb", module_path.as_str()),
                    None,
                    None,
                );
            } else if aaa == "Vec" {
                /*return get_to_func_name_quote(
                  name,
                  format!("{}::conv_multi_to_multi_pb", module_path.as_str()),
                  Some(type_vec[0].clone()),
                  Some(syn::Ident::new(
                    &format!("pb_{}", type_vec[0]),
                    type_vec[0].span(),
                  )),
                );*/
                return get_to_func_name_quote(
                    name,
                    format!("{}::conv_multi_to_multi_pb", module_path.as_str()),
                    None,
                    None,
                );
            }
        }
        return quote! {
            #name: ssss.#name.into()
        };
    }
}

fn get_custom_func_ident(func_name: String) -> syn::Type {
    println!(
        "get_custom_func_ident: {:?}",
        format!("{}", func_name.as_str())
    );

    return syn::parse_str(&format!("{}", func_name.as_str())).unwrap();
}

fn get_from_field_quote(
    module_path: &String,
    name: syn::Ident,
    type_vec: Vec<syn::Ident>,
    parent_type: Option<syn::Ident>,
    func_name: Option<String>,
) -> proc_macro2::TokenStream {
    println!(
        "get_from_field_quote: {:?} {:?} {:?} {:?} {:?}",
        name, module_path, type_vec, parent_type, func_name
    );
    if let Some(f) = func_name {
        let f_name = get_custom_func_ident(f);
        return quote! {
            #name: #f_name(ssss.#name)
        };
    }

    if name == "any"
        || name == "anys"
        || vec!["String", "i32", "u8", "u32", "bool", "f32", "i64", "f64"]
            .contains(&type_vec[0].to_string().as_str())
            == true
    {
        return quote! {
            #name: ssss.#name
        };
    } else if type_vec[0] == "Option" || type_vec[0] == "Vec" {
        return get_from_field_quote(
            module_path,
            name,
            get_next_type_vec(type_vec.clone()),
            Some(type_vec[0].clone()),
            func_name,
        );
    } else if type_vec[0] == "HashMap" {
        return get_from_func_name_quote(
            name,
            format!("{}::conv_pb_map_to_map_one", module_path.as_str()),
            None,
            None,
        );
    }
    /* else if type_vec[0] == "Uuid" {
        return get_from_func_name_quote(
            name,
            format!("{}::conv_pb_uuid_to_uuid_one", module_path.as_str()),
            None,
            None,
        );
    }  */
    else {
        if let Some(parent_type) = parent_type {
            if parent_type == "Option" {
                return get_from_func_name_quote(
                    name,
                    format!("{}::conv_option_pb_to_option_one", module_path.as_str()),
                    None,
                    None,
                );
            } else if parent_type == "Vec" {
                return get_from_func_name_quote(
                    name,
                    format!("{}::conv_pb_multi_to_multi", module_path.as_str()),
                    None,
                    None,
                );
                /*

                let pb_type_name_ = syn::Ident::new(&format!("pb_{}", type_vec[0]), type_vec[0].span()); //.as_str();

                  let ttt = &type_vec[0];
                    return get_to_func_name_quote(
                      name,
                      format!("{}::conv_pb_multi_to_multi", module_path.as_str()),
                      None,
                      None,
                    );
                    */
            }
        }

        /*let pb_type_name_ = syn::Ident::new(&format!("pb_{}", type_vec[0]), type_vec[0].span());
        return quote! {
            #name: #pb_type_name_::from(ssss.#name)
        };
        */
        return quote! {
            #name: ssss.#name.into()
        };
    }
}

pub fn get_next_type_vec(type_vec: Vec<syn::Ident>) -> Vec<syn::Ident> {
    let mut nnn = vec![];
    for (i, val) in type_vec.iter().enumerate() {
        if i != 0 {
            nnn.push(val.clone());
        }
    }

    return nnn;
}

// -------------------------------------

pub fn get_to_func_name_quote(
    name: syn::Ident,
    default_name: String,
    t: Option<proc_macro2::Ident>,
    u: Option<proc_macro2::Ident>,
) -> proc_macro2::TokenStream {
    let f = get_custom_func_ident(default_name);

    if let (Some(t), Some(u)) = (t, u) {
        quote! {
            #name: #f::<#t, #u>(ssss.#name)
        }
    } else {
        quote! {
            #name: #f(ssss.#name)
        }
    }
}

pub fn get_from_func_name_quote(
    name: syn::Ident,
    default_name: String,
    t: Option<proc_macro2::Ident>,
    u: Option<proc_macro2::Ident>,
) -> proc_macro2::TokenStream {
    let f = get_custom_func_ident(default_name);

    if let (Some(t), Some(u)) = (t, u) {
        quote! {
            #name: #f::<#t, #u>(ssss.#name)
        }
    } else {
        quote! {
            #name: #f(ssss.#name)
        }
    }
}
