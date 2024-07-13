use std::{collections::HashMap, error::Error};

use syn::{parenthesized, token, LitStr};

#[derive(Debug)]
pub struct FieldMetaMap {
    pub field_meta_map: HashMap<String, HashMap<String, String>>,
}

impl FieldMetaMap {
    pub fn new_from(variants: &Vec<&syn::Variant>) -> Self {
        let mut field_meta_map = HashMap::new();
        for variant in variants.iter() {
            println!("{:?}", variant);
            for attr in variant.attrs.iter() {
                println!("{:?}", attr);
                let hhh = meta_to_hashmap(attr).unwrap();

                field_meta_map.insert(variant.ident.clone().to_string(), hhh);
            }
        }

        return Self {
            field_meta_map: field_meta_map,
        };
    }
    pub fn new(fields: &Vec<&syn::Field>) -> Self {
        let mut field_meta_map = HashMap::new();
        for field in fields.iter() {
            for attr in field.attrs.iter() {
                let hhh = meta_to_hashmap(attr).unwrap();

                println!("{:?}", field.ident.clone());

                field_meta_map.insert(field.ident.clone().unwrap().to_string(), hhh);
            }
        }

        return Self {
            field_meta_map: field_meta_map,
        };
    }

    pub fn get_value(&self, field_name: String, key: String) -> Option<String> {
        let field_map = self.field_meta_map.get(&field_name)?;

        let value = field_map.get(&key)?;

        return Some(value.clone());
    }
}

pub fn meta_to_hashmap(attr: &syn::Attribute) -> Result<HashMap<String, String>, Box<dyn Error>> {
    let mut r = HashMap::new();

    println!("attr: {:?}", attr);

    // 純粋なEnumの時にエラー
    attr.parse_nested_meta(|meta| {
        for path in &meta.path.segments {
            let k = path.ident.to_string();
            eprintln!("key: {:?}", k);

            let value = meta.value()?;
            let s: LitStr = value.parse()?;
            let v = s.value();
            eprintln!("value: {:?}", v);

            r.insert(k, v);
        }

        return Ok(());
    })?;
    // .unwrap();
    /*
    let metas = attr.parse_meta().unwrap();

    if let syn::Meta::List(aaa) = metas {
        for bbb in aaa.nested.iter() {
            match bbb {
                syn::NestedMeta::Meta(syn::Meta::NameValue(nv)) => {
                    // eprintln!("bc: {:?}", nv);
                    for path in &nv.path.segments {
                        if let syn::Lit::Str(vvv) = &nv.lit {
                            // eprintln!("{:?}", path.ident);
                            r.insert(path.ident.to_string(), vvv.value());
                        }
                    }
                }
                _ => {}
            }
        }
    }

    */
    return Ok(r);
}
