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

# Auditoría empírica: segundo ensayo de sincronización concurrente

## Objetivo

Repetir de forma independiente el escenario de cambio remoto durante una sincronización.

## Comando de Sabiparche ensayado

```powershell
sabiparche github sync `
    --root <clon-local> `
    --base auditoria-real/verificacion-sync-20260730-202219 `
    --validate <validacion-lenta>
```

## Conceptos previstos

- Mutación remota inicial.
- Compilación de una afirmación sobre una cabeza concreta.
- Validación de la integración provisional.
- Mutación competidora.
- Revalidación de la cabeza remota.
- Confirmación o rechazo transaccional.

## Problema de la prueba

La validación lenta no se ejecutó como estaba previsto. El intérprete fue iniciado sin conservar la orden completa.

## Resultado

La clasificación inicial fue un falso positivo causado por el montaje auxiliar.

## Estado

**PRUEBA INVÁLIDA**

No demuestra un defecto de Sabiparche.

<!-- SABIPARCHE-AUDITORIA:FIN -->

