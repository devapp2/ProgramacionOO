package co.edu.udistrital.estatuto.modelo;

import java.util.Objects;

/**
 * Representa a un estudiante de la Universidad Distrital.
 *
 * <p>Un estudiante posee un código estudiantil único, está matriculado
 * en un programa académico y transita por distintos estados a lo largo
 * de su ciclo de vida institucional.</p>
 *
 * @author Roberto Albeiro Pava-Díaz
 * @version 1.0
 */
public class Estudiante extends Persona {

    /** Código estudiantil único (diferente del número de identificación). */
    private String codigoEstudiantil;

    /** Código del programa académico en que está matriculado. */
    private String programaCodigo;

    /** Semestre académico actual del estudiante (1 a 10 para pregrado). */
    private int semestre;

    /** Estado académico vigente del estudiante. */
    private EstadoEstudiante estado;

    /**
     * Construye un Estudiante con todos sus atributos.
     *
     * @param nombre             nombre completo
     * @param identificacion     número de identificación
     * @param correo             correo electrónico
     * @param codigoEstudiantil  código estudiantil único (no nulo)
     * @param programaCodigo     código del programa en que está matriculado
     * @param semestre           semestre actual (debe ser &gt; 0)
     * @param estado             estado académico vigente
     * @throws NullPointerException     si codigoEstudiantil es nulo
     * @throws IllegalArgumentException si semestre &le; 0
     */
    public Estudiante(String nombre, String identificacion, String correo,
                      String codigoEstudiantil, String programaCodigo,
                      int semestre, EstadoEstudiante estado) {
        super(nombre, identificacion, correo);
        this.codigoEstudiantil = Objects.requireNonNull(codigoEstudiantil,
                                 "El código estudiantil no puede ser nulo.");
        this.programaCodigo    = programaCodigo;
        setSemestre(semestre);
        this.estado            = estado != null ? estado : EstadoEstudiante.ASPIRANTE;
    }

    @Override
    public String getRol() {
        return "Estudiante";
    }

    // ── Getters y Setters ──────────────────────────────────────────────────

    /** @return código estudiantil */
    public String getCodigoEstudiantil() { return codigoEstudiantil; }

    /** @param codigoEstudiantil nuevo código estudiantil */
    public void setCodigoEstudiantil(String codigoEstudiantil) {
        this.codigoEstudiantil = Objects.requireNonNull(codigoEstudiantil);
    }

    /** @return código del programa académico */
    public String getProgramaCodigo() { return programaCodigo; }

    /** @param programaCodigo nuevo código de programa */
    public void setProgramaCodigo(String programaCodigo) {
        this.programaCodigo = programaCodigo;
    }

    /** @return semestre académico actual */
    public int getSemestre() { return semestre; }

    /**
     * Actualiza el semestre del estudiante.
     * @param semestre nuevo semestre (debe ser &gt; 0)
     * @throws IllegalArgumentException si semestre &le; 0
     */
    public void setSemestre(int semestre) {
        if (semestre <= 0) {
            throw new IllegalArgumentException(
                "El semestre debe ser mayor a cero.");
        }
        this.semestre = semestre;
    }

    /** @return estado académico vigente */
    public EstadoEstudiante getEstado() { return estado; }

    /** @param estado nuevo estado académico */
    public void setEstado(EstadoEstudiante estado) {
        this.estado = Objects.requireNonNull(estado);
    }

    /**
     * Verifica si el estudiante está activo (matrícula vigente).
     * @return {@code true} si el estado es {@link EstadoEstudiante#ACTIVO}
     */
    public boolean estaActivo() {
        return EstadoEstudiante.ACTIVO.equals(estado);
    }

    @Override
    public String toString() {
        return String.format("Estudiante[cod=%s, id=%s, nombre=%s, programa=%s, semestre=%d, estado=%s]",
                             codigoEstudiantil, getIdentificacion(), getNombre(),
                             programaCodigo, semestre, estado);
    }
}
