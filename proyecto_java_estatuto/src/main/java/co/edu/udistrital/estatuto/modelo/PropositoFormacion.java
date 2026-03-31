package co.edu.udistrital.estatuto.modelo;

import java.util.Objects;

/**
 * Propósito de Formación y Aprendizaje (PFA) en uno de los tres ámbitos
 * definidos por el documento de PFA de la Universidad Distrital (2023):
 * ontológico, epistemológico y contextual.
 *
 * <p>Los propósitos de formación orientan el proyecto curricular hacia
 * el desarrollo integral del sujeto en sus dimensiones ser, saber y hacer,
 * articulados con las tres funciones misionales de la universidad.</p>
 *
 * @author Roberto Albeiro Pava-Díaz
 * @version 1.0
 */
public class PropositoFormacion {

    /**
     * Ámbitos de los propósitos de formación según el documento PFA de la UD.
     */
    public enum Ambito {
        /** Dimensión del ser: formación integral del sujeto. */
        ONTOLOGICO,

        /** Dimensión del saber: construcción del conocimiento. */
        EPISTEMOLOGICO,

        /** Dimensión del hacer: relación con el entorno y la sociedad. */
        CONTEXTUAL
    }

    /** Ámbito al que pertenece este propósito. */
    private Ambito ambito;

    /** Descripción detallada del propósito de formación. */
    private String descripcion;

    /** Código del programa académico al que está asociado. */
    private String programaCodigo;

    /**
     * Construye un PropositoFormacion.
     *
     * @param ambito         ámbito del propósito (no nulo)
     * @param descripcion    descripción del propósito (no nula)
     * @param programaCodigo código del programa asociado
     */
    public PropositoFormacion(Ambito ambito, String descripcion, String programaCodigo) {
        this.ambito        = Objects.requireNonNull(ambito, "El ámbito no puede ser nulo.");
        this.descripcion   = Objects.requireNonNull(descripcion, "La descripción no puede ser nula.");
        this.programaCodigo = programaCodigo;
    }

    // ── Getters y Setters ──────────────────────────────────────────────────

    /** @return ámbito del propósito */
    public Ambito getAmbito()           { return ambito; }

    /** @param ambito nuevo ámbito */
    public void   setAmbito(Ambito a)   { this.ambito = Objects.requireNonNull(a); }

    /** @return descripción del propósito */
    public String getDescripcion()      { return descripcion; }

    /** @param descripcion nueva descripción */
    public void   setDescripcion(String d) { this.descripcion = d; }

    /** @return código del programa */
    public String getProgramaCodigo()   { return programaCodigo; }

    /** @param p nuevo código de programa */
    public void   setProgramaCodigo(String p) { this.programaCodigo = p; }

    @Override
    public String toString() {
        return String.format("PFA[ambito=%s, programa=%s, desc=%s]",
                             ambito, programaCodigo, descripcion);
    }
}
