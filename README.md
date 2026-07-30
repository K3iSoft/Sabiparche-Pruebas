# Workflow causal para GitHub

Genera y audita un workflow causal con eventos y permisos controlados.

Esta rama contiene una implementacion y su evidencia empirica. Los resultados describen lo que se probo realmente; no afirman que el sistema sea correcto para entradas que no fueron ejercitadas.

## Comando

```powershell
sabiparche github integration-workflow .\entrada-workflow.json
```

El comando usa el binario instalado sabiparche. Los detalles de compilacion pertenecen al desarrollo interno y no forman parte del uso publico.

## Ejemplo de entrada

```json
{
  "root": ".",
  "workflow": {
    "workflow_path": ".github/workflows/sabiparche-causal.yml",
    "action_ref": "K3iSoft/sabiparche-action@0123456789abcdef0123456789abcdef01234567",
    "events": ["pull_request", "merge_group", "workflow_dispatch"],
    "permissions": {
      "contents": "read",
      "checks": "write",
      "pull_requests": "write",
      "actions": "read",
      "attestations": null,
      "id_token": null
    },
    "patch_path": "parche.toml"
  },
  "audit_policy": {
    "require_merge_group": true,
    "forbid_pull_request_target": true,
    "require_full_history": true,
    "forbid_unpinned_external_actions": true,
    "forbid_global_write_all": true
  }
}
```

Sustituye los valores de ejemplo por rutas y commits reales del repositorio que quieras verificar.

## Que comprueba

- Rechaza referencias mutables de Actions.
- Rechaza pull_request_target cuando la politica lo prohibe.
- Rechaza permisos excesivos o desconocidos.
- Genera evidencia reproducible del workflow.

## Historial de pruebas

- 15 pruebas iniciales: 14 correctas y 1 hueco detectado en permisos.
- Se restringieron contents y actions a none/read, y se rechazan permisos desconocidos.
- 5 repruebas posteriores: 5 correctas.

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