package co.edu.udistrital.estatuto.modelo;

import java.time.LocalDate;
import java.util.Objects;

/**
 * Evidencia de aprendizaje registrada por o sobre un estudiante.
 *
 * <p>Las evidencias son los productos concretos que demuestran el
 * logro de los resultados de aprendizaje. Su calificación se registra
 * en la escala 0.0–5.0 vigente en la Universidad Distrital.</p>
 *
 * @author Roberto Albeiro Pava-Díaz
 * @version 1.0
 */
public class Evidencia {

    /** Tipo de evidencia: "Tarea", "Proyecto", "Examen", "Exposición", etc. */
    private String tipo;

    /** Descripción del contenido o propósito de la evidencia. */
    private String descripcion;

    /** Fecha en que se entregó o evaluó la evidencia. */
    private LocalDate fecha;

    /** Calificación obtenida (escala 0.0–5.0). */
    private double calificacion;

    /**
     * Construye una Evidencia.
     *
     * @param tipo          tipo de evidencia (no nulo)
     * @param descripcion   descripción del contenido
     * @param fecha         fecha de entrega o evaluación
     * @param calificacion  calificación en escala 0.0–5.0
     * @throws IllegalArgumentException si la calificación está fuera del rango
     */
    public Evidencia(String tipo, String descripcion,
                     LocalDate fecha, double calificacion) {
        this.tipo        = Objects.requireNonNull(tipo, "El tipo no puede ser nulo.");
        this.descripcion = descripcion;
        this.fecha       = fecha;
        setCalificacion(calificacion);
    }

    // ── Getters y Setters ──────────────────────────────────────────────────

    /** @return tipo de evidencia */
    public String    getTipo()         { return tipo; }

    /** @param tipo nuevo tipo */
    public void      setTipo(String t) { this.tipo = t; }

    /** @return descripción de la evidencia */
    public String    getDescripcion()  { return descripcion; }

    /** @param descripcion nueva descripción */
    public void      setDescripcion(String d) { this.descripcion = d; }

    /** @return fecha de entrega o evaluación */
    public LocalDate getFecha()        { return fecha; }

    /** @param fecha nueva fecha */
    public void      setFecha(LocalDate f) { this.fecha = f; }

    /** @return calificación obtenida (0.0–5.0) */
    public double    getCalificacion() { return calificacion; }

    /**
     * Actualiza la calificación.
     * @param calificacion nueva calificación (0.0–5.0)
     * @throws IllegalArgumentException si está fuera del rango
     */
    public void setCalificacion(double calificacion) {
        if (calificacion < 0.0 || calificacion > 5.0) {
            throw new IllegalArgumentException(
                "La calificación debe estar entre 0.0 y 5.0. Valor: " + calificacion);
        }
        this.calificacion = calificacion;
    }

    /**
     * Verifica si la evidencia aprueba con una calificación mínima dada.
     * @param umbral calificación mínima de aprobación
     * @return {@code true} si la calificación supera o iguala el umbral
     */
    public boolean aprueba(double umbral) {
        return calificacion >= umbral;
    }

    @Override
    public String toString() {
        return String.format("Evidencia[tipo=%s, fecha=%s, calificacion=%.2f]",
                             tipo, fecha, calificacion);
    }
}
