use std::process::Command;

#[test]
fn el_binario_se_ejecuta_y_muestra_el_mensaje() {
    let executable = env!("CARGO_BIN_EXE_saludo-sabiparche");

    let output = Command::new(executable)
        .output()
        .expect("el binario debe poder ejecutarse");

    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout)
        .expect("la salida debe ser UTF-8");

    assert_eq!(
        stdout.trim(),
        "Sabiparche: proyecto público de pruebas operativo"
    );
}
