# Cubo en raytracer (Rust)

Demostración de ray tracing de una mena de diamante estilo Minecraft sobre un suelo, con iluminación **exclusivamente difusa** (Lambert). Cada cara utiliza una textura pixel-art procedural de 16×16: piedra gris irregular con vetas de mineral cian dispersas. Las vetas tienen un mapa de altura para producir relieve mediante *bump mapping*.

Incluye una luz puntual con atenuación por distancia, sombras duras proyectadas sobre el suelo y corrección gamma; no usa componente especular ni luz ambiente. La textura se genera completamente en Rust y no necesita archivos de imagen externos.

La cámara se calcula una sola vez por fotograma y los rayos primarios no se normalizan (no es necesario para estas intersecciones), reduciendo el trabajo de renderizado por píxel.

## Organización

- `main.rs`: ventana, controles y ciclo principal.
- `math.rs` y `ray.rs`: vectores y rayos.
- `camera.rs`: cámara orbital.
- `geometry.rs`: intersecciones con el cubo y el suelo.
- `texture.rs`: textura pixel-art y relieve de la mena.
- `lighting.rs`: iluminación Lambert y sombras.
- `scene.rs`: objetos y luz de la escena.
- `renderer.rs`: trazado y conversión final de color.

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
