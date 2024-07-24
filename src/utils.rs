use proc_macro2::TokenStream; // Punct, Spacing, Span, TokenTree
use quote::quote;
use quote::ToTokens; // TokenStreamExt

use syn::DeriveInput;
use syn::Expr;
use syn::ExprLit;
use syn::Lit;

pub fn get_target_info(
    ast: &DeriveInput,
) -> (proc_macro2::Ident, proc_macro2::Ident, String, bool) {
    let name = &ast.ident;
    let mut pb_name = String::from("");
    let mut module_path = String::from("m_helper");
    let mut is_pure_enum = false;
    let mut is_primitive = false;

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
    println!("B");

    return (name.clone(), pb_name_, module_path, is_pure_enum);
}

pub fn get_to_field_quote(
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

pub fn get_from_field_quote(
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
