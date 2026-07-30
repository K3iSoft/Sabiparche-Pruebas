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

# Notas públicas de la auditoría

Esta rama sirve como punto de entrada para entender las pruebas realizadas con Sabiparche y GitHub.

No contiene secretos, rutas locales, diagnósticos completos ni certificados internos.

## Comando que sí se probó en esta auditoría

```powershell
sabiparche github sync `
    --root <clon-local> `
    --base <rama-remota> `
    --validate <orden-de-validacion>
```

El comando sincroniza un clon con una rama de GitHub, prepara la integración de forma aislada, ejecuta las validaciones declaradas y evita confirmar el resultado si la base remota deja de ser válida.

## Mutaciones y afirmaciones

En estas pruebas, una mutación es un cambio observable del repositorio: un archivo nuevo, un commit adicional o el avance de una rama por otro actor.

Sabiparche trabaja con afirmaciones sobre estados concretos. Antes de aceptar una operación debe comprobar que esas condiciones siguen siendo ciertas.

La compilación de afirmaciones consiste en transformar esas declaraciones y dependencias en un plan que pueda ejecutarse y verificarse. No debe confundirse únicamente con la compilación de código fuente.

## Validación y resultado transaccional

Las órdenes pasadas mediante `--validate` se utilizan para comprobar la integración preparada.

Cuando una validación falla, o cuando la cabeza remota cambia durante la operación, la integración no debe confirmarse sobre el repositorio local definitivo.

## Certificados

Sabiparche puede conservar evidencia estructurada de las condiciones observadas, las validaciones y el resultado. Los certificados y diagnósticos completos no se publican en esta rama.

## Otros comandos de Sabiparche

Sabiparche también dispone de operaciones como `sabiparche apply`, relacionadas con la aplicación de afirmaciones declaradas.

Ese comando se menciona aquí únicamente como contexto general del proyecto. No fue el objeto de las pruebas conservadas en estas ramas.

## Resultados que ya tienen evidencia

- GitHub rechaza un push directo basado en una cabeza obsoleta.
- `github sync` funciona contra un repositorio real.
- Una validación fallida provoca el rechazo de la operación.
- Se observó la conservación del estado local ante fallos.
- Sabiparche detecta que la cabeza remota ha cambiado.
- Una integración construida sobre una base obsoleta no se confirma.

## Escenarios que todavía faltan

- Una respuesta HTTP 429 real de GitHub.
- Un error HTTP 5xx real de GitHub.
- La modificación de un ruleset durante una operación.
- Una Merge Queue que produzca eventos `merge_group`.
- Una entrega real de webhook desde GitHub.

El historial de ramas y commits permite seguir la parte remota de cada experimento. Las conclusiones sobre la respuesta de Sabiparche proceden de las observaciones locales realizadas durante esas ejecuciones.

<!-- SABIPARCHE-AUDITORIA:FIN -->

