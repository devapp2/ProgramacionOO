package co.edu.udistrital.estatuto.modelo;

/**
 * Tipos de vinculación docente con la Universidad Distrital
 * Francisco José de Caldas, conforme al artículo 8 del Acuerdo 004 de 2025.
 *
 * @author Roberto Albeiro Pava-Díaz
 * @version 1.0
 */
public enum TipoVinculacion {

    /** Docente de planta con nombramiento permanente mediante concurso. */
    PLANTA("Planta"),

    /** Docente ocasional vinculado por contrato a término definido. */
    OCASIONAL("Ocasional"),

    /** Docente vinculado por hora cátedra según plan de trabajo. */
    HORA_CATEDRA("Hora Cátedra"),

    /** Docente visitante proveniente de otra institución académica. */
    VISITANTE("Visitante"),

    /** Experto o experta invitada del sector productivo o profesional. */
    EXPERTO("Experto/a");

    /** Nombre descriptivo para presentación en interfaces de usuario. */
    private final String descripcion;

    TipoVinculacion(String descripcion) {
        this.descripcion = descripcion;
    }

    /**
     * Retorna la descripción legible del tipo de vinculación.
     * @return descripción del tipo
     */
    public String getDescripcion() {
        return descripcion;
    }

    @Override
    public String toString() {
        return descripcion;
    }
}
