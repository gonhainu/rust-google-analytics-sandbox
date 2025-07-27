use std::env;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR")?;
    let googleapis_path = PathBuf::from(&manifest_dir).join("googleapis");

    println!("cargo:warning=Manifest dir: {}", manifest_dir);
    println!(
        "cargo:warning=googleapis path: {}",
        googleapis_path.display()
    );

    // `src/generated` ディレクトリを作成
    let out_dir = PathBuf::from(&manifest_dir).join("src").join("generated");
    std::fs::create_dir_all(&out_dir)?;

    tonic_build::configure()
        .build_server(false)
        .out_dir(&out_dir)
        .compile_protos(
            &[
                "google/analytics/data/v1beta/analytics_data_api.proto",
                "google/analytics/data/v1beta/data.proto",
                "google/longrunning/operations.proto",
                "google/rpc/status.proto",
                "google/api/annotations.proto",
                "google/api/client.proto",
                "google/api/field_behavior.proto",
                "google/api/http.proto",
                "google/api/resource.proto",
            ],
            &[googleapis_path.to_str().unwrap()],
        )?;

    Ok(())
}
