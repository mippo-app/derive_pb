use derive_pb::Pb;

#[test]
fn test_macro() {
    // cargo test --test aaa -- test_macro --nocapture

    pub struct pb_MantleType {
        pub valueof: Option<pb_Valueof>,
    }

    /// Nested message and enum types in `MInfoType`.
    pub enum pb_Valueof {
        Sand(Sand),
        Stone(Stone),
    }

    #[derive(Pb)]
    // #[pb(pb_name = "pb_Valueof")]
    #[pb(pb_name = "pb_MantleType")]
    pub enum MantleType {
        Sand(Sand),
        Stone(Stone),
    }

    struct pb_Sand {
        id: u64,
        name: String,
    }

    #[derive(Pb)]
    #[pb(pb_name = "pb_Sand")]
    struct Sand {
        id: u64,
        name: String,
    }

    struct pb_Stone {
        id: u64,
        name: String,
    }

    #[derive(Pb)]
    #[pb(pb_name = "pb_Stone")]
    struct Stone {
        id: u64,
        name: String,
    }

    // assert_eq!(Hoge::hello_macro(), "A".to_string());
    // cargo expand --test aaa -- test_macro
}
