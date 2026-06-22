use std::fs;
use std::io::Write;

fn main() {
    // Force use custom linker script
    println!("cargo:rustc-link-arg=-Tlinker.ld");
    println!("cargo:rerun-if-changed=linker.ld");

    // Path to user apps
    const TARGET_PATH: &str = "../user/target/riscv64gc-unknown-none-elf/release";
    println!("cargo:rerun-if-changed={}", TARGET_PATH);
    println!("cargo:rerun-if-changed=../user/src/");

    // Create link_app.S to embed user applications
    let mut file = fs::File::create("src/link_app.S").unwrap();

    // Collect app filenames
    let mut apps = Vec::new();
    for entry in fs::read_dir("../user/src/bin").unwrap() {
        let entry = entry.unwrap();
        let fname = entry.file_name().into_string().unwrap();
        // Remove file extension
        if let Some(dot) = fname.find('.') {
            apps.push(fname[..dot].to_string());
        }
    }
    apps.sort();
    let app_count = apps.len();

    // Write assembly data section
    writeln!(file, ".align 3").unwrap();
    writeln!(file, ".section .data").unwrap();
    writeln!(file, ".global _num_app").unwrap();
    writeln!(file, "_num_app:").unwrap();
    writeln!(file, "    .quad {}", app_count).unwrap();

    // Write app start pointers
    for i in 0..app_count {
        writeln!(file, "    .quad app_{}_start", i).unwrap();
    }
    // Write last app end pointer
    writeln!(file, "    .quad app_{}_end", app_count - 1).unwrap();

    // Embed each app binary
    for (idx, name) in apps.iter().enumerate() {
        writeln!(file).unwrap();
        writeln!(file, ".section .data").unwrap();
        writeln!(file, ".global app_{}_start", idx).unwrap();
        writeln!(file, ".global app_{}_end", idx).unwrap();
        writeln!(file, ".align 3").unwrap();
        writeln!(file, "app_{}_start:", idx).unwrap();
        writeln!(file, "    .incbin \"{}/{}\"", TARGET_PATH, name).unwrap();
        writeln!(file, "app_{}_end:", idx).unwrap();
    }
}
