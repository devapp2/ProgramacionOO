# Sistema de Información Estatutario — Universidad Distrital Francisco José de Caldas

**Proyecto de caso de estudio** del libro:

> *Programación Orientada a Objetos: Diseño e Implementación de un Sistema de
> Información Académico-Administrativo para la Universidad Distrital Francisco
> José de Caldas*
>
> **Autor:** Roberto Albeiro Pava-Díaz, Ph.D.
> **ORCID:** [0000-0003-0440-892X](https://orcid.org/0000-0003-0440-892X)
> **Bogotá, 2026**

---

## Descripción

Sistema Java 17 + JavaFX 21 que implementa el dominio académico-administrativo
definido por el Acuerdo 004 de 2025 (Estatuto General) y el Estatuto Académico
de la Universidad Distrital. Modela Facultades, Escuelas, Centros, Institutos,
Docentes, Estudiantes, Programas Académicos y el sistema de evaluación formativa
(Rúbricas, Evidencias, Resultados de Aprendizaje).

---

## Requisitos

| Herramienta | Versión mínima |
|---|---|
| Java JDK | 17 (LTS) |
| Apache Maven | 3.8+ |
| JavaFX SDK | 21.0.2 (descargado por Maven) |
| Visual Studio Code | 1.88+ |
| Extension Pack for Java (VSCode) | Última |

---

## Estructura del proyecto

```
proyecto_java_estatuto/
├── .vscode/
│   ├── settings.json         # Configuración del proyecto para VSCode
│   └── launch.json           # Configuraciones de ejecución
├── pom.xml                   # Descriptores Maven (deps JavaFX 21, Gson, JUnit 5)
├── README.md                 # Este archivo
└── src/
    └── main/
        └── java/
            └── co/edu/udistrital/estatuto/
                ├── App.java                        # Punto de entrada
                ├── modelo/                         # Clases del dominio
                │   ├── UnidadAcademica.java        # Clase abstracta raíz
                │   ├── Facultad.java
                │   ├── Escuela.java
                │   ├── Centro.java
                │   ├── Instituto.java
                │   ├── Caba.java
                │   ├── Persona.java                # Clase abstracta raíz actores
                │   ├── TipoVinculacion.java        # Enum
                │   ├── Docente.java
                │   ├── EstadoEstudiante.java       # Enum
                │   ├── Estudiante.java
                │   ├── PersonalAdministrativo.java
                │   ├── Egresado.java
                │   ├── NivelPrograma.java          # Enum
                │   ├── ProgramaAcademico.java
                │   ├── PlanDeEstudios.java
                │   ├── EspacioAcademico.java
                │   ├── Matricula.java
                │   ├── ResultadoDeAprendizaje.java
                │   ├── Rubrica.java
                │   ├── CriterioRubrica.java
                │   ├── Evidencia.java
                │   └── PropositoFormacion.java
                ├── controlador/                    # Lógica de negocio (MVC)
                │   ├── FacultadControlador.java
                │   ├── DocenteControlador.java
                │   ├── EstudianteControlador.java
                │   └── ProgramaControlador.java
                ├── vista/                          # Interfaz JavaFX (MVC)
                │   └── VistaPrincipal.java
                ├── persistencia/                   # Acceso a datos (DAO)
                │   ├── GenericDAO.java             # Interface CRUD genérica
                │   └── JsonDAO.java                # Implementación con Gson
                ├── patron/                         # Patrones de diseño
                │   ├── UnidadAcademicaFactory.java # Factory Method
                │   ├── EvaluacionStrategy.java     # Strategy
                │   └── ConsejoObserver.java        # Observer
                ├── excepcion/                      # Jerarquía de excepciones
                │   ├── EstatutoException.java      # Excepción raíz
                │   └── MatriculaException.java
                └── util/                           # Utilidades transversales
                    └── Validador.java
```

---

## Compilación y ejecución

### Con Maven (recomendado)

```bash
# Compilar
mvn clean compile

# Ejecutar pruebas
mvn test

# Ejecutar la aplicación JavaFX
mvn javafx:run

# Generar Javadoc
mvn javadoc:javadoc
# La documentación queda en: target/site/apidocs/index.html

# Empaquetar como JAR
mvn package
```

### Con Visual Studio Code

1. Abrir la carpeta `proyecto_java_estatuto/` en VSCode.
2. Instalar el **Extension Pack for Java** si no está instalado.
3. VSCode detectará automáticamente el proyecto Maven.
4. Para ejecutar: usar la configuración **"Sistema Estatutario UD"** en el panel de *Run and Debug*.

> **Nota sobre JavaFX:** Si JavaFX no se descarga automáticamente, descargar
> el SDK desde [gluonhq.com/products/javafx/](https://gluonhq.com/products/javafx/)
> y configurar `PATH_TO_FX` en la variable de entorno del sistema.
> La forma más sencilla es usar exclusivamente `mvn javafx:run`.

---

## Datos de ejemplo

El sistema crea automáticamente el directorio `datos/` en la raíz del proyecto
y persiste los objetos en archivos JSON:

| Archivo | Contenido |
|---|---|
| `datos/facultades.json` | Lista de Facultades |
| `datos/docentes.json` | Lista de Docentes |
| `datos/estudiantes.json` | Lista de Estudiantes |
| `datos/matriculas.json` | Matrículas registradas |
| `datos/programas.json` | Programas Académicos |

---

## Patrones de diseño implementados

| Patrón | Clase | Descripción |
|---|---|---|
| Factory Method | `UnidadAcademicaFactory` | Crea Facultades, Escuelas, Centros e Institutos sin requerir constructores directos |
| Strategy | `EvaluacionStrategy` | Algoritmos intercambiables para el cálculo de nota final |
| Observer | `ConsejoObserver` | Notificación automática de decisiones del Consejo a observadores suscritos |

---

## Tecnologías utilizadas

- **Java 17** (LTS): lenguaje principal
- **JavaFX 21** (LTS): interfaz gráfica de escritorio
- **Gson 2.10**: serialización/deserialización JSON
- **JUnit 5.10**: pruebas unitarias
- **Maven 3.8+**: gestión de dependencias y construcción

---

## Licencia

Proyecto de uso académico. Todos los derechos reservados.
© Roberto Albeiro Pava-Díaz, Ph.D. — Universidad Distrital Francisco José de Caldas, 2026.
