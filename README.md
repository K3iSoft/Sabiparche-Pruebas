# Sabiparche Pruebas

Este es el repositorio oficial de K3iSoft para las pruebas públicas de Sabiparche.

## Qué es Sabiparche

Sabiparche es un sistema para aplicar, verificar y publicar cambios sobre proyectos de
software con evidencia comprobable.

Además de modificar archivos, Sabiparche puede:

- crear o verificar repositorios;
- crear carpetas, archivos, commits y ramas desde una especificación TOML;
- publicar cambios en GitHub;
- mantener journals autenticados;
- comprobar commits, árboles, certificados y manifiestos;
- detectar manipulaciones posteriores;
- reanudar operaciones interrumpidas;
- evitar duplicados mediante ejecución idempotente;
- usar la GitHub App instalada sin pedir tokens personales.

## Finalidad de este repositorio

Este repositorio no contiene el código de producción de K3iSoft. Se usa como entorno
público y reproducible para probar Sabiparche sobre un proyecto pequeño pero real.

Aquí se comprueba que Sabiparche puede:

1. crear o verificar un repositorio público;
2. generar un proyecto Rust funcional;
3. crear archivos y carpetas;
4. crear y publicar ramas;
5. repetir la operación sin duplicar recursos;
6. producir journals y hashes de evidencia;
7. autenticar mediante la GitHub App ya instalada.

## Proyecto de ejemplo

El proyecto incluido es una aplicación Rust mínima llamada `saludo-sabiparche`.

Comprobación local:

```text
cargo check
cargo test
cargo run
```

Salida esperada:

```text
Sabiparche: proyecto público de pruebas operativo
```

## Ramas de prueba

- `prueba/publicacion-nueva-ancla`
- `prueba/manipulacion-lineage`
- `prueba/reanudacion-interrumpida`
- `prueba/identidad-cruzada`
- `prueba/cambios-proyecto-real`

## Titularidad

K3iSoft mantiene este repositorio como entorno oficial de pruebas públicas de
Sabiparche.

<!-- SABIPARCHE-AUDITORIA:INICIO -->

# Primer intento de sincronización concurrente

La intención era comprobar qué hacía Sabiparche si la rama remota avanzaba mientras `github sync` seguía trabajando.

## Comando ensayado

```powershell
sabiparche github sync `
    --root <clon-local> `
    --base auditoria-real/sync-20260730-201658 `
    --validate <validacion-lenta>
```

Primero se publicó una mutación remota. Después se inició la sincronización y se intentó publicar otra mutación durante la fase de validación.

## Por qué no cuenta como prueba

La orden auxiliar no mantuvo abierta la validación como estaba previsto. El intérprete se inició, pero la espera completa no llegó a ejecutarse correctamente.

No se puede asegurar que el cambio remoto ocurriera mientras Sabiparche seguía dentro de la operación.

## Balance

Ensayo inválido. Se conserva para documentar el proceso, pero no demuestra un éxito ni un fallo del programa.

<!-- SABIPARCHE-AUDITORIA:FIN -->

