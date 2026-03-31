package co.edu.udistrital.estatuto.modelo;

/**
 * Nivel de formación de un programa académico en la Universidad Distrital.
 *
 * @author Roberto Albeiro Pava-Díaz
 * @version 1.0
 */
public enum NivelPrograma {

    /** Programa de pregrado (tecnología o carrera universitaria). */
    PREGRADO("Pregrado"),

    /** Programa de especialización (posgrado de primer nivel). */
    ESPECIALIZACION("Especialización"),

    /** Programa de maestría. */
    MAESTRIA("Maestría"),

    /** Programa de doctorado. */
    DOCTORADO("Doctorado");

    /** Descripción legible para interfaces de usuario. */
    private final String descripcion;

    NivelPrograma(String descripcion) {
        this.descripcion = descripcion;
    }

    /**
     * Retorna la descripción del nivel.
     * @return descripción del nivel de formación
     */
    public String getDescripcion() {
        return descripcion;
    }

    @Override
    public String toString() {
        return descripcion;
    }
}
