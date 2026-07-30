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

# Auditoría empírica: revalidación de la cabeza remota

## Objetivo

Comprobar que Sabiparche no confirma una sincronización cuando GitHub cambia durante la operación.

## Comando de Sabiparche probado

```powershell
sabiparche github sync `
    --root <clon-local> `
    --base auditoria-real/revalidacion-final-20260730-203042 `
    --validate '.\validacion-lenta.cmd'
```

## Validación auxiliar

```batch
@ping -n 16 127.0.0.1 >nul
```

La validación auxiliar mantuvo abierta la operación el tiempo suficiente para que otro actor avanzara la rama remota.

## Escenario ejecutado

1. Se creó una rama temporal desde una base conocida.
2. Se publicó una primera mutación remota.
3. Sabiparche inició `github sync`.
4. Sabiparche preparó una integración aislada.
5. Se ejecutó una validación lenta.
6. Durante la operación se publicó una segunda mutación.
7. Sabiparche volvió a consultar la cabeza remota.
8. La cabeza encontrada ya no coincidía con la usada inicialmente.

## Mutación

Los dos commits representan estados sucesivos del repositorio. La segunda mutación invalida una afirmación construida exclusivamente sobre la primera cabeza.

## Compilación de afirmaciones

Sabiparche prepara una afirmación integrada sobre una base concreta. Esa afirmación solo puede confirmarse si las validaciones son correctas y la base continúa vigente.

## Validación

Las órdenes declaradas mediante `--validate` se ejecutan sobre la integración aislada antes de modificar el repositorio local definitivo.

## Certificación

El rechazo constituye evidencia de que Sabiparche comparó la cabeza esperada con la cabeza encontrada antes de confirmar la operación.

## Integración transaccional

El repositorio local no fue actualizado con una integración basada en una cabeza remota obsoleta.

## Concurrencia

La segunda publicación simuló otro actor modificando GitHub mientras Sabiparche realizaba la operación.

## Resultado

Sabiparche detectó el cambio de cabeza remota, rechazó la sincronización, devolvió un código de error y conservó el estado local anterior.

## Estado

**PRUEBA EMPÍRICA SUPERADA: REVALIDACIÓN REMOTA**

<!-- SABIPARCHE-AUDITORIA:FIN -->

