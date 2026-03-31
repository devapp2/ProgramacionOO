package co.edu.udistrital.estatuto.modelo;

import java.util.Objects;

/**
 * Programa académico ofrecido por la Universidad Distrital.
 *
 * <p>Cada programa posee un código SNIES, un nivel de formación,
 * una modalidad de oferta (presencial, virtual, distancia) y un
 * plan de estudios vigente. Los programas están adscritos a una
 * Facultad a través de las Áreas de Formación.</p>
 *
 * @author Roberto Albeiro Pava-Díaz
 * @version 1.0
 */
public class ProgramaAcademico {

    /** Nombre completo del programa. */
    private String nombre;

    /** Código SNIES u código institucional del programa. */
    private String codigo;

    /** Nivel de formación (pregrado, especialización, maestría, doctorado). */
    private NivelPrograma nivel;

    /** Modalidad de oferta (presencial, virtual, distancia). */
    private String modalidad;

    /** Código de la Facultad que ofrece el programa. */
    private String facultadCodigo;

    /** Plan de estudios vigente del programa. */
    private PlanDeEstudios planDeEstudios;

    /**
     * Construye un ProgramaAcademico sin plan de estudios inicial.
     *
     * @param nombre        nombre completo del programa (no nulo)
     * @param codigo        código SNIES o institucional (no nulo)
     * @param nivel         nivel de formación (no nulo)
     * @param modalidad     modalidad de oferta
     * @param facultadCodigo código de la facultad ofertante
     */
    public ProgramaAcademico(String nombre, String codigo,
                              NivelPrograma nivel, String modalidad,
                              String facultadCodigo) {
        this.nombre        = Objects.requireNonNull(nombre, "El nombre no puede ser nulo.");
        this.codigo        = Objects.requireNonNull(codigo, "El código no puede ser nulo.");
        this.nivel         = Objects.requireNonNull(nivel, "El nivel no puede ser nulo.");
        this.modalidad     = modalidad;
        this.facultadCodigo = facultadCodigo;
    }

    // ── Getters y Setters ──────────────────────────────────────────────────

    /** @return nombre del programa */
    public String getNombre() { return nombre; }

    /** @param nombre nuevo nombre */
    public void setNombre(String nombre) { this.nombre = nombre; }

    /** @return código SNIES o institucional */
    public String getCodigo() { return codigo; }

    /** @param codigo nuevo código */
    public void setCodigo(String codigo) { this.codigo = codigo; }

    /** @return nivel de formación */
    public NivelPrograma getNivel() { return nivel; }

    /** @param nivel nuevo nivel de formación */
    public void setNivel(NivelPrograma nivel) {
        this.nivel = Objects.requireNonNull(nivel);
    }

    /** @return modalidad de oferta */
    public String getModalidad() { return modalidad; }

    /** @param modalidad nueva modalidad */
    public void setModalidad(String modalidad) { this.modalidad = modalidad; }

    /** @return código de la facultad */
    public String getFacultadCodigo() { return facultadCodigo; }

    /** @param facultadCodigo nuevo código de facultad */
    public void setFacultadCodigo(String facultadCodigo) {
        this.facultadCodigo = facultadCodigo;
    }

    /** @return plan de estudios vigente (puede ser null si no se ha asignado) */
    public PlanDeEstudios getPlanDeEstudios() { return planDeEstudios; }

    /** @param planDeEstudios nuevo plan de estudios */
    public void setPlanDeEstudios(PlanDeEstudios planDeEstudios) {
        this.planDeEstudios = planDeEstudios;
    }

    @Override
    public boolean equals(Object o) {
        if (this == o) return true;
        if (!(o instanceof ProgramaAcademico p)) return false;
        return codigo.equals(p.codigo);
    }

    @Override
    public int hashCode() {
        return Objects.hash(codigo);
    }

    @Override
    public String toString() {
        return String.format("Programa[%s - %s, nivel=%s, modalidad=%s]",
                             codigo, nombre, nivel, modalidad);
    }
}
