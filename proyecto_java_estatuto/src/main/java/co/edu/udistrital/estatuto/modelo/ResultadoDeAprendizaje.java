package co.edu.udistrital.estatuto.modelo;

import java.util.Objects;

/**
 * Resultado de Aprendizaje (RA) asociado a un Programa Académico.
 *
 * <p>Formulado con verbos de acción observables (Taxonomía de Bloom)
 * y articulado con el perfil de egreso del programa, conforme al
 * Decreto 1330 de 2019 y el Acuerdo 02 de 2020 del CESU.</p>
 *
 * @author Roberto Albeiro Pava-Díaz
 * @version 1.0
 */
public class ResultadoDeAprendizaje {

    /** Descripción observable del resultado (formulada con verbo de acción). */
    private String descripcion;

    /** Nivel de dominio esperado: "Básico", "Intermedio", "Avanzado". */
    private String nivelDominio;

    /** Código del programa académico al que está asociado. */
    private String programaCodigo;

    /**
     * Construye un ResultadoDeAprendizaje.
     *
     * @param descripcion    descripción observable del resultado (no nula)
     * @param nivelDominio   nivel de dominio esperado
     * @param programaCodigo código del programa asociado
     */
    public ResultadoDeAprendizaje(String descripcion, String nivelDominio,
                                   String programaCodigo) {
        this.descripcion   = Objects.requireNonNull(descripcion, "La descripción no puede ser nula.");
        this.nivelDominio  = nivelDominio;
        this.programaCodigo = programaCodigo;
    }

    // ── Getters y Setters ──────────────────────────────────────────────────

    /** @return descripción del resultado */
    public String getDescripcion()    { return descripcion; }

    /** @param descripcion nueva descripción */
    public void   setDescripcion(String d) { this.descripcion = d; }

    /** @return nivel de dominio */
    public String getNivelDominio()   { return nivelDominio; }

    /** @param n nuevo nivel de dominio */
    public void   setNivelDominio(String n) { this.nivelDominio = n; }

    /** @return código del programa */
    public String getProgramaCodigo() { return programaCodigo; }

    /** @param p nuevo código de programa */
    public void   setProgramaCodigo(String p) { this.programaCodigo = p; }

    @Override
    public String toString() {
        return String.format("RA[nivel=%s, programa=%s, desc=%s]",
                             nivelDominio, programaCodigo, descripcion);
    }
}
