# Integracion sintetica de pull requests

Construye una integracion Git aislada para comprobar compatibilidad entre dos commits.

Esta rama contiene una implementacion y su evidencia empirica. Los resultados describen lo que se probo realmente; no afirman que el sistema sea correcto para entradas que no fueron ejercitadas.

## Comando

```powershell
sabiparche github integration-pr-synthetic .\entrada-integracion.json
```

El comando usa el binario instalado sabiparche. Los detalles de compilacion pertenecen al desarrollo interno y no forman parte del uso publico.

## Ejemplo de entrada

```json
{
  "repository_root": ".",
  "base_sha": "COMMIT_BASE",
  "head_sha": "COMMIT_HEAD",
  "verification_document": ".\verificacion.toml"
}
```

Sustituye los valores de ejemplo por rutas y commits reales del repositorio que quieras verificar.

## Que comprueba

- Detecta conflictos de contenido.
- Detecta conflictos de borrado contra edicion.
- Rechaza commits inexistentes o ajenos al repositorio.
- No modifica el checkout fuente usado para la prueba.

## Historial de pruebas

- 15 pruebas iniciales: 15 correctas.
- No fue necesario corregir huecos en esta implementacion durante esta tanda.
- La bateria inicial quedo completamente verde.

La evidencia detallada se conserva dentro de pruebas-github/ en esta misma rama. Las pruebas posteriores no borran los resultados anteriores: quedan en el historial para mostrar el descubrimiento, la correccion y la reprueba.

## Interpretacion del resultado

Un resultado aceptado significa que la entrada cumplio las comprobaciones implementadas para esta integracion. Un rechazo devuelve un codigo distinto de cero y un mensaje que identifica el dato invalido o la evidencia insuficiente.

## Limites

- Las pruebas cubren los casos publicados en esta rama.
- No sustituyen una auditoria completa de GitHub, Git ni del repositorio objetivo.
- Las rutas y credenciales reales no deben incluirse en evidencias publicas.
- No se deben publicar tokens, claves ni datos locales innecesarios.

## Estructura de evidencia

```text
pruebas-github/
  resultados iniciales
  bateria-15/
  repruebas posteriores, cuando corresponda
```