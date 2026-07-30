# Evidencias para release

Prepara un plan de release con certificados, manifiestos y evidencias enlazadas por hash.

Esta rama contiene una implementacion y su evidencia empirica. Los resultados describen lo que se probo realmente; no afirman que el sistema sea correcto para entradas que no fueron ejercitadas.

## Comando

```powershell
sabiparche github integration-release-plan .\entrada-release.json
```

El comando usa el binario instalado sabiparche. Los detalles de compilacion pertenecen al desarrollo interno y no forman parte del uso publico.

## Ejemplo de entrada

```json
{
  "tag": "v1.0.0",
  "release_commit": "COMMIT_DEL_RELEASE",
  "certificate_paths": [
    ".\evidencias\certificate.json"
  ],
  "manifest_paths": [
    ".\evidencias\manifest.json"
  ],
  "heavy_evidence_locations": [
    "sha256:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa@https://example.invalid/evidence"
  ]
}
```

Sustituye los valores de ejemplo por rutas y commits reales del repositorio que quieras verificar.

## Que comprueba

- Calcula SHA-256 de certificados y manifiestos.
- Rechaza archivos ausentes o repetidos.
- Rechaza certificados vacios, JSON invalido y campos obligatorios ausentes.
- Permite campos adicionales cuando el certificado minimo sigue siendo valido.

## Historial de pruebas

- 15 pruebas iniciales: 14 correctas y 1 hueco de validacion semantica del certificado.
- Ahora se valida que cada certificado sea JSON, tenga operation_id y contenga un result valido o identidad de evidencia.
- 10 repruebas posteriores: 10 correctas.

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