package co.edu.udistrital.estatuto.persistencia;

import java.io.IOException;
import java.util.List;
import java.util.Optional;

/**
 * Interfaz genérica para operaciones de acceso a datos (patrón DAO).
 *
 * <p>Define el contrato CRUD que toda implementación de repositorio
 * debe cumplir. El tipo genérico {@code T} representa la entidad del
 * dominio; {@code ID} representa el tipo de su identificador.</p>
 *
 * @param <T>  tipo de la entidad gestionada
 * @param <ID> tipo del identificador de la entidad
 *
 * @author Roberto Albeiro Pava-Díaz
 * @version 1.0
 */
public interface GenericDAO<T, ID> {

    /**
     * Persiste una nueva entidad en el repositorio.
     *
     * @param entidad entidad a guardar (no nula)
     * @throws IOException          si ocurre un error de escritura
     * @throws IllegalArgumentException si la entidad ya existe
     */
    void guardar(T entidad) throws IOException;

    /**
     * Recupera una entidad por su identificador.
     *
     * @param id identificador de la entidad
     * @return {@link Optional} con la entidad, o vacío si no existe
     * @throws IOException si ocurre un error de lectura
     */
    Optional<T> buscarPorId(ID id) throws IOException;

    /**
     * Retorna todas las entidades del repositorio.
     *
     * @return lista de entidades (puede estar vacía, nunca es nula)
     * @throws IOException si ocurre un error de lectura
     */
    List<T> buscarTodos() throws IOException;

    /**
     * Actualiza una entidad existente en el repositorio.
     * La identidad de la entidad no debe cambiar.
     *
     * @param entidad entidad con datos actualizados
     * @throws IOException          si ocurre un error de escritura
     * @throws IllegalArgumentException si la entidad no existe
     */
    void actualizar(T entidad) throws IOException;

    /**
     * Elimina una entidad por su identificador.
     *
     * @param id identificador de la entidad a eliminar
     * @return {@code true} si se eliminó; {@code false} si no existía
     * @throws IOException si ocurre un error de escritura
     */
    boolean eliminar(ID id) throws IOException;

    /**
     * Retorna el número de entidades en el repositorio.
     *
     * @return número de entidades
     * @throws IOException si ocurre un error de lectura
     */
    default long contarTodos() throws IOException {
        return buscarTodos().size();
    }
}
