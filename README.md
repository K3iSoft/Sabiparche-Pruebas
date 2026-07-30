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

# La rama remota cambió durante `github sync`

Esta es la reproducción válida del escenario de concurrencia.

## Preparación

Se creó una rama temporal y se publicó una primera mutación. Después se inició una sincronización de Sabiparche sobre esa cabeza.

El comando utilizado fue:

```powershell
sabiparche github sync `
    --root <clon-local> `
    --base auditoria-real/revalidacion-final-20260730-203042 `
    --validate '.\validacion-lenta.cmd'
```

En la rama se incluyó esta validación auxiliar para abrir una ventana temporal:

```batch
@ping -n 16 127.0.0.1 >nul
```

La salida conservada de la prueba no certifica por separado cada detalle interno de esa validación. Lo que sí quedó demostrado es que GitHub avanzó desde el commit inicial hasta un segundo commit mientras la operación estaba en curso.

## El cambio competidor

Otro actor publicó una segunda mutación en la misma rama. A partir de ese momento, la cabeza utilizada inicialmente por Sabiparche ya no era la vigente.

Una integración preparada sobre el primer commit no podía considerarse válida sin volver a comprobar el estado remoto.

## Respuesta de Sabiparche

Sabiparche consultó de nuevo la rama, detectó que la cabeza encontrada era distinta de la esperada y rechazó la sincronización.

El repositorio local no terminó integrado sobre la cabeza obsoleta.

## Qué demuestra

- La base remota se comprueba antes de confirmar la operación.
- Una mutación concurrente invalida la integración preparada sobre la cabeza anterior.
- La operación devuelve error en lugar de declarar éxito obsoleto.
- El estado local se conserva.

## Conclusión

Prueba empírica superada: `github sync` detecta y rechaza un cambio concurrente de la cabeza remota.

<!-- SABIPARCHE-AUDITORIA:FIN -->

