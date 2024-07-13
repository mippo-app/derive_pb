use super::FieldMetaMap;

#[derive(Clone)]

pub struct VariantAttr {
    pub is_skip: bool,
    pub type_vec: Vec<proc_macro2::Ident>,
    pub from_pb_func_name: Option<String>,
    pub to_pb_func_name: Option<String>,
    pub field: syn::Variant,
}

impl VariantAttr {
    pub fn new(f: &syn::Variant, field_meta_map: &FieldMetaMap) -> Self {
        let field_name = f.ident.clone().to_string();
        println!("{:?}", field_name);

        let is_skip = if let Some(_skip) =
            field_meta_map.get_value(field_name.clone(), String::from("skip"))
        {
            true
        } else {
            false
        };

        let mut type_vec = vec![];
        if let syn::Fields::Named(syn::FieldsNamed { ref named, .. }) = f.fields {
            if named.len() > 0 {
                type_vec = get_type_vec(&named[0]);
            }
        }

        println!("{:?}", type_vec);

        let from_pb_func_name =
            field_meta_map.get_value(field_name.clone(), String::from("from_pb_func_name"));
        let to_pb_func_name =
            field_meta_map.get_value(field_name.clone(), String::from("to_pb_func_name"));

        return Self {
            is_skip,
            type_vec,
            from_pb_func_name,
            to_pb_func_name,
            field: f.clone(),
        };
    }
}

#[derive(Debug)]
pub struct FieldAttr {
    pub is_skip: bool,
    pub type_vec: Vec<proc_macro2::Ident>,
    pub from_pb_func_name: Option<String>,
    pub to_pb_func_name: Option<String>,
    pub field: syn::Field,
}

impl FieldAttr {
    pub fn new(f: &syn::Field, field_meta_map: &FieldMetaMap) -> Self {
        println!("FieldAttr: {:?}", f);
        let field_name = f.ident.clone().unwrap().to_string();

        let is_skip = if let Some(_skip) =
            field_meta_map.get_value(field_name.clone(), String::from("skip"))
        {
            true
        } else {
            false
        };
        let type_vec = get_type_vec(f);

        let from_pb_func_name =
            field_meta_map.get_value(field_name.clone(), String::from("from_pb_func_name"));
        let to_pb_func_name =
            field_meta_map.get_value(field_name.clone(), String::from("to_pb_func_name"));

        return Self {
            is_skip,
            type_vec,
            from_pb_func_name,
            to_pb_func_name,
            field: f.clone(),
        };
    }

    pub fn get_field_name(&self) -> syn::Ident {
        println!("get_field_name: {:?}", self.field.ident);
        return self.field.ident.clone().unwrap();
    }
}

fn get_type_vec(field: &syn::Field) -> Vec<syn::Ident> {
    let mut r = vec![];
    if let syn::Type::Path(syn::TypePath {
        path: syn::Path { ref segments, .. },
        ..
    }) = field.ty
    {
        let (name, sgs) = get_path_name_path(&segments[0].clone());
        // println!("1: {:?}", name); // HashMap
        r.push(name);
        for sg in sgs.iter() {
            let (name, sgs) = get_path_name_path(&sg.clone());
            // println!("2: {:?}", name); // String, Vec
            // println!("2: {:?}", sgs);
            r.push(name);

            for sg in sgs.iter() {
                let (name, _sgs) = get_path_name_path(&sg.clone());
                // println!("3: {:?}", name); // Seeds
                // println!("3: {:?}", sgs);

                r.push(name);
            }
        }
    }

    return r;
}

fn get_path_name_path(segment: &syn::PathSegment) -> (syn::Ident, Vec<syn::PathSegment>) {
    //
    let mut r_args = vec![];
    if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
        // key
        for arg in args.args.iter() {
            if let syn::GenericArgument::Type(syn::Type::Path(bbb)) = &arg {
                // eprintln!("XXXXXXXXXXXXXXXXXXXXXXXXX");
                // eprintln!("{:?}", bbb);
                for segment in bbb.path.segments.iter() {
                    r_args.push(segment.clone());
                }
                // r_args.push()
            }
        }
    }

    // println!("{:?}", &segment);

    return (segment.ident.clone(), r_args);
}
