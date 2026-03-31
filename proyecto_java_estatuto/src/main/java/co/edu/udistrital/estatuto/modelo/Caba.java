package co.edu.udistrital.estatuto.modelo;

import java.util.Objects;

/**
 * Comité Académico Básico de Área (CABA).
 *
 * <p>Los CABAS articulan las funciones de docencia e investigación
 * al interior de cada Escuela, conforme al Acuerdo 004 de 2025.
 * Cada CABA pertenece a un área de formación y es coordinado por un
 * docente designado por el Consejo de Escuela.</p>
 *
 * @author Roberto Albeiro Pava-Díaz
 * @version 1.0
 */
public class Caba {

    /** Nombre del CABA. */
    private String nombre;

    /** Área de formación a la que pertenece este CABA. */
    private String areaFormacion;

    /** Nombre del coordinador del CABA. */
    private String coordinador;

    /**
     * Construye un CABA con los datos básicos.
     *
     * @param nombre        nombre del CABA (no nulo)
     * @param areaFormacion área de formación (no nula)
     * @param coordinador   nombre del coordinador designado
     */
    public Caba(String nombre, String areaFormacion, String coordinador) {
        this.nombre        = Objects.requireNonNull(nombre, "El nombre no puede ser nulo.");
        this.areaFormacion = Objects.requireNonNull(areaFormacion, "El área de formación no puede ser nula.");
        this.coordinador   = coordinador;
    }

    // ── Getters y Setters ──────────────────────────────────────────────────

    /** @return nombre del CABA */
    public String getNombre() { return nombre; }

    /** @param nombre nuevo nombre del CABA */
    public void setNombre(String nombre) { this.nombre = nombre; }

    /** @return área de formación */
    public String getAreaFormacion() { return areaFormacion; }

    /** @param areaFormacion nueva área de formación */
    public void setAreaFormacion(String areaFormacion) { this.areaFormacion = areaFormacion; }

    /** @return nombre del coordinador */
    public String getCoordinador() { return coordinador; }

    /** @param coordinador nombre del nuevo coordinador */
    public void setCoordinador(String coordinador) { this.coordinador = coordinador; }

    @Override
    public boolean equals(Object o) {
        if (this == o) return true;
        if (!(o instanceof Caba c)) return false;
        return nombre.equals(c.nombre) && areaFormacion.equals(c.areaFormacion);
    }

    @Override
    public int hashCode() {
        return Objects.hash(nombre, areaFormacion);
    }

    @Override
    public String toString() {
        return String.format("CABA[nombre=%s, area=%s, coordinador=%s]",
                             nombre, areaFormacion, coordinador);
    }
}
