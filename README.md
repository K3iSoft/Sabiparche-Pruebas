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

# Timeline público de la auditoría de Sabiparche

## Objetivo

Mantener una referencia visible en GitHub sobre las pruebas empíricas sin publicar secretos, rutas locales ni salidas internas completas.

## Comandos públicos documentados

### Sincronización transaccional

```powershell
sabiparche github sync `
    --root <clon-local> `
    --base <rama-remota> `
    --validate <orden-de-validacion>
```

### Aplicación de afirmaciones

```powershell
sabiparche apply <archivo-de-afirmaciones>
```

Los parámetros concretos dependen del contrato declarado por el archivo de afirmaciones.

## Qué representa una mutación

Una mutación es una modificación deliberada del estado del repositorio utilizada para comprobar si una afirmación sigue siendo válida.

Puede materializarse mediante:

- creación o modificación de archivos;
- cambio de un commit remoto;
- avance concurrente de una rama;
- alteración de una condición que debía permanecer estable.

## Qué significa compilar afirmaciones

Sabiparche interpreta las afirmaciones declaradas, determina sus dependencias y prepara una operación verificable.

La compilación de afirmaciones no equivale únicamente a compilar código fuente. Consiste en convertir declaraciones y condiciones en un plan comprobable.

## Validaciones

Las validaciones son órdenes declaradas que deben terminar correctamente antes de que una integración pueda confirmarse.

Ejemplo:

```powershell
sabiparche github sync `
    --root <clon-local> `
    --base <rama-remota> `
    --validate <comprobacion-1> `
    --validate <comprobacion-2>
```

## Certificados

Un certificado conserva evidencia estructurada sobre lo que se observó y comprobó durante una operación.

Puede incluir conceptos como:

- base utilizada;
- afirmaciones evaluadas;
- validaciones ejecutadas;
- resultado de la operación;
- integridad o rollback verificados.

No se publican aquí los certificados completos ni sus salidas internas.

## Integración transaccional

Sabiparche prepara y valida los cambios antes de modificar el repositorio local definitivo.

Una operación no debe confirmarse cuando:

- falla una validación;
- cambia la cabeza remota;
- se incumple una condición declarada;
- no puede verificarse la integridad del resultado.

## Evidencia visible en GitHub

El historial remoto permite observar:

- creación de ramas temporales;
- commits que representan mutaciones;
- avances competidores;
- pushes aceptados y rechazados;
- secuencia temporal de los escenarios.

## Evidencia no publicada

No se incluyen:

- rutas locales;
- tokens;
- secretos;
- URL privadas;
- stdout o stderr completos;
- JSON internos;
- detalles privados de implementación.

## Resultados confirmados

- GitHub rechaza un push basado en una cabeza obsoleta.
- `sabiparche github sync` opera contra un repositorio real.
- Sabiparche rechaza validaciones fallidas.
- Sabiparche verifica rollback local ante fallos observados.
- Sabiparche detecta cambios de cabeza remota durante una operación.
- Sabiparche evita confirmar una integración basada en una cabeza obsoleta.

## Pendiente de prueba empírica

- HTTP 429 real de GitHub.
- HTTP 5xx real de GitHub.
- Cambio real de ruleset durante una operación.
- Merge Queue y eventos `merge_group`.
- Entrega real de webhook desde GitHub.

## Estado

**TIMELINE DE AUDITORÍA ACTIVO**

<!-- SABIPARCHE-AUDITORIA:FIN -->

