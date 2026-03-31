package co.edu.udistrital.estatuto.util;

import java.util.regex.Pattern;

/**
 * Clase de utilidad con métodos de validación para los datos del sistema.
 *
 * <p>Centraliza las validaciones de formato y contenido, evitando que
 * la lógica de validación se duplique en las clases de dominio y en
 * los controladores.</p>
 *
 * @author Roberto Albeiro Pava-Díaz
 * @version 1.0
 */
public final class Validador {

    /** Patrón para código institucional (letras mayúsculas, números, guiones). */
    private static final Pattern PATRON_CODIGO =
        Pattern.compile("^[A-Z0-9][A-Z0-9\\-]{0,19}$");

    /** Patrón básico de validación de correo electrónico. */
    private static final Pattern PATRON_CORREO =
        Pattern.compile("^[\\w._%+\\-]+@[\\w.\\-]+\\.[A-Za-z]{2,}$");

    /** Patrón para periodo académico (ej: "2025-1", "2025-2"). */
    private static final Pattern PATRON_PERIODO =
        Pattern.compile("^\\d{4}-[12]$");

    /** Clase de utilidad: constructor privado. */
    private Validador() {
        throw new UnsupportedOperationException("Clase de utilidad, no instanciar.");
    }

    /**
     * Verifica si una cadena no es nula y no está vacía (después de recorte).
     *
     * @param valor cadena a verificar
     * @return {@code true} si la cadena tiene contenido
     */
    public static boolean noEsVacia(String valor) {
        return valor != null && !valor.isBlank();
    }

    /**
     * Verifica si un código institucional tiene formato válido.
     * Formato esperado: entre 1 y 20 caracteres alfanuméricos en mayúsculas,
     * con guiones permitidos (no al inicio).
     *
     * @param codigo código a validar
     * @return {@code true} si el formato es válido
     */
    public static boolean esCodigoValido(String codigo) {
        if (codigo == null || codigo.isBlank()) return false;
        return PATRON_CODIGO.matcher(codigo.trim().toUpperCase()).matches();
    }

    /**
     * Verifica si una dirección de correo electrónico tiene formato válido.
     *
     * @param correo dirección de correo a validar
     * @return {@code true} si el formato es válido
     */
    public static boolean esCorreoValido(String correo) {
        if (correo == null || correo.isBlank()) return false;
        return PATRON_CORREO.matcher(correo.trim()).matches();
    }

    /**
     * Verifica si un periodo académico tiene el formato correcto (AAAA-N).
     *
     * @param periodo periodo a validar (e.g., "2025-1")
     * @return {@code true} si el formato es válido
     */
    public static boolean esPeriodoValido(String periodo) {
        if (periodo == null || periodo.isBlank()) return false;
        return PATRON_PERIODO.matcher(periodo.trim()).matches();
    }

    /**
     * Verifica si una calificación está en el rango válido (0.0–5.0).
     *
     * @param calificacion valor de la calificación
     * @return {@code true} si está en rango
     */
    public static boolean esCalificacionValida(double calificacion) {
        return calificacion >= 0.0 && calificacion <= 5.0;
    }

    /**
     * Verifica si un número de semestre es válido (1 a 20).
     *
     * @param semestre número de semestre
     * @return {@code true} si está en rango
     */
    public static boolean esSemestreValido(int semestre) {
        return semestre >= 1 && semestre <= 20;
    }

    /**
     * Verifica si una ponderación está en el rango válido (0.0–1.0).
     *
     * @param ponderacion valor de ponderación
     * @return {@code true} si está en rango
     */
    public static boolean esPonderacionValida(double ponderacion) {
        return ponderacion >= 0.0 && ponderacion <= 1.0;
    }

    /**
     * Normaliza un código institucional: elimina espacios y convierte a mayúsculas.
     *
     * @param codigo código a normalizar
     * @return código normalizado, o cadena vacía si la entrada es nula
     */
    public static String normalizarCodigo(String codigo) {
        if (codigo == null) return "";
        return codigo.trim().toUpperCase();
    }

    /**
     * Normaliza un nombre propio: elimina espacios extremos y unifica espacios internos.
     *
     * @param nombre nombre a normalizar
     * @return nombre normalizado
     */
    public static String normalizarNombre(String nombre) {
        if (nombre == null) return "";
        return nombre.trim().replaceAll("\\s+", " ");
    }
}
