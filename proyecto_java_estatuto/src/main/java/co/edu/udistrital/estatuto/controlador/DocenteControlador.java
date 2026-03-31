package co.edu.udistrital.estatuto.controlador;

import co.edu.udistrital.estatuto.modelo.Docente;
import co.edu.udistrital.estatuto.modelo.TipoVinculacion;
import co.edu.udistrital.estatuto.persistencia.JsonDAO;
import co.edu.udistrital.estatuto.util.Validador;
import com.google.gson.reflect.TypeToken;

import java.io.IOException;
import java.lang.reflect.Type;
import java.util.ArrayList;
import java.util.List;
import java.util.Optional;

/**
 * Controlador para la gestión de Docentes.
 *
 * <p>Aplica las validaciones de negocio del dominio docente y
 * delega la persistencia al {@link JsonDAO}. Los docentes se
 * identifican por su número de identificación personal.</p>
 *
 * @author Roberto Albeiro Pava-Díaz
 * @version 1.0
 */
public class DocenteControlador {

    private final JsonDAO<Docente, String> dao;
    private static final String RUTA_DATOS = "datos/docentes.json";

    /**
     * Construye el controlador con la ruta de datos por defecto.
     */
    public DocenteControlador() {
        Type tipo = new TypeToken<List<Docente>>() {}.getType();
        this.dao = new JsonDAO<>(RUTA_DATOS, tipo, Docente::getIdentificacion);
    }

    /**
     * Registra un nuevo docente.
     *
     * @param nombre          nombre completo
     * @param identificacion  número de identificación
     * @param correo          correo institucional
     * @param tipo            tipo de vinculación
     * @param escuelaCodigo   código de la escuela adscrita
     * @param categoria       categoría docente (null para no-planta)
     * @return el Docente registrado
     * @throws IOException              si ocurre un error de persistencia
     * @throws IllegalArgumentException si los datos son inválidos
     */
    public Docente registrar(String nombre, String identificacion, String correo,
                              TipoVinculacion tipo, String escuelaCodigo,
                              String categoria) throws IOException {
        if (!Validador.noEsVacia(nombre)) {
            throw new IllegalArgumentException("El nombre del docente no puede estar vacío.");
        }
        if (!Validador.noEsVacia(identificacion)) {
            throw new IllegalArgumentException("La identificación no puede estar vacía.");
        }
        if (correo != null && !correo.isBlank() && !Validador.esCorreoValido(correo)) {
            throw new IllegalArgumentException("El correo no tiene un formato válido.");
        }

        Docente d = new Docente(
            Validador.normalizarNombre(nombre),
            identificacion.trim(),
            correo != null ? correo.trim() : null,
            tipo,
            escuelaCodigo,
            categoria
        );
        dao.guardar(d);
        return d;
    }

    /**
     * Lista todos los docentes.
     * @return lista de docentes registrados
     */
    public List<Docente> listarTodos() {
        try {
            return dao.buscarTodos();
        } catch (IOException e) {
            System.err.println("Error al listar docentes: " + e.getMessage());
            return new ArrayList<>();
        }
    }

    /**
     * Busca un docente por su número de identificación.
     *
     * @param identificacion número de identificación
     * @return {@link Optional} con el docente
     */
    public Optional<Docente> buscarPorId(String identificacion) {
        try {
            return dao.buscarPorId(identificacion.trim());
        } catch (IOException e) {
            return Optional.empty();
        }
    }

    /**
     * Lista los docentes de una escuela específica.
     *
     * @param escuelaCodigo código de la escuela
     * @return lista de docentes adscritos a esa escuela
     */
    public List<Docente> listarPorEscuela(String escuelaCodigo) {
        try {
            return dao.buscarTodos().stream()
                      .filter(d -> escuelaCodigo.equals(d.getEscuelaCodigo()))
                      .toList();
        } catch (IOException e) {
            return new ArrayList<>();
        }
    }

    /**
     * Lista los docentes de planta.
     * @return lista de docentes con vinculación de planta
     */
    public List<Docente> listarPlanta() {
        try {
            return dao.buscarTodos().stream()
                      .filter(Docente::esPlanta)
                      .toList();
        } catch (IOException e) {
            return new ArrayList<>();
        }
    }

    /**
     * Actualiza los datos de un docente.
     *
     * @param docente docente con datos actualizados
     * @throws IOException si ocurre un error de persistencia
     */
    public void actualizar(Docente docente) throws IOException {
        dao.actualizar(docente);
    }

    /**
     * Elimina un docente por su número de identificación.
     *
     * @param identificacion número de identificación
     * @return {@code true} si se eliminó
     */
    public boolean eliminar(String identificacion) {
        try {
            return dao.eliminar(identificacion.trim());
        } catch (IOException e) {
            System.err.println("Error al eliminar docente: " + e.getMessage());
            return false;
        }
    }
}
