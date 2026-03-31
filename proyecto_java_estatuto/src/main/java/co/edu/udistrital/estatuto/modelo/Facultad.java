package co.edu.udistrital.estatuto.modelo;

import java.time.LocalDate;
import java.util.ArrayList;
import java.util.Collections;
import java.util.List;
import java.util.Objects;

/**
 * Representa una Facultad de la Universidad Distrital.
 *
 * <p>Las Facultades son las unidades académico-administrativas de primer
 * nivel, conforme al artículo 24 del Acuerdo 004 de 2025. Cada Facultad
 * está encabezada por un Decano o Decana y agrupa un conjunto de Escuelas.</p>
 *
 * @author Roberto Albeiro Pava-Díaz
 * @version 1.0
 */
public class Facultad extends UnidadAcademica {

    /** Nombre completo del decano o decana vigente. */
    private String decano;

    /** Lista de escuelas adscritas a esta Facultad. */
    private final List<Escuela> escuelas;

    /**
     * Construye una Facultad con los datos mínimos requeridos.
     *
     * @param nombre        nombre oficial de la facultad
     * @param codigo        código institucional
     * @param fechaCreacion fecha de creación oficial
     * @param director      director académico general
     * @param decano        nombre del decano o decana
     * @throws NullPointerException si el decano es nulo
     */
    public Facultad(String nombre, String codigo,
                    LocalDate fechaCreacion, String director, String decano) {
        super(nombre, codigo, fechaCreacion, director);
        this.decano   = Objects.requireNonNull(decano, "El decano no puede ser nulo.");
        this.escuelas = new ArrayList<>();
    }

    /**
     * Constructor simplificado que usa el mismo valor para director y decano.
     *
     * @param nombre        nombre oficial
     * @param codigo        código institucional
     * @param fechaCreacion fecha de creación
     * @param decano        nombre del decano (se usa también como director)
     */
    public Facultad(String nombre, String codigo,
                    LocalDate fechaCreacion, String decano) {
        this(nombre, codigo, fechaCreacion, decano, decano);
    }

    @Override
    public String getTipo() {
        return "Facultad";
    }

    /**
     * Agrega una Escuela a la lista de escuelas de la Facultad.
     * No se permiten escuelas duplicadas (comparación por código).
     *
     * @param escuela escuela a agregar
     * @return {@code true} si se agregó, {@code false} si ya existía
     * @throws NullPointerException si la escuela es nula
     */
    public boolean agregarEscuela(Escuela escuela) {
        Objects.requireNonNull(escuela, "La escuela no puede ser nula.");
        if (escuelas.contains(escuela)) {
            return false;
        }
        return escuelas.add(escuela);
    }

    /**
     * Elimina una Escuela de la lista por su código.
     *
     * @param codigo código de la escuela a eliminar
     * @return {@code true} si se eliminó; {@code false} si no existía
     */
    public boolean eliminarEscuela(String codigo) {
        return escuelas.removeIf(e -> e.getCodigo().equals(codigo));
    }

    /**
     * Busca una Escuela por su código.
     *
     * @param codigo código de la escuela
     * @return la Escuela encontrada, o {@code null} si no existe
     */
    public Escuela buscarEscuela(String codigo) {
        return escuelas.stream()
                       .filter(e -> e.getCodigo().equals(codigo))
                       .findFirst()
                       .orElse(null);
    }

    /**
     * Retorna una vista no modificable de la lista de escuelas.
     * @return lista inmutable de escuelas
     */
    public List<Escuela> getEscuelas() {
        return Collections.unmodifiableList(escuelas);
    }

    /**
     * Retorna el número de escuelas adscritas.
     * @return cantidad de escuelas
     */
    public int getNumeroEscuelas() {
        return escuelas.size();
    }

    /**
     * Retorna el nombre del decano o decana.
     * @return nombre del decano
     */
    public String getDecano() {
        return decano;
    }

    /**
     * Actualiza el nombre del decano.
     * @param decano nombre del nuevo decano
     */
    public void setDecano(String decano) {
        this.decano = decano;
    }

    @Override
    public String toString() {
        return String.format("Facultad[codigo=%s, nombre=%s, decano=%s, escuelas=%d]",
                             getCodigo(), getNombre(), decano, escuelas.size());
    }
}
