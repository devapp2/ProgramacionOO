package co.edu.udistrital.estatuto.patron;

import co.edu.udistrital.estatuto.modelo.*;

import java.time.LocalDate;

/**
 * Factory Method para la creación de unidades académicas.
 *
 * <p>Centraliza la instanciación de {@link UnidadAcademica} y permite
 * incorporar lógica de inicialización compartida (auditoría, logging)
 * sin duplicar código en cada punto de creación. Sigue el principio
 * Abierto/Cerrado: puede extenderse con nuevos tipos sin modificar
 * el código cliente.</p>
 *
 * @author Roberto Albeiro Pava-Díaz
 * @version 1.0
 */
public class UnidadAcademicaFactory {

    /**
     * Tipos de unidad académica que puede crear la factory.
     */
    public enum TipoUnidad {
        FACULTAD, ESCUELA, CENTRO, INSTITUTO
    }

    /**
     * Crea una unidad académica del tipo especificado.
     *
     * @param tipo          tipo de unidad a crear
     * @param nombre        nombre oficial de la unidad
     * @param codigo        código institucional
     * @param fechaCreacion fecha de creación oficial
     * @param director      nombre del director o decano
     * @return instancia concreta de {@link UnidadAcademica}
     * @throws IllegalArgumentException si el tipo no es reconocido
     */
    public static UnidadAcademica crear(TipoUnidad tipo,
                                        String nombre,
                                        String codigo,
                                        LocalDate fechaCreacion,
                                        String director) {
        return switch (tipo) {
            case FACULTAD  -> new Facultad(nombre, codigo, fechaCreacion, director);
            case ESCUELA   -> new Escuela(nombre, codigo, fechaCreacion, director);
            case CENTRO    -> new Centro(nombre, codigo, fechaCreacion, director);
            case INSTITUTO -> new Instituto(nombre, codigo, fechaCreacion, director);
        };
    }

    /**
     * Crea una unidad académica con la fecha de creación como hoy.
     *
     * @param tipo     tipo de unidad
     * @param nombre   nombre oficial
     * @param codigo   código institucional
     * @param director nombre del director
     * @return instancia concreta de {@link UnidadAcademica}
     */
    public static UnidadAcademica crear(TipoUnidad tipo,
                                        String nombre,
                                        String codigo,
                                        String director) {
        return crear(tipo, nombre, codigo, LocalDate.now(), director);
    }

    /**
     * Detecta el tipo de unidad a partir de su código institucional.
     * <ul>
     *   <li>Códigos que empiezan con "FI" o "FAC" → FACULTAD</li>
     *   <li>Códigos que empiezan con "EIS" o "ESC" → ESCUELA</li>
     *   <li>Códigos que empiezan con "CEN" → CENTRO</li>
     *   <li>Códigos que empiezan con "INS" → INSTITUTO</li>
     * </ul>
     *
     * @param codigo código institucional
     * @return tipo inferido, o {@code null} si no se puede determinar
     */
    public static TipoUnidad inferirTipo(String codigo) {
        if (codigo == null || codigo.isBlank()) return null;
        String c = codigo.toUpperCase().trim();
        if (c.startsWith("FAC") || c.startsWith("FI")) return TipoUnidad.FACULTAD;
        if (c.startsWith("EIS") || c.startsWith("ESC")) return TipoUnidad.ESCUELA;
        if (c.startsWith("CEN")) return TipoUnidad.CENTRO;
        if (c.startsWith("INS")) return TipoUnidad.INSTITUTO;
        return null;
    }
}
