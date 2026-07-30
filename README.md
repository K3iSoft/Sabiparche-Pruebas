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

# Auditoría empírica: concurrencia de GitHub

## Objetivo

Comprobar cómo responde GitHub cuando dos actores intentan actualizar una rama partiendo de la misma cabeza.

## Escenario ejecutado

1. Se prepararon dos clones independientes.
2. Cada clon creó una mutación distinta.
3. El primer actor publicó su commit.
4. El segundo actor intentó publicar un commit basado en la cabeza anterior.
5. Se comprobó la cabeza final de la rama remota.

## Operaciones ejecutadas

```powershell
git clone <repositorio-github> <clon-a>
git clone <repositorio-github> <clon-b>

git -C <clon-a> push origin HEAD:<rama-temporal>
git -C <clon-b> push origin HEAD:<rama-temporal>
```

## Mutación

Cada commit representa una transformación distinta del mismo estado inicial.

## Concurrencia

Los dos actores trabajaron desde una base común. Después del primer push, el segundo commit quedó basado en una cabeza obsoleta.

## Resultado

GitHub aceptó el primer commit y rechazó el segundo push. La rama remota permaneció en el commit aceptado.

## Alcance

Esta rama prueba Git y GitHub. No atribuye el rechazo directamente a Sabiparche.

## Estado

**PRUEBA EMPÍRICA SUPERADA: GIT Y GITHUB**

<!-- SABIPARCHE-AUDITORIA:FIN -->

