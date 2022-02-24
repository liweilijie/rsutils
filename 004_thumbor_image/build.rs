use std::process::Command;

fn main() {
    // option_env! 如果在编译时存在指定的环境变量，则它将扩展为 Option<&'static str> 类型的表达式，
    // 其值是环境变量的 Some 值。如果不存在环境变量，那么它将扩展为 None 。
    let print_build: Option<&'static str> = option_env!("BUILD_PROTO");
    println!("the build proto is {:?}", print_build);

    let build_enabled = option_env!("BUILD_PROTO")
        .map(|v| v == "1")
        .unwrap_or(false);

    if !build_enabled {
        println!("=== Skipped compiling protos ===");
        return;
    }

    prost_build::Config::new()
        .out_dir("src/pb")
        .compile_protos(&["abi.proto"], &["."])
        .unwrap();
    Command::new("cargo")
        .args(&["fmt", "--", "src/*.rs"])
        .status()
        .expect("cargo fmt failed");
}
