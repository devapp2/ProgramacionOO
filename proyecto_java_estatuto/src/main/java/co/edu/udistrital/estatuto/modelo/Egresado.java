package co.edu.udistrital.estatuto.modelo;

import java.util.Objects;

/**
 * Egresado de la Universidad Distrital Francisco José de Caldas.
 *
 * <p>Los egresados forman parte de la comunidad universitaria con
 * derechos de participación, conforme al artículo 8 del Acuerdo 004 de 2025.</p>
 *
 * @author Roberto Albeiro Pava-Díaz
 * @version 1.0
 */
public class Egresado extends Persona {

    /** Nombre del programa del que se graduó. */
    private String programaGraduacion;

    /** Año de graduación. */
    private int anioGraduacion;

    /** Título académico obtenido. */
    private String tituloObtenido;

    /**
     * Construye un Egresado con sus datos de graduación.
     *
     * @param nombre              nombre completo
     * @param identificacion      número de identificación
     * @param correo              correo electrónico
     * @param programaGraduacion  nombre del programa de graduación
     * @param anioGraduacion      año de graduación
     * @param tituloObtenido      título académico obtenido
     */
    public Egresado(String nombre, String identificacion, String correo,
                    String programaGraduacion, int anioGraduacion,
                    String tituloObtenido) {
        super(nombre, identificacion, correo);
        this.programaGraduacion = Objects.requireNonNull(programaGraduacion);
        this.anioGraduacion     = anioGraduacion;
        this.tituloObtenido     = Objects.requireNonNull(tituloObtenido);
    }

    @Override
    public String getRol() {
        return "Egresado";
    }

    // ── Getters y Setters ──────────────────────────────────────────────────

    /** @return programa del que se graduó */
    public String getProgramaGraduacion() { return programaGraduacion; }

    /** @param programaGraduacion nuevo programa de graduación */
    public void setProgramaGraduacion(String programaGraduacion) {
        this.programaGraduacion = programaGraduacion;
    }

    /** @return año de graduación */
    public int getAnioGraduacion() { return anioGraduacion; }

    /** @param anioGraduacion nuevo año de graduación */
    public void setAnioGraduacion(int anioGraduacion) {
        this.anioGraduacion = anioGraduacion;
    }

    /** @return título académico obtenido */
    public String getTituloObtenido() { return tituloObtenido; }

    /** @param tituloObtenido nuevo título obtenido */
    public void setTituloObtenido(String tituloObtenido) {
        this.tituloObtenido = tituloObtenido;
    }

    @Override
    public String toString() {
        return String.format("Egresado[id=%s, nombre=%s, programa=%s, anio=%d, titulo=%s]",
                             getIdentificacion(), getNombre(),
                             programaGraduacion, anioGraduacion, tituloObtenido);
    }
}
