package co.edu.udistrital.estatuto.modelo;

import java.util.Objects;

/**
 * Clase abstracta que representa a cualquier persona vinculada
 * a la Universidad Distrital Francisco José de Caldas.
 *
 * <p>La comunidad universitaria comprende docentes, estudiantes,
 * personal administrativo y egresados (Art. 8, Acuerdo 004 de 2025).
 * Todos comparten los atributos básicos de identificación definidos
 * en esta clase.</p>
 *
 * @author Roberto Albeiro Pava-Díaz
 * @version 1.0
 * @see Docente
 * @see Estudiante
 * @see PersonalAdministrativo
 * @see Egresado
 */
public abstract class Persona {

    /** Nombre completo de la persona. */
    private String nombre;

    /** Número de identificación (cédula, pasaporte, etc.). */
    private String identificacion;

    /** Dirección de correo electrónico institucional o personal. */
    private String correo;

    /**
     * Construye una Persona con los datos de identificación.
     *
     * @param nombre         nombre completo (no nulo)
     * @param identificacion número de identificación (no nulo)
     * @param correo         dirección de correo electrónico
     * @throws NullPointerException si nombre o identificacion son nulos
     */
    protected Persona(String nombre, String identificacion, String correo) {
        this.nombre         = Objects.requireNonNull(nombre, "El nombre es requerido.");
        this.identificacion = Objects.requireNonNull(identificacion, "La identificación es requerida.");
        this.correo         = correo;
    }

    /**
     * Retorna el rol institucional de la persona.
     * Cada subclase debe implementar este método.
     *
     * @return cadena descriptiva del rol (e.g., "Docente", "Estudiante")
     */
    public abstract String getRol();

    // ── Getters y Setters ──────────────────────────────────────────────────

    /** @return nombre completo de la persona */
    public String getNombre() { return nombre; }

    /** @param nombre nuevo nombre completo */
    public void setNombre(String nombre) {
        this.nombre = Objects.requireNonNull(nombre, "El nombre no puede ser nulo.");
    }

    /** @return número de identificación */
    public String getIdentificacion() { return identificacion; }

    /** @param identificacion nuevo número de identificación */
    public void setIdentificacion(String identificacion) {
        this.identificacion = Objects.requireNonNull(identificacion);
    }

    /** @return dirección de correo electrónico */
    public String getCorreo() { return correo; }

    /** @param correo nueva dirección de correo */
    public void setCorreo(String correo) { this.correo = correo; }

    // ── equals, hashCode, toString ─────────────────────────────────────────

    /**
     * Dos personas son iguales si tienen la misma identificación.
     */
    @Override
    public boolean equals(Object o) {
        if (this == o) return true;
        if (!(o instanceof Persona p)) return false;
        return identificacion.equals(p.identificacion);
    }

    @Override
    public int hashCode() {
        return Objects.hash(identificacion);
    }

    @Override
    public String toString() {
        return String.format("%s[id=%s, nombre=%s, correo=%s]",
                             getRol(), identificacion, nombre, correo);
    }
}
