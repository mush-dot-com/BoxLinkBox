fn main() {
    println!("cargo:rerun-if-changed=assets/app.rc");
    println!("cargo:rerun-if-changed=assets/app.manifest");
    println!("cargo:rerun-if-changed=assets/icon.ico");
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("windows") {
        let out = std::path::PathBuf::from(std::env::var_os("OUT_DIR").unwrap_or_default())
            .join("app.res");
        let status = std::process::Command::new("rc.exe")
            .arg("/nologo")
            .arg("/fo")
            .arg(&out)
            .arg("assets/app.rc")
            .status();
        match status {
            Ok(status) if status.success() => println!("cargo:rustc-link-arg={}", out.display()),
            _ => panic!(
                "Windowsリソースを作成できません。Windows SDKのあるDeveloper PowerShellでビルドしてください。"
            ),
        }
    }
}
