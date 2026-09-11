# Cubo en raytracer (Rust)

Demostración mínima de ray tracing de un cubo con iluminación **exclusivamente difusa** (Lambert). El cubo se intersecta usando el método de *slabs* y no usa luz ambiente ni componente especular.

## Ejecutar

```powershell
cargo run --release
```

## Controles

- Flechas izquierda/derecha: rotación orbital horizontal.
- Flechas arriba/abajo: rotación orbital vertical.
- `W` / `S`: acercar / alejar la cámara.
- `R`: restaurar la vista inicial.
- `Esc`: cerrar.
