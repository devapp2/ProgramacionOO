package co.edu.udistrital.estatuto.patron;

import co.edu.udistrital.estatuto.modelo.Evidencia;

import java.util.List;

/**
 * Interfaz Strategy para el cálculo de la nota final
 * a partir de un conjunto de evidencias de aprendizaje.
 *
 * <p>Permite seleccionar en tiempo de ejecución el algoritmo de
 * cálculo apropiado según el tipo de programa o espacio académico,
 * sin modificar el código cliente.</p>
 *
 * @author Roberto Albeiro Pava-Díaz
 * @version 1.0
 */
public interface EvaluacionStrategy {

    /**
     * Calcula la nota final con base en las evidencias proporcionadas.
     *
     * @param evidencias lista de evidencias con sus calificaciones
     * @return nota final en escala 0.0–5.0
     */
    double calcularNotaFinal(List<Evidencia> evidencias);

    /**
     * Retorna el nombre descriptivo de la estrategia (para reportes).
     * @return nombre de la estrategia
     */
    String getNombreEstrategia();

    // ── Implementaciones internas (clases estáticas) ─────────────────────

    /**
     * Estrategia: promedio aritmético simple de todas las evidencias.
     */
    final class PromedioAritmetico implements EvaluacionStrategy {

        @Override
        public double calcularNotaFinal(List<Evidencia> evidencias) {
            if (evidencias == null || evidencias.isEmpty()) return 0.0;
            return evidencias.stream()
                             .mapToDouble(Evidencia::getCalificacion)
                             .average()
                             .orElse(0.0);
        }

        @Override
        public String getNombreEstrategia() { return "Promedio Aritmético"; }
    }

    /**
     * Estrategia: nota mínima (la más baja de todas las evidencias).
     * Útil para componentes donde se exige un nivel mínimo en todos los ítems.
     */
    final class NotaMinima implements EvaluacionStrategy {

        @Override
        public double calcularNotaFinal(List<Evidencia> evidencias) {
            if (evidencias == null || evidencias.isEmpty()) return 0.0;
            return evidencias.stream()
                             .mapToDouble(Evidencia::getCalificacion)
                             .min()
                             .orElse(0.0);
        }

        @Override
        public String getNombreEstrategia() { return "Nota Mínima"; }
    }

    /**
     * Estrategia: nota máxima (la más alta de todas las evidencias).
     * Puede usarse en evaluación recuperativa.
     */
    final class NotaMaxima implements EvaluacionStrategy {

        @Override
        public double calcularNotaFinal(List<Evidencia> evidencias) {
            if (evidencias == null || evidencias.isEmpty()) return 0.0;
            return evidencias.stream()
                             .mapToDouble(Evidencia::getCalificacion)
                             .max()
                             .orElse(0.0);
        }

        @Override
        public String getNombreEstrategia() { return "Nota Máxima"; }
    }
}
