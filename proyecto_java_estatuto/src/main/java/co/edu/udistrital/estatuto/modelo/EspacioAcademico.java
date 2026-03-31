package co.edu.udistrital.estatuto.modelo;

import java.util.Objects;

/**
 * Espacio académico (asignatura, componente o actividad académica)
 * del plan de estudios de un programa.
 *
 * @author Roberto Albeiro Pava-Díaz
 * @version 1.0
 */
public class EspacioAcademico {

    /** Nombre del espacio académico. */
    private String nombre;

    /** Código único del espacio en el plan de estudios. */
    private String codigo;

    /** Número de créditos académicos. */
    private int creditos;

    /** Tipo: "Obligatorio", "Electivo", "Libre Elección", etc. */
    private String tipo;

    /** Horas semanales de actividad teórica. */
    private int horasTeoricas;

    /** Horas semanales de actividad práctica. */
    private int horasPracticas;

    /**
     * Construye un EspacioAcademico completo.
     *
     * @param nombre         nombre del espacio (no nulo)
     * @param codigo         código único (no nulo)
     * @param creditos       número de créditos (&gt; 0)
     * @param tipo           tipo de espacio
     * @param horasTeoricas  horas semanales teóricas (&ge; 0)
     * @param horasPracticas horas semanales prácticas (&ge; 0)
     */
    public EspacioAcademico(String nombre, String codigo, int creditos,
                             String tipo, int horasTeoricas, int horasPracticas) {
        this.nombre          = Objects.requireNonNull(nombre);
        this.codigo          = Objects.requireNonNull(codigo);
        this.creditos        = creditos;
        this.tipo            = tipo;
        this.horasTeoricas   = horasTeoricas;
        this.horasPracticas  = horasPracticas;
    }

    // ── Getters y Setters ──────────────────────────────────────────────────

    /** @return nombre del espacio */
    public String getNombre()          { return nombre; }

    /** @param nombre nuevo nombre */
    public void   setNombre(String n)  { this.nombre = n; }

    /** @return código del espacio */
    public String getCodigo()          { return codigo; }

    /** @param codigo nuevo código */
    public void   setCodigo(String c)  { this.codigo = c; }

    /** @return número de créditos */
    public int    getCreditos()        { return creditos; }

    /** @param creditos nuevo número de créditos */
    public void   setCreditos(int c)   { this.creditos = c; }

    /** @return tipo de espacio académico */
    public String getTipo()            { return tipo; }

    /** @param tipo nuevo tipo */
    public void   setTipo(String t)    { this.tipo = t; }

    /** @return horas semanales teóricas */
    public int    getHorasTeoricas()   { return horasTeoricas; }

    /** @param h nuevas horas teóricas */
    public void   setHorasTeoricas(int h) { this.horasTeoricas = h; }

    /** @return horas semanales prácticas */
    public int    getHorasPracticas()  { return horasPracticas; }

    /** @param h nuevas horas prácticas */
    public void   setHorasPracticas(int h) { this.horasPracticas = h; }

    /**
     * Calcula el total de horas semanales (teóricas + prácticas).
     * @return total de horas semanales
     */
    public int    getHorasTotales()    { return horasTeoricas + horasPracticas; }

    @Override
    public boolean equals(Object o) {
        if (this == o) return true;
        if (!(o instanceof EspacioAcademico e)) return false;
        return codigo.equals(e.codigo);
    }

    @Override
    public int hashCode() { return Objects.hash(codigo); }

    @Override
    public String toString() {
        return String.format("Espacio[%s - %s, creditos=%d, tipo=%s]",
                             codigo, nombre, creditos, tipo);
    }
}
