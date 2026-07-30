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

# Dos actores intentando actualizar la misma rama

Esta rama conserva una prueba sencilla de concurrencia realizada directamente con Git y GitHub.

## Qué se hizo

Se prepararon dos clones desde la misma cabeza. Cada uno creó un commit diferente y ambos intentaron publicarlo en la misma rama temporal.

```powershell
git clone <repositorio-github> <clon-a>
git clone <repositorio-github> <clon-b>

git -C <clon-a> push origin HEAD:<rama-temporal>
git -C <clon-b> push origin HEAD:<rama-temporal>
```

## Qué ocurrió

El primer push fue aceptado. El segundo quedó basado en una cabeza que ya no era la vigente y GitHub lo rechazó.

Los dos commits pueden entenderse como mutaciones distintas producidas desde el mismo estado inicial.

## Conclusión

GitHub conservó el commit aceptado y no permitió que el segundo actor sobrescribiera la rama con una actualización obsoleta.

Esta prueba describe el comportamiento de Git y GitHub. No evalúa por sí sola una operación de Sabiparche.

<!-- SABIPARCHE-AUDITORIA:FIN -->

