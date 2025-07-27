pub mod generated {
    pub mod google {
        pub mod analytics {
            pub mod data {
                pub mod v1beta {
                    include!(concat!(
                        env!("CARGO_MANIFEST_DIR"),
                        "/src/generated/google.analytics.data.v1beta.rs"
                    ));
                }
            }
        }
        pub mod longrunning {
            include!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/src/generated/google.longrunning.rs"
            ));
        }
        pub mod rpc {
            include!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/src/generated/google.rpc.rs"
            ));
        }
        pub mod api {
            include!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/src/generated/google.api.rs"
            ));
        }
    }
}

pub use generated::google::analytics::data::v1beta::*;
pub use generated::google::longrunning::*;
