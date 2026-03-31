package co.edu.udistrital.estatuto.controlador;

import co.edu.udistrital.estatuto.modelo.Facultad;
import co.edu.udistrital.estatuto.persistencia.JsonDAO;
import co.edu.udistrital.estatuto.util.Validador;
import com.google.gson.reflect.TypeToken;

import java.io.IOException;
import java.lang.reflect.Type;
import java.time.LocalDate;
import java.util.ArrayList;
import java.util.List;
import java.util.Optional;

/**
 * Controlador para la gestión de Facultades.
 *
 * <p>Orquesta las operaciones CRUD sobre {@link Facultad}, aplica validaciones
 * de negocio y delega la persistencia al {@link JsonDAO}. Actúa como
 * intermediario entre la vista y el modelo.</p>
 *
 * @author Roberto Albeiro Pava-Díaz
 * @version 1.0
 */
public class FacultadControlador {

    /** DAO para persistencia de Facultades en JSON. */
    private final JsonDAO<Facultad, String> dao;

    /** Ruta por defecto del archivo de datos. */
    private static final String RUTA_DATOS = "datos/facultades.json";

    /**
     * Construye el controlador con la ruta de datos por defecto.
     */
    public FacultadControlador() {
        Type tipo = new TypeToken<List<Facultad>>() {}.getType();
        this.dao = new JsonDAO<>(RUTA_DATOS, tipo, Facultad::getCodigo);
    }

    /**
     * Construye el controlador con una ruta de datos personalizada.
     * @param rutaDatos ruta al archivo JSON de datos
     */
    public FacultadControlador(String rutaDatos) {
        Type tipo = new TypeToken<List<Facultad>>() {}.getType();
        this.dao = new JsonDAO<>(rutaDatos, tipo, Facultad::getCodigo);
    }

    /**
     * Crea una nueva Facultad con los datos básicos.
     * Valida que el código y el nombre no estén vacíos y que el
     * código no esté ya registrado.
     *
     * @param codigo código institucional (no nulo, no vacío)
     * @param nombre nombre oficial de la facultad
     * @param decano nombre del decano o decana
     * @return la Facultad creada
     * @throws IllegalArgumentException si el código o nombre son inválidos
     * @throws IOException              si ocurre un error de persistencia
     */
    public Facultad crear(String codigo, String nombre, String decano)
            throws IOException {
        if (!Validador.esCodigoValido(codigo)) {
            throw new IllegalArgumentException(
                "Código de facultad inválido: '" + codigo + "'.");
        }
        if (!Validador.noEsVacia(nombre)) {
            throw new IllegalArgumentException("El nombre de la facultad no puede estar vacío.");
        }
        Facultad f = new Facultad(
            Validador.normalizarNombre(nombre),
            Validador.normalizarCodigo(codigo),
            LocalDate.now(),
            Validador.normalizarNombre(decano)
        );
        dao.guardar(f);
        return f;
    }

    /**
     * Crea o actualiza una Facultad (conveniente para la vista).
     * Si la Facultad ya existe, la actualiza; si no, la crea.
     *
     * @param codigo código institucional
     * @param nombre nombre oficial
     * @param decano nombre del decano
     * @return la Facultad creada o actualizada
     */
    public Facultad crearOActualizar(String codigo, String nombre, String decano) {
        try {
            Optional<Facultad> existente = dao.buscarPorId(
                Validador.normalizarCodigo(codigo));
            if (existente.isPresent()) {
                Facultad f = existente.get();
                f.setNombre(Validador.normalizarNombre(nombre));
                f.setDecano(Validador.normalizarNombre(decano));
                dao.actualizar(f);
                return f;
            } else {
                return crear(codigo, nombre, decano);
            }
        } catch (IOException e) {
            System.err.println("Error de persistencia: " + e.getMessage());
            // En caso de error de IO, retorna el objeto sin persistir
            return new Facultad(nombre, codigo, LocalDate.now(), decano);
        }
    }

    /**
     * Retorna todas las facultades registradas.
     *
     * @return lista de facultades (puede estar vacía)
     */
    public List<Facultad> listarTodos() {
        try {
            return dao.buscarTodos();
        } catch (IOException e) {
            System.err.println("Error al listar facultades: " + e.getMessage());
            return new ArrayList<>();
        }
    }

    /**
     * Busca una facultad por su código.
     *
     * @param codigo código de la facultad
     * @return {@link Optional} con la facultad, o vacío si no existe
     */
    public Optional<Facultad> buscarPorCodigo(String codigo) {
        try {
            return dao.buscarPorId(Validador.normalizarCodigo(codigo));
        } catch (IOException e) {
            System.err.println("Error al buscar facultad: " + e.getMessage());
            return Optional.empty();
        }
    }

    /**
     * Actualiza los datos de una facultad existente.
     *
     * @param facultad facultad con datos actualizados
     * @throws IOException si ocurre un error de persistencia
     */
    public void actualizar(Facultad facultad) throws IOException {
        dao.actualizar(facultad);
    }

    /**
     * Elimina una facultad por su código.
     *
     * @param codigo código de la facultad a eliminar
     * @return {@code true} si se eliminó; {@code false} si no existía
     */
    public boolean eliminar(String codigo) {
        try {
            return dao.eliminar(Validador.normalizarCodigo(codigo));
        } catch (IOException e) {
            System.err.println("Error al eliminar facultad: " + e.getMessage());
            return false;
        }
    }

    /**
     * Busca facultades cuyo nombre contiene el término dado (búsqueda parcial).
     *
     * @param termino término de búsqueda (ignorar mayúsculas/minúsculas)
     * @return lista de facultades que coinciden
     */
    public List<Facultad> buscarPorNombre(String termino) {
        try {
            String t = termino.toLowerCase().trim();
            return dao.buscarTodos().stream()
                      .filter(f -> f.getNombre().toLowerCase().contains(t))
                      .toList();
        } catch (IOException e) {
            System.err.println("Error en búsqueda: " + e.getMessage());
            return new ArrayList<>();
        }
    }
}
