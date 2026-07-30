# Auditoria de seguridad para GitHub

Audita archivos y workflows para detectar secretos y configuraciones peligrosas.

Esta rama contiene una implementacion y su evidencia empirica. Los resultados describen lo que se probo realmente; no afirman que el sistema sea correcto para entradas que no fueron ejercitadas.

## Comando

```powershell
sabiparche github integration-security-audit .\entrada-seguridad.json
```

El comando usa el binario instalado sabiparche. Los detalles de compilacion pertenecen al desarrollo interno y no forman parte del uso publico.

## Ejemplo de entrada

```json
{
  "paths": [
    ".\src",
    ".\config"
  ],
  "workflow_files": [
    ".\.github\workflows\ci.yml"
  ],
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

- Detecta tokens GitHub en texto, comentarios y JSON.
- Redacta el valor sensible en la evidencia.
- Detecta Actions no fijadas a un commit.
- Detecta permisos globales de escritura y pull_request_target prohibido.

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