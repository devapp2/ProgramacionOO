package co.edu.udistrital.estatuto.modelo;

import java.time.LocalDate;

/**
 * Representa un Instituto universitario con consejo y dirección propios,
 * conforme al artículo 32 del Acuerdo 004 de 2025.
 *
 * @author Roberto Albeiro Pava-Díaz
 * @version 1.0
 */
public class Instituto extends UnidadAcademica {

    /**
     * Construye un Instituto universitario.
     *
     * @param nombre        nombre oficial del Instituto
     * @param codigo        código institucional
     * @param fechaCreacion fecha de creación
     * @param director      nombre del director del Instituto
     */
    public Instituto(String nombre, String codigo,
                     LocalDate fechaCreacion, String director) {
        super(nombre, codigo, fechaCreacion, director);
    }

    @Override
    public String getTipo() {
        return "Instituto";
    }

    @Override
    public String toString() {
        return String.format("Instituto[codigo=%s, nombre=%s, director=%s]",
                             getCodigo(), getNombre(), getDirector());
    }
}
