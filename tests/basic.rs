use derive_pb::Pb;

#[test]
fn test_macro() {
    // cargo test --test basic -- test_macro --nocapture
    trait HelloMacro {
        fn hello_macro() -> String {
            String::from("666")
        }
    }

    struct pb_Hoge {
        id: u64,
        name: String,
    }

    #[derive(Pb)]
    #[pb(pb_name = "pb_Hoge", aa = "A")]
    struct Hoge {
        id: u64,
        name: String,
    }

    // assert_eq!(Hoge::hello_macro(), "A".to_string());
    // cargo expand --test basic -- test_macro
}

#[test]
fn test_macro2() {
    // cargo test --test basic -- test_macro2 --nocapture

    pub mod a {
        use derive_pb::Pb;

        pub struct pb_Uuid {
            pub target_type: String,

            pub uuid: Option<String>,
        }

        pub struct pb_Id {
            pub valueof: Option<ValueOf>,
        }
        /// Nested message and enum types in `ID`.
        pub enum pb_Valueof {
            Uuid(pb_Uuid),
        }

        #[derive(Pb)]
        #[pb(pb_name = "pb_Uuid")]
        pub struct Uuid {
            pub target_type: String,
            pub uuid: Option<String>,
        }

        #[derive(Pb)]
        #[pb(pb_name = "pb_Valueof")]
        pub enum ValueOf {
            Uuid(pb_Uuid),
        }

        #[derive(Pb)]
        #[pb(pb_name = "pb_Id")]
        pub struct Id {
            pub valueof: Option<ValueOf>,
        }
    }

    // assert_eq!(Hoge::hello_macro(), "A".to_string());
    // cargo expand --test basic -- test_macro2
}

fn test_enum() {
    // cargo expand --test basic -- test_enum
    // cargo test --test basic -- test_enum --nocapture
    pub mod a {
        pub mod pp {

            #[repr(i32)]
            pub enum MantleType {
                Unknown = 0,
                Sand = 1,
                Stone = 2,
            }
            impl MantleType {
                /// String value of the enum field names used in the ProtoBuf definition.
                ///
                /// The values are not transformed in any way and thus are considered stable
                /// (if the ProtoBuf definition does not change) and safe for programmatic use.
                pub fn as_str_name(&self) -> &'static str {
                    match self {
                        MantleType::Unknown => "MANTLE_TYPE_UNKNOWN",
                        MantleType::Sand => "MANTLE_TYPE_SAND",
                        MantleType::Stone => "MANTLE_TYPE_STONE",
                    }
                }
                /// Creates an enum from field names used in the ProtoBuf definition.
                pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                    match value {
                        "MANTLE_TYPE_UNKNOWN" => Some(Self::Unknown),
                        "MANTLE_TYPE_SAND" => Some(Self::Sand),
                        "MANTLE_TYPE_STONE" => Some(Self::Stone),
                        _ => None,
                    }
                }
            }
        }

        pub mod mm {
            use derive_pb::Pb;

            use super::pp::MantleType as pb_MantleType;

            #[derive(Pb)]
            #[pb(pb_name = "pb_MantleType")]
            #[repr(i32)]
            pub enum MantleType {
                Unknown = 0,
                Sand = 1,
                Stone = 2,
            }
            impl MantleType {
                /// String value of the enum field names used in the ProtoBuf definition.
                ///
                /// The values are not transformed in any way and thus are considered stable
                /// (if the ProtoBuf definition does not change) and safe for programmatic use.
                pub fn as_str_name(&self) -> &'static str {
                    match self {
                        MantleType::Unknown => "MANTLE_TYPE_UNKNOWN",
                        MantleType::Sand => "MANTLE_TYPE_SAND",
                        MantleType::Stone => "MANTLE_TYPE_STONE",
                    }
                }
                /// Creates an enum from field names used in the ProtoBuf definition.
                pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                    match value {
                        "MANTLE_TYPE_UNKNOWN" => Some(Self::Unknown),
                        "MANTLE_TYPE_SAND" => Some(Self::Sand),
                        "MANTLE_TYPE_STONE" => Some(Self::Stone),
                        _ => None,
                    }
                }
            }
        }
    }
}

fn test_enum2() {
    // cargo expand --test basic -- test_enum
    pub mod a {
        pub mod pp {
            pub struct MantleId {
                pub place_id: String,
                pub mantle_type: i32,
                pub mineral_type: i32,
                pub rift_name: String,
            }

            #[repr(i32)]
            pub enum MantleType {
                Unknown = 0,
                Sand = 1,
                Stone = 2,
            }
            impl MantleType {
                /// String value of the enum field names used in the ProtoBuf definition.
                ///
                /// The values are not transformed in any way and thus are considered stable
                /// (if the ProtoBuf definition does not change) and safe for programmatic use.
                pub fn as_str_name(&self) -> &'static str {
                    match self {
                        MantleType::Unknown => "MANTLE_TYPE_UNKNOWN",
                        MantleType::Sand => "MANTLE_TYPE_SAND",
                        MantleType::Stone => "MANTLE_TYPE_STONE",
                    }
                }
                /// Creates an enum from field names used in the ProtoBuf definition.
                pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                    match value {
                        "MANTLE_TYPE_UNKNOWN" => Some(Self::Unknown),
                        "MANTLE_TYPE_SAND" => Some(Self::Sand),
                        "MANTLE_TYPE_STONE" => Some(Self::Stone),
                        _ => None,
                    }
                }
            }

            #[repr(i32)]
            pub enum MineralType {
                Unknown = 0,
                Obj = 1,
                Email = 2,
                Sheet = 3,
                Storage = 4,
                Pc = 5,
                Calc = 6,
            }
            impl MineralType {
                /// String value of the enum field names used in the ProtoBuf definition.
                ///
                /// The values are not transformed in any way and thus are considered stable
                /// (if the ProtoBuf definition does not change) and safe for programmatic use.
                pub fn as_str_name(&self) -> &'static str {
                    match self {
                        MineralType::Unknown => "MINERAL_TYPE_UNKNOWN",
                        MineralType::Obj => "MINERAL_TYPE_OBJ",
                        MineralType::Email => "MINERAL_TYPE_EMAIL",
                        MineralType::Sheet => "MINERAL_TYPE_SHEET",
                        MineralType::Storage => "MINERAL_TYPE_STORAGE",
                        MineralType::Pc => "MINERAL_TYPE_PC",
                        MineralType::Calc => "MINERAL_TYPE_CALC",
                    }
                }
                /// Creates an enum from field names used in the ProtoBuf definition.
                pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                    match value {
                        "MINERAL_TYPE_UNKNOWN" => Some(Self::Unknown),
                        "MINERAL_TYPE_OBJ" => Some(Self::Obj),
                        "MINERAL_TYPE_EMAIL" => Some(Self::Email),
                        "MINERAL_TYPE_SHEET" => Some(Self::Sheet),
                        "MINERAL_TYPE_STORAGE" => Some(Self::Storage),
                        "MINERAL_TYPE_PC" => Some(Self::Pc),
                        "MINERAL_TYPE_CALC" => Some(Self::Calc),
                        _ => None,
                    }
                }
            }

            pub struct MagnaInfo {
                pub mantle_id: ::core::option::Option<MantleId>,

                pub target_types: Vec<String>,

                pub result_types: Vec<String>,
            }
            /// rift情報
            ///
            pub struct RiftInfo {
                pub mantle_id: ::core::option::Option<MantleId>,

                pub target_types: Vec<String>,

                pub result_types: Vec<String>,
            }
        }

        pub mod mm {
            use derive_pb::Pb;

            use super::pp::MantleId as pb_MantleId;
            use super::pp::MantleType as pb_MantleType;

            #[derive(Pb)]
            #[pb(pb_name = "pb_MantleId")]
            pub struct MantleId {
                pub place_id: String,
                pub mantle_type: MantleType,
                pub mineral_type: i32,
                pub rift_name: String,
            }

            #[derive(Pb)]
            #[pb(pb_name = "pb_MantleType")]
            #[repr(i32)]
            pub enum MantleType {
                Unknown = 0,
                Sand = 1,
                Stone = 2,
            }
            impl MantleType {
                /// String value of the enum field names used in the ProtoBuf definition.
                ///
                /// The values are not transformed in any way and thus are considered stable
                /// (if the ProtoBuf definition does not change) and safe for programmatic use.
                pub fn as_str_name(&self) -> &'static str {
                    match self {
                        MantleType::Unknown => "MANTLE_TYPE_UNKNOWN",
                        MantleType::Sand => "MANTLE_TYPE_SAND",
                        MantleType::Stone => "MANTLE_TYPE_STONE",
                    }
                }
                /// Creates an enum from field names used in the ProtoBuf definition.
                pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                    match value {
                        "MANTLE_TYPE_UNKNOWN" => Some(Self::Unknown),
                        "MANTLE_TYPE_SAND" => Some(Self::Sand),
                        "MANTLE_TYPE_STONE" => Some(Self::Stone),
                        _ => None,
                    }
                }
            }

            #[repr(i32)]
            pub enum MineralType {
                Unknown = 0,
                Obj = 1,
                Email = 2,
                Sheet = 3,
                Storage = 4,
                Pc = 5,
                Calc = 6,
            }
            impl MineralType {
                /// String value of the enum field names used in the ProtoBuf definition.
                ///
                /// The values are not transformed in any way and thus are considered stable
                /// (if the ProtoBuf definition does not change) and safe for programmatic use.
                pub fn as_str_name(&self) -> &'static str {
                    match self {
                        MineralType::Unknown => "MINERAL_TYPE_UNKNOWN",
                        MineralType::Obj => "MINERAL_TYPE_OBJ",
                        MineralType::Email => "MINERAL_TYPE_EMAIL",
                        MineralType::Sheet => "MINERAL_TYPE_SHEET",
                        MineralType::Storage => "MINERAL_TYPE_STORAGE",
                        MineralType::Pc => "MINERAL_TYPE_PC",
                        MineralType::Calc => "MINERAL_TYPE_CALC",
                    }
                }
                /// Creates an enum from field names used in the ProtoBuf definition.
                pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
                    match value {
                        "MINERAL_TYPE_UNKNOWN" => Some(Self::Unknown),
                        "MINERAL_TYPE_OBJ" => Some(Self::Obj),
                        "MINERAL_TYPE_EMAIL" => Some(Self::Email),
                        "MINERAL_TYPE_SHEET" => Some(Self::Sheet),
                        "MINERAL_TYPE_STORAGE" => Some(Self::Storage),
                        "MINERAL_TYPE_PC" => Some(Self::Pc),
                        "MINERAL_TYPE_CALC" => Some(Self::Calc),
                        _ => None,
                    }
                }
            }

            pub struct MagnaInfo {
                pub mantle_id: ::core::option::Option<MantleId>,

                pub target_types: Vec<String>,

                pub result_types: Vec<String>,
            }
            /// rift情報
            ///
            pub struct RiftInfo {
                pub mantle_id: ::core::option::Option<MantleId>,

                pub target_types: Vec<String>,

                pub result_types: Vec<String>,
            }
        }
    }
}

fn main() {}
