fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Generating proto code...");

    // パスを定義
    let backend_dir = concat!(env!("CARGO_MANIFEST_DIR"), "/..");
    let bff_services_dir = concat!(env!("CARGO_MANIFEST_DIR"), "/../bff/src/services");

    // 各サービスの設定 (service_name, proto_file_name)
    let services = vec![
        ("authservice", "auth"),
        // ("gameservice", "game"),
        // ("profileservice", "profile"),
    ];

    // 並列処理でクライアントとサーバーコードを同時生成
    std::thread::scope(|s| {
        for (service_name, proto_name) in &services {
            let service_name = *service_name;
            let proto_name = *proto_name;

            // クライアント用コード生成（BFF向け）
            s.spawn(move || {
                let proto_file = format!("{backend_dir}/{service_name}/proto/{proto_name}.proto");
                let include_path = format!("{backend_dir}/{service_name}/proto");
                let output_path = std::path::PathBuf::from(bff_services_dir);

                std::fs::create_dir_all(&output_path).unwrap();

                println!("✓ Generating client for {} -> {}", service_name, output_path.display());

                let result = tonic_prost_build::configure()
                    .build_client(true)
                    .build_server(false)
                    .out_dir(&output_path)
                    .compile_protos(&[&proto_file], &[&include_path]);

                if let Err(e) = result {
                    eprintln!("Failed to generate client for {service_name}: {e}");
                }
            });

            // サーバー用コード生成（各マイクロサービス向け）
            s.spawn(move || {
                let proto_file = format!("{backend_dir}/{service_name}/proto/{proto_name}.proto");
                let include_path = format!("{backend_dir}/{service_name}/proto");
                let output_path =
                    std::path::PathBuf::from(format!("{backend_dir}/{service_name}/src/proto"));

                std::fs::create_dir_all(&output_path).unwrap();

                println!("✓ Generating server for {} -> {}", service_name, output_path.display());

                let result = tonic_prost_build::configure()
                    .build_client(false)
                    .build_server(true)
                    .out_dir(&output_path)
                    .compile_protos(&[&proto_file], &[&include_path]);

                if let Err(e) = result {
                    eprintln!("Failed to generate server for {service_name}: {e}");
                }
            });
        }
    });

    println!("Proto code generation completed!");
    Ok(())
}
