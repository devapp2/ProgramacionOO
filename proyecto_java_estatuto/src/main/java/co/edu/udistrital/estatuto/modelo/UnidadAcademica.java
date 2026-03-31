package co.edu.udistrital.estatuto.modelo;

import java.time.LocalDate;
import java.util.Objects;

/**
 * Clase abstracta que representa una unidad académico-administrativa
 * de la Universidad Distrital Francisco José de Caldas.
 *
 * <p>Conforme al Acuerdo 004 de 2025 (Estatuto General), toda unidad
 * académica posee un nombre, un código institucional, una fecha de
 * creación oficial y un director designado por el órgano competente.</p>
 *
 * <p>Las subclases concretas son: {@link Facultad}, {@link Escuela},
 * {@link Centro} e {@link Instituto}.</p>
 *
 * @author Roberto Albeiro Pava-Díaz
 * @version 1.0
 * @see Facultad
 * @see Escuela
 * @see Centro
 * @see Instituto
 */
public abstract class UnidadAcademica {

    /** Nombre oficial de la unidad académica. */
    private String nombre;

    /** Código institucional único asignado por la Rectoría. */
    private String codigo;

    /** Fecha de creación o reconocimiento oficial de la unidad. */
    private LocalDate fechaCreacion;

    /** Nombre completo del director o directora vigente. */
    private String director;

    /**
     * Construye una unidad académica con los datos básicos.
     *
     * @param nombre        nombre oficial de la unidad (no nulo, no vacío)
     * @param codigo        código institucional (no nulo, no vacío)
     * @param fechaCreacion fecha de creación oficial
     * @param director      nombre del director vigente
     * @throws IllegalArgumentException si el código o el nombre son nulos/vacíos
     * @throws NullPointerException     si el nombre es nulo
     */
    protected UnidadAcademica(String nombre, String codigo,
                               LocalDate fechaCreacion, String director) {
        if (codigo == null || codigo.isBlank()) {
            throw new IllegalArgumentException(
                "El código de la unidad académica no puede ser nulo ni vacío.");
        }
        this.nombre = Objects.requireNonNull(nombre, "El nombre no puede ser nulo.");
        this.codigo = codigo.trim().toUpperCase();
        this.fechaCreacion = fechaCreacion;
        this.director = director;
    }

    /**
     * Retorna el tipo de unidad académica (Facultad, Escuela, Centro, etc.).
     * Cada subclase debe implementar este método con su denominación específica.
     *
     * @return cadena descriptiva del tipo de unidad
     */
    public abstract String getTipo();

    // ── Getters y Setters ──────────────────────────────────────────────────

    /**
     * Retorna el nombre oficial de la unidad.
     * @return nombre de la unidad
     */
    public String getNombre() {
        return nombre;
    }

    /**
     * Actualiza el nombre oficial de la unidad.
     * @param nombre nuevo nombre (no nulo, no vacío)
     * @throws IllegalArgumentException si el nombre es nulo o vacío
     */
    public void setNombre(String nombre) {
        if (nombre == null || nombre.isBlank()) {
            throw new IllegalArgumentException("El nombre no puede ser vacío.");
        }
        this.nombre = nombre.trim();
    }

    /**
     * Retorna el código institucional.
     * @return código en mayúsculas
     */
    public String getCodigo() {
        return codigo;
    }

    /**
     * Actualiza el código institucional. Se normaliza a mayúsculas.
     * @param codigo nuevo código (no nulo, no vacío)
     * @throws IllegalArgumentException si el código es nulo o vacío
     */
    public void setCodigo(String codigo) {
        if (codigo == null || codigo.isBlank()) {
            throw new IllegalArgumentException("El código no puede ser vacío.");
        }
        this.codigo = codigo.trim().toUpperCase();
    }

    /**
     * Retorna la fecha de creación oficial.
     * @return fecha de creación
     */
    public LocalDate getFechaCreacion() {
        return fechaCreacion;
    }

    /**
     * Actualiza la fecha de creación.
     * @param fechaCreacion nueva fecha de creación
     */
    public void setFechaCreacion(LocalDate fechaCreacion) {
        this.fechaCreacion = fechaCreacion;
    }

    /**
     * Retorna el nombre del director vigente.
     * @return nombre del director
     */
    public String getDirector() {
        return director;
    }

    /**
     * Actualiza el nombre del director.
     * @param director nombre del nuevo director
     */
    public void setDirector(String director) {
        this.director = director;
    }

    // ── equals, hashCode, toString ─────────────────────────────────────────

    /**
     * Dos unidades académicas son iguales si tienen el mismo código.
     */
    @Override
    public boolean equals(Object o) {
        if (this == o) return true;
        if (!(o instanceof UnidadAcademica u)) return false;
        return codigo.equals(u.codigo);
    }

    @Override
    public int hashCode() {
        return Objects.hash(codigo);
    }

    @Override
    public String toString() {
        return String.format("%s[codigo=%s, nombre=%s, director=%s]",
                             getTipo(), codigo, nombre, director);
    }
}
