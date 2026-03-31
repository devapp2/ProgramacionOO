package co.edu.udistrital.estatuto.modelo;

import java.util.ArrayList;
import java.util.Collections;
import java.util.List;
import java.util.Objects;

/**
 * Plan de estudios de un programa académico.
 *
 * <p>Contiene la versión vigente, los créditos totales aprobados por el
 * Consejo Académico y la lista de espacios académicos que lo componen.</p>
 *
 * @author Roberto Albeiro Pava-Díaz
 * @version 1.0
 */
public class PlanDeEstudios {

    /** Versión o resolución que aprobó este plan. */
    private String version;

    /** Número de créditos totales aprobados para el plan. */
    private int creditosTotales;

    /** Lista de espacios académicos que componen el plan. */
    private final List<EspacioAcademico> espacios;

    /**
     * Construye un PlanDeEstudios.
     *
     * @param version        versión o resolución de aprobación (no nula)
     * @param creditosTotales créditos totales aprobados (debe ser &gt; 0)
     * @throws IllegalArgumentException si creditosTotales &le; 0
     */
    public PlanDeEstudios(String version, int creditosTotales) {
        this.version         = Objects.requireNonNull(version, "La versión no puede ser nula.");
        if (creditosTotales <= 0) {
            throw new IllegalArgumentException("Los créditos totales deben ser mayores a cero.");
        }
        this.creditosTotales = creditosTotales;
        this.espacios        = new ArrayList<>();
    }

    /**
     * Agrega un espacio académico al plan.
     *
     * @param espacio espacio académico a agregar (no nulo)
     * @return {@code true} si se agregó correctamente
     */
    public boolean agregarEspacio(EspacioAcademico espacio) {
        Objects.requireNonNull(espacio, "El espacio académico no puede ser nulo.");
        return espacios.add(espacio);
    }

    /**
     * Elimina un espacio académico por su código.
     *
     * @param codigo código del espacio a eliminar
     * @return {@code true} si se eliminó
     */
    public boolean eliminarEspacio(String codigo) {
        return espacios.removeIf(e -> e.getCodigo().equals(codigo));
    }

    /**
     * Calcula la suma real de créditos de los espacios registrados.
     * Puede diferir de {@link #getCreditosTotales()} si el plan está incompleto.
     *
     * @return suma de créditos de los espacios registrados
     */
    public int calcularCreditosAcumulados() {
        return espacios.stream().mapToInt(EspacioAcademico::getCreditos).sum();
    }

    // ── Getters y Setters ──────────────────────────────────────────────────

    /** @return versión del plan */
    public String getVersion() { return version; }

    /** @param version nueva versión */
    public void setVersion(String version) { this.version = version; }

    /** @return créditos totales aprobados */
    public int getCreditosTotales() { return creditosTotales; }

    /** @param creditosTotales nuevos créditos totales */
    public void setCreditosTotales(int creditosTotales) {
        if (creditosTotales <= 0) {
            throw new IllegalArgumentException("Los créditos totales deben ser mayores a cero.");
        }
        this.creditosTotales = creditosTotales;
    }

    /** @return lista inmutable de espacios académicos */
    public List<EspacioAcademico> getEspacios() {
        return Collections.unmodifiableList(espacios);
    }

    @Override
    public String toString() {
        return String.format("PlanDeEstudios[version=%s, creditos=%d, espacios=%d]",
                             version, creditosTotales, espacios.size());
    }
}
