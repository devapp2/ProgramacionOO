package co.edu.udistrital.estatuto.modelo;

import java.time.LocalDate;

/**
 * Representa un Centro universitario con consejo y dirección propios,
 * conforme al artículo 30 del Acuerdo 004 de 2025.
 *
 * @author Roberto Albeiro Pava-Díaz
 * @version 1.0
 */
public class Centro extends UnidadAcademica {

    /**
     * Construye un Centro universitario.
     *
     * @param nombre        nombre oficial del Centro
     * @param codigo        código institucional
     * @param fechaCreacion fecha de creación
     * @param director      nombre del director del Centro
     */
    public Centro(String nombre, String codigo,
                  LocalDate fechaCreacion, String director) {
        super(nombre, codigo, fechaCreacion, director);
    }

    @Override
    public String getTipo() {
        return "Centro";
    }

    @Override
    public String toString() {
        return String.format("Centro[codigo=%s, nombre=%s, director=%s]",
                             getCodigo(), getNombre(), getDirector());
    }
}
