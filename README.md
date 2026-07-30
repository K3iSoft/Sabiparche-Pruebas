# Evidencia empírica de Sabiparche

Esta rama muestra cómo se comporta Sabiparche mediante pruebas de caja negra. Se publican las entradas de prueba, el código del proyecto de demostración, los resultados observados y los efectos visibles en GitHub. No se publica el código fuente de Sabiparche ni información suficiente para reconstruir su implementación interna.

## Qué se demuestra

- Compilación y verificación de afirmaciones causales.
- Rechazo de afirmaciones inválidas o manipuladas.
- Pruebas de mutación, contrafactuales, oráculos y mínimos alternativos.
- Aceptación de cambios que satisfacen las comprobaciones declaradas.
- Rechazo de cambios incorrectos sin alterar main.
- Idempotencia, rollback y ausencia de commits vacíos.
- Observación independiente de ramas, commits y ejecuciones de GitHub Actions.

## Método público

1. Se parte de código de demostración visible.
2. Se entrega a Sabiparche una entrada o parche público.
3. Se observan compilación, pruebas, mutaciones y efectos remotos.
4. Se compara el commit de main antes y después de los casos negativos.
5. Se publica únicamente el resultado funcional y verificable.

## Resultado de esta ejecución

- Run: $runId
- PASS: $pass
- FAIL: $fail
- BLOCKED: $blocked
- main antes de las pruebas negativas: $remoteHeadBefore
- main después de las pruebas negativas: $remoteHeadAfterTests
- main permaneció sin cambios: $negativeRemoteStable

## Código público observado

### src/main.rs

`ust
pub fn mensaje() -> &'static str {
    "Sabiparche: proyecto público de pruebas operativo"
}

fn main() {
    println!("{}", mensaje());
}

#[cfg(test)]
mod tests {
    use super::mensaje;

    #[test]
    fn devuelve_el_mensaje_oficial() {
        assert_eq!(
            mensaje(),
            "Sabiparche: proyecto público de pruebas operativo"
        );
    }
}

`

### 	ests/ejecucion.rs

`ust
use std::process::Command;

#[test]
fn el_binario_se_ejecuta_y_muestra_el_mensaje() {
    let executable = env!("CARGO_BIN_EXE_saludo-sabiparche");

    let output = Command::new(executable)
        .output()
        .expect("el binario debe poder ejecutarse");

    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).expect("la salida debe ser UTF-8");

    assert_eq!(
        stdout.trim(),
        "Sabiparche: proyecto público de pruebas operativo"
    );
}

`

## Qué no contiene esta rama

No contiene fuentes de Sabiparche, nombres de funciones internas, trazas privadas, rutas del repositorio privado, algoritmos de decisión, ni parches usados para corregir el motor. Un FAIL describe el comportamiento externo observado, no la causa interna.

El detalle estructurado está en informe-publico.json.
