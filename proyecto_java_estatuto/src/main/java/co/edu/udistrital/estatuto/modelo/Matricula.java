package co.edu.udistrital.estatuto.modelo;

import java.util.Objects;

/**
 * Matrícula que vincula a un Estudiante con un ProgramaAcademico
 * en un periodo académico determinado.
 *
 * @author Roberto Albeiro Pava-Díaz
 * @version 1.0
 */
public class Matricula {

    /** Número de identificación del estudiante matriculado. */
    private String estudianteId;

    /** Código del programa en que se matricula. */
    private String programaCodigo;

    /** Periodo académico en formato "AAAA-N" (e.g., "2025-2"). */
    private String periodo;

    /** Estado académico del estudiante en esta matrícula. */
    private EstadoEstudiante estado;

    /** Valor cancelado por concepto de matrícula. */
    private double valorPagado;

    /**
     * Construye una Matrícula.
     *
     * @param estudianteId   número de identificación del estudiante (no nulo)
     * @param programaCodigo código del programa (no nulo)
     * @param periodo        periodo académico en formato "AAAA-N" (no nulo)
     * @param estado         estado académico del estudiante
     */
    public Matricula(String estudianteId, String programaCodigo,
                     String periodo, EstadoEstudiante estado) {
        this.estudianteId   = Objects.requireNonNull(estudianteId, "El ID del estudiante no puede ser nulo.");
        this.programaCodigo = Objects.requireNonNull(programaCodigo, "El código del programa no puede ser nulo.");
        this.periodo        = Objects.requireNonNull(periodo, "El periodo no puede ser nulo.");
        this.estado         = estado != null ? estado : EstadoEstudiante.ACTIVO;
        this.valorPagado    = 0.0;
    }

    // ── Getters y Setters ──────────────────────────────────────────────────

    /** @return ID del estudiante */
    public String getEstudianteId() { return estudianteId; }

    /** @return código del programa */
    public String getProgramaCodigo() { return programaCodigo; }

    /** @return periodo académico */
    public String getPeriodo() { return periodo; }

    /** @return estado del estudiante en esta matrícula */
    public EstadoEstudiante getEstado() { return estado; }

    /** @param estado nuevo estado */
    public void setEstado(EstadoEstudiante estado) {
        this.estado = Objects.requireNonNull(estado);
    }

    /** @return valor pagado por matrícula */
    public double getValorPagado() { return valorPagado; }

    /**
     * Registra el valor pagado por matrícula.
     * @param valorPagado monto pagado (debe ser &ge; 0)
     * @throws IllegalArgumentException si el valor es negativo
     */
    public void setValorPagado(double valorPagado) {
        if (valorPagado < 0) {
            throw new IllegalArgumentException("El valor pagado no puede ser negativo.");
        }
        this.valorPagado = valorPagado;
    }

    @Override
    public boolean equals(Object o) {
        if (this == o) return true;
        if (!(o instanceof Matricula m)) return false;
        return estudianteId.equals(m.estudianteId)
            && programaCodigo.equals(m.programaCodigo)
            && periodo.equals(m.periodo);
    }

    @Override
    public int hashCode() {
        return Objects.hash(estudianteId, programaCodigo, periodo);
    }

    @Override
    public String toString() {
        return String.format("Matricula[estudiante=%s, programa=%s, periodo=%s, estado=%s]",
                             estudianteId, programaCodigo, periodo, estado);
    }
}
