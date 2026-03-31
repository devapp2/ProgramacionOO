package co.edu.udistrital.estatuto.modelo;

/**
 * Estado académico del estudiante a lo largo de su ciclo de vida
 * en la Universidad Distrital Francisco José de Caldas.
 *
 * @author Roberto Albeiro Pava-Díaz
 * @version 1.0
 */
public enum EstadoEstudiante {

    /** Aspirante que ha presentado solicitud de admisión. */
    ASPIRANTE("Aspirante"),

    /** Estudiante admitido pendiente de formalizar matrícula. */
    ADMITIDO("Admitido"),

    /** Estudiante con matrícula vigente en el período en curso. */
    ACTIVO("Activo"),

    /** Estudiante con matrícula suspendida por causas académicas o disciplinarias. */
    SUSPENDIDO("Suspendido"),

    /** Estudiante que completó todos los requisitos del programa. */
    GRADUADO("Graduado"),

    /** Estudiante retirado voluntaria o involuntariamente del programa. */
    RETIRADO("Retirado");

    /** Descripción legible para interfaces de usuario. */
    private final String descripcion;

    EstadoEstudiante(String descripcion) {
        this.descripcion = descripcion;
    }

    /**
     * Retorna la descripción del estado.
     * @return descripción del estado
     */
    public String getDescripcion() {
        return descripcion;
    }

    @Override
    public String toString() {
        return descripcion;
    }
}
