# figuras/

Directorio para figuras del libro.

## Archivos requeridos

- `logo_ud.pdf` o `logo_ud.png` — Logotipo de la Universidad Distrital Francisco José de Caldas
  (debe obtenerse de la imagen oficial de la institución).
- Figuras generadas por PlantUML (se colocan aquí tras compilar los .puml).

## Generación de diagramas PlantUML

Para generar las imágenes a partir de los listados de código PlantUML incluidos en los .tex:

```bash
java -jar plantuml.jar -tpdf figuras/*.puml
```

O bien emplear el plugin PlantUML de IntelliJ IDEA / VS Code.
