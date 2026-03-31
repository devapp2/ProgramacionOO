package co.edu.udistrital.estatuto.modelo;

import java.util.Objects;

/**
 * Representa a un docente de la Universidad Distrital Francisco José de Caldas.
 *
 * <p>Los docentes se clasifican según su tipo de vinculación (planta, ocasional,
 * hora cátedra, visitante, experto) y están adscritos a una Escuela.
 * La categoría docente (Auxiliar, Asistente, Asociado, Titular) aplica
 * únicamente a los docentes de planta.</p>
 *
 * @author Roberto Albeiro Pava-Díaz
 * @version 1.0
 */
public class Docente extends Persona {

    /** Tipo de vinculación del docente con la universidad. */
    private TipoVinculacion tipoVinculacion;

    /** Código de la Escuela a la que está adscrito. */
    private String escuelaCodigo;

    /** Categoría docente (aplica solo para planta). */
    private String categoria;

    /**
     * Construye un Docente con todos sus atributos.
     *
     * @param nombre         nombre completo
     * @param identificacion número de identificación
     * @param correo         correo institucional
     * @param tipoVinculacion tipo de vinculación (no nulo)
     * @param escuelaCodigo  código de la escuela adscrita
     * @param categoria      categoría docente (puede ser null para no-planta)
     * @throws NullPointerException si tipoVinculacion es nulo
     */
    public Docente(String nombre, String identificacion, String correo,
                   TipoVinculacion tipoVinculacion, String escuelaCodigo,
                   String categoria) {
        super(nombre, identificacion, correo);
        this.tipoVinculacion = Objects.requireNonNull(tipoVinculacion,
                               "El tipo de vinculación no puede ser nulo.");
        this.escuelaCodigo   = escuelaCodigo;
        this.categoria       = categoria;
    }

    /**
     * Constructor simplificado sin categoría (para docentes no-planta).
     *
     * @param nombre          nombre completo
     * @param identificacion  número de identificación
     * @param correo          correo institucional
     * @param tipoVinculacion tipo de vinculación
     * @param escuelaCodigo   código de la escuela adscrita
     */
    public Docente(String nombre, String identificacion, String correo,
                   TipoVinculacion tipoVinculacion, String escuelaCodigo) {
        this(nombre, identificacion, correo, tipoVinculacion, escuelaCodigo, null);
    }

    @Override
    public String getRol() {
        return "Docente";
    }

    // ── Getters y Setters ──────────────────────────────────────────────────

    /** @return tipo de vinculación del docente */
    public TipoVinculacion getTipoVinculacion() { return tipoVinculacion; }

    /** @param tipoVinculacion nuevo tipo de vinculación */
    public void setTipoVinculacion(TipoVinculacion tipoVinculacion) {
        this.tipoVinculacion = Objects.requireNonNull(tipoVinculacion);
    }

    /** @return código de la escuela adscrita */
    public String getEscuelaCodigo() { return escuelaCodigo; }

    /** @param escuelaCodigo nuevo código de escuela */
    public void setEscuelaCodigo(String escuelaCodigo) { this.escuelaCodigo = escuelaCodigo; }

    /** @return categoría docente (puede ser null) */
    public String getCategoria() { return categoria; }

    /** @param categoria nueva categoría docente */
    public void setCategoria(String categoria) { this.categoria = categoria; }

    /**
     * Verifica si el docente es de planta.
     * @return {@code true} si su vinculación es {@link TipoVinculacion#PLANTA}
     */
    public boolean esPlanta() {
        return TipoVinculacion.PLANTA.equals(tipoVinculacion);
    }

    @Override
    public String toString() {
        return String.format("Docente[id=%s, nombre=%s, tipo=%s, escuela=%s, categoria=%s]",
                             getIdentificacion(), getNombre(),
                             tipoVinculacion, escuelaCodigo, categoria);
    }
}
