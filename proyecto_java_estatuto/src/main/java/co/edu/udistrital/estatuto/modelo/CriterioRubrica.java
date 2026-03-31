package co.edu.udistrital.estatuto.modelo;

import java.util.Collections;
import java.util.LinkedHashMap;
import java.util.Map;
import java.util.Objects;

/**
 * Criterio individual dentro de una rúbrica.
 *
 * <p>Cada criterio tiene un nombre, una ponderación relativa (entre 0 y 1)
 * y descriptores textuales para cada nivel de desempeño de la rúbrica.</p>
 *
 * @author Roberto Albeiro Pava-Díaz
 * @version 1.0
 */
public class CriterioRubrica {

    /** Nombre descriptivo del criterio. */
    private String nombre;

    /**
     * Ponderación del criterio (valor entre 0.0 y 1.0).
     * La suma de ponderaciones de todos los criterios de una rúbrica debe ser 1.0.
     */
    private double ponderacion;

    /** Mapa de nivel de desempeño a descriptor textual. */
    private final Map<String, String> descriptores;

    /**
     * Construye un CriterioRubrica.
     *
     * @param nombre      nombre del criterio (no nulo)
     * @param ponderacion peso relativo del criterio (0.0 a 1.0)
     * @throws IllegalArgumentException si la ponderación está fuera del rango [0, 1]
     */
    public CriterioRubrica(String nombre, double ponderacion) {
        if (ponderacion < 0.0 || ponderacion > 1.0) {
            throw new IllegalArgumentException(
                "La ponderación debe estar entre 0.0 y 1.0. Valor recibido: " + ponderacion);
        }
        this.nombre       = Objects.requireNonNull(nombre, "El nombre no puede ser nulo.");
        this.ponderacion  = ponderacion;
        this.descriptores = new LinkedHashMap<>();
    }

    /**
     * Agrega o actualiza el descriptor para un nivel de desempeño.
     *
     * @param nivel     nombre del nivel (e.g., "Sobresaliente")
     * @param descriptor texto descriptivo del desempeño esperado
     */
    public void agregarDescriptor(String nivel, String descriptor) {
        Objects.requireNonNull(nivel, "El nivel no puede ser nulo.");
        descriptores.put(nivel, descriptor);
    }

    // ── Getters y Setters ──────────────────────────────────────────────────

    /** @return nombre del criterio */
    public String  getNombre()       { return nombre; }

    /** @param nombre nuevo nombre */
    public void    setNombre(String nombre) { this.nombre = nombre; }

    /** @return ponderación del criterio */
    public double  getPonderacion()  { return ponderacion; }

    /**
     * Actualiza la ponderación.
     * @param ponderacion nueva ponderación (0.0 a 1.0)
     */
    public void    setPonderacion(double ponderacion) {
        if (ponderacion < 0.0 || ponderacion > 1.0) {
            throw new IllegalArgumentException("La ponderación debe estar entre 0.0 y 1.0.");
        }
        this.ponderacion = ponderacion;
    }

    /** @return mapa inmutable de descriptores */
    public Map<String, String> getDescriptores() {
        return Collections.unmodifiableMap(descriptores);
    }

    @Override
    public String toString() {
        return String.format("Criterio[nombre=%s, ponderacion=%.2f]",
                             nombre, ponderacion);
    }
}
