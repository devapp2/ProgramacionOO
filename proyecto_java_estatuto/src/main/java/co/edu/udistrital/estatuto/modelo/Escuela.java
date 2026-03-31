package co.edu.udistrital.estatuto.modelo;

import java.time.LocalDate;
import java.util.ArrayList;
import java.util.Collections;
import java.util.List;
import java.util.Objects;
import java.util.Optional;

/**
 * Representa una Escuela adscrita a una Facultad.
 *
 * <p>Todos los docentes de la universidad pertenecen a una Escuela,
 * conforme al artículo 28 del Acuerdo 004 de 2025. Las Escuelas
 * organizan los Comités Académicos Básicos de Área (CABA) que
 * articulan docencia e investigación.</p>
 *
 * @author Roberto Albeiro Pava-Díaz
 * @version 1.0
 */
public class Escuela extends UnidadAcademica {

    /** Docentes adscritos a esta Escuela. */
    private final List<Docente> docentes;

    /** Comités Académicos Básicos de Área de esta Escuela. */
    private final List<Caba> cabas;

    /**
     * Construye una Escuela con los datos básicos.
     *
     * @param nombre        nombre oficial
     * @param codigo        código institucional
     * @param fechaCreacion fecha de creación
     * @param director      nombre del director de Escuela
     */
    public Escuela(String nombre, String codigo,
                   LocalDate fechaCreacion, String director) {
        super(nombre, codigo, fechaCreacion, director);
        this.docentes = new ArrayList<>();
        this.cabas    = new ArrayList<>();
    }

    @Override
    public String getTipo() {
        return "Escuela";
    }

    /**
     * Adscribe un docente a esta Escuela.
     * No se permite duplicados (por número de identificación).
     *
     * @param docente docente a adscribir
     * @return {@code true} si se agregó; {@code false} si ya estaba adscrito
     * @throws NullPointerException si el docente es nulo
     */
    public boolean agregarDocente(Docente docente) {
        Objects.requireNonNull(docente, "El docente no puede ser nulo.");
        if (docentes.contains(docente)) {
            return false;
        }
        return docentes.add(docente);
    }

    /**
     * Retira un docente de la Escuela por su número de identificación.
     *
     * @param identificacion número de identificación del docente
     * @return {@code true} si se retiró
     */
    public boolean retirarDocente(String identificacion) {
        return docentes.removeIf(d -> d.getIdentificacion().equals(identificacion));
    }

    /**
     * Busca un docente por su número de identificación.
     *
     * @param identificacion número de identificación
     * @return {@code Optional} con el docente, o vacío si no existe
     */
    public Optional<Docente> buscarDocente(String identificacion) {
        return docentes.stream()
                       .filter(d -> d.getIdentificacion().equals(identificacion))
                       .findFirst();
    }

    /**
     * Agrega un CABA a esta Escuela.
     *
     * @param caba CABA a agregar
     * @return {@code true} si se agregó
     */
    public boolean agregarCaba(Caba caba) {
        Objects.requireNonNull(caba, "El CABA no puede ser nulo.");
        if (cabas.contains(caba)) {
            return false;
        }
        return cabas.add(caba);
    }

    /**
     * Retorna la lista no modificable de docentes.
     * @return lista de docentes adscritos
     */
    public List<Docente> getDocentes() {
        return Collections.unmodifiableList(docentes);
    }

    /**
     * Retorna la lista no modificable de CABAs.
     * @return lista de CABAs
     */
    public List<Caba> getCabas() {
        return Collections.unmodifiableList(cabas);
    }

    /**
     * Retorna el número de docentes adscritos.
     * @return número de docentes
     */
    public int getNumeroDocentes() {
        return docentes.size();
    }

    @Override
    public String toString() {
        return String.format("Escuela[codigo=%s, nombre=%s, director=%s, docentes=%d]",
                             getCodigo(), getNombre(), getDirector(), docentes.size());
    }
}
