use std::process::Command;

fn main() {
    // Compilar el archivo de recursos usando `windres`
    let output = Command::new("windres")
        .args(&["resources/app.rc", "-o", "app.res"]) // Rutas correctas
        .output()
        .expect("Error al compilar el archivo de recursos.");

    if !output.status.success() {
        panic!(
            "Error al compilar el archivo de recursos: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    println!("cargo:rustc-link-arg=app.res");
}