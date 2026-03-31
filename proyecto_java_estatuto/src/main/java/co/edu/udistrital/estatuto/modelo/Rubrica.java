package co.edu.udistrital.estatuto.modelo;

import java.util.ArrayList;
import java.util.Collections;
import java.util.List;
import java.util.Objects;

/**
 * Rúbrica de evaluación con criterios y niveles de desempeño.
 *
 * <p>Una rúbrica estructura la evaluación formativa en criterios
 * ponderados, cada uno con descriptores para cada nivel de desempeño.
 * Su uso sistemático favorece la transparencia y la coherencia de la
 * evaluación, tal como lo requiere el sistema de Propósitos de Formación
 * y Aprendizaje de la Universidad Distrital.</p>
 *
 * @author Roberto Albeiro Pava-Díaz
 * @version 1.0
 */
public class Rubrica {

    /** Nombre descriptivo de la rúbrica. */
    private String nombre;

    /** Criterios de evaluación que componen la rúbrica. */
    private final List<CriterioRubrica> criterios;

    /** Nombres de los niveles de desempeño (e.g., "Deficiente", "Aceptable", "Sobresaliente"). */
    private final List<String> niveles;

    /**
     * Construye una rúbrica con su nombre y los niveles de desempeño.
     *
     * @param nombre  nombre descriptivo de la rúbrica (no nulo)
     * @param niveles lista de nombres de niveles de desempeño (no nula, no vacía)
     * @throws IllegalArgumentException si la lista de niveles está vacía
     */
    public Rubrica(String nombre, List<String> niveles) {
        this.nombre    = Objects.requireNonNull(nombre, "El nombre no puede ser nulo.");
        Objects.requireNonNull(niveles, "Los niveles no pueden ser nulos.");
        if (niveles.isEmpty()) {
            throw new IllegalArgumentException("La rúbrica debe tener al menos un nivel.");
        }
        this.niveles   = new ArrayList<>(niveles);
        this.criterios = new ArrayList<>();
    }

    /**
     * Agrega un criterio a la rúbrica.
     *
     * @param criterio criterio a agregar (no nulo)
     * @return {@code true} si se agregó correctamente
     */
    public boolean agregarCriterio(CriterioRubrica criterio) {
        Objects.requireNonNull(criterio, "El criterio no puede ser nulo.");
        return criterios.add(criterio);
    }

    /**
     * Calcula la nota total de la rúbrica a partir de calificaciones por criterio.
     * Las calificaciones están indexadas según el orden de los criterios.
     *
     * @param calificaciones mapa de criterio (índice) a calificación (0.0–5.0)
     * @return nota ponderada en escala 0.0–5.0
     */
    public double calcularNota(double[] calificaciones) {
        if (calificaciones.length != criterios.size()) {
            throw new IllegalArgumentException(
                "El número de calificaciones no coincide con el número de criterios.");
        }
        double suma = 0.0;
        for (int i = 0; i < criterios.size(); i++) {
            suma += criterios.get(i).getPonderacion() * calificaciones[i];
        }
        return suma;
    }

    // ── Getters ────────────────────────────────────────────────────────────

    /** @return nombre de la rúbrica */
    public String getNombre()                       { return nombre; }

    /** @param nombre nuevo nombre */
    public void   setNombre(String nombre)          { this.nombre = nombre; }

    /** @return lista inmutable de criterios */
    public List<CriterioRubrica> getCriterios()     { return Collections.unmodifiableList(criterios); }

    /** @return lista inmutable de niveles */
    public List<String> getNiveles()                { return Collections.unmodifiableList(niveles); }

    /** @return número de criterios registrados */
    public int    getNumCriterios()                 { return criterios.size(); }

    @Override
    public String toString() {
        return String.format("Rubrica[nombre=%s, criterios=%d, niveles=%s]",
                             nombre, criterios.size(), niveles);
    }
}
