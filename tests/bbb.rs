use derive_pb::Pb;

#[test]
fn test_macro() {
    // cargo test --test bbb -- test_macro --nocapture
    // cargo expand --test bbb -- test_macro

    pub enum pb_ValueOf {
        Int32Value(i32),
        Int64Value(i64),
        FloatValue(f32),
        DoubleValue(f64),
    }

    #[derive(Pb)]
    #[pb(pb_name = "pb_ValueOf")]
    pub enum ValueOf {
        Int32Value(i32),
        Int64Value(i64),
        FloatValue(f32),
        DoubleValue(f64),
    }

    // assert_eq!(Hoge::hello_macro(), "A".to_string());
}
