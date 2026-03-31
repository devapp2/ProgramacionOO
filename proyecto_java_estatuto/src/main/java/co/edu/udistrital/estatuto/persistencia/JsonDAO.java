package co.edu.udistrital.estatuto.persistencia;

import com.google.gson.*;
//import com.google.gson.reflect.TypeToken;

import java.io.*;
import java.lang.reflect.Type;
import java.nio.file.*;
import java.time.LocalDate;
import java.util.ArrayList;
import java.util.List;
import java.util.Optional;
import java.util.function.Function;

/**
 * Implementación de DAO con serialización JSON mediante Gson.
 *
 * <p>Persiste listas de entidades en archivos {@code .json} en el
 * directorio de datos de la aplicación. Registra adaptadores para
 * tipos no soportados nativamente por Gson ({@link LocalDate}).</p>
 *
 * @param <T>  tipo de la entidad a persistir
 * @param <ID> tipo del identificador de la entidad
 *
 * @author Roberto Albeiro Pava-Díaz
 * @version 1.0
 */
public class JsonDAO<T, ID> implements GenericDAO<T, ID> {

    /** Instancia de Gson configurada con adaptadores del dominio. */
    private final Gson gson;

    /** Ruta del archivo JSON de datos. */
    private final Path archivoJson;

    /** Tipo de la lista para Gson (p.ej., {@code new TypeToken<List<Facultad>>(){}.getType()}). */
    private final Type tipoLista;

    /** Función para extraer el ID de una entidad (usado en búsqueda y eliminación). */
    private final Function<T, ID> extractorId;

    /**
     * Construye un JsonDAO.
     *
     * @param rutaArchivo ruta al archivo JSON (se crea si no existe)
     * @param tipoLista   token de tipo Gson para la lista
     * @param extractorId función que extrae el ID de una entidad T
     */
    public JsonDAO(String rutaArchivo, Type tipoLista, Function<T, ID> extractorId) {
        this.archivoJson  = Path.of(rutaArchivo);
        this.tipoLista    = tipoLista;
        this.extractorId  = extractorId;
        this.gson         = new GsonBuilder()
            .registerTypeAdapter(LocalDate.class,
                (JsonSerializer<LocalDate>) (src, t, ctx) ->
                    new JsonPrimitive(src.toString()))
            .registerTypeAdapter(LocalDate.class,
                (JsonDeserializer<LocalDate>) (json, t, ctx) ->
                    LocalDate.parse(json.getAsString()))
            .setPrettyPrinting()
            .create();
    }

    /**
     * Lee la lista completa desde el archivo JSON.
     *
     * @return lista de entidades (vacía si el archivo no existe o está vacío)
     * @throws IOException si ocurre un error de lectura
     */
    @Override
    public List<T> buscarTodos() throws IOException {
        if (!Files.exists(archivoJson)) {
            return new ArrayList<>();
        }
        try (Reader reader = Files.newBufferedReader(archivoJson)) {
            List<T> lista = gson.fromJson(reader, tipoLista);
            return lista != null ? lista : new ArrayList<>();
        } catch (JsonSyntaxException e) {
            throw new IOException("Error al parsear el archivo JSON: " + archivoJson, e);
        }
    }

    /**
     * Guarda la lista completa en el archivo JSON.
     *
     * @param lista lista de entidades a persistir
     * @throws IOException si ocurre un error de escritura
     */
    private void guardarTodos(List<T> lista) throws IOException {
        Files.createDirectories(archivoJson.getParent());
        try (Writer writer = Files.newBufferedWriter(archivoJson)) {
            gson.toJson(lista, tipoLista, writer);
        }
    }

    @Override
    public void guardar(T entidad) throws IOException {
        List<T> lista = buscarTodos();
        ID id = extractorId.apply(entidad);
        boolean existe = lista.stream()
            .anyMatch(e -> extractorId.apply(e).equals(id));
        if (existe) {
            throw new IllegalArgumentException(
                "Ya existe una entidad con el identificador: " + id);
        }
        lista.add(entidad);
        guardarTodos(lista);
    }

    @Override
    public Optional<T> buscarPorId(ID id) throws IOException {
        return buscarTodos().stream()
            .filter(e -> extractorId.apply(e).equals(id))
            .findFirst();
    }

    @Override
    public void actualizar(T entidad) throws IOException {
        List<T> lista = buscarTodos();
        ID id = extractorId.apply(entidad);
        boolean encontrado = false;
        for (int i = 0; i < lista.size(); i++) {
            if (extractorId.apply(lista.get(i)).equals(id)) {
                lista.set(i, entidad);
                encontrado = true;
                break;
            }
        }
        if (!encontrado) {
            throw new IllegalArgumentException(
                "No existe entidad con el identificador: " + id);
        }
        guardarTodos(lista);
    }

    @Override
    public boolean eliminar(ID id) throws IOException {
        List<T> lista = buscarTodos();
        boolean eliminado = lista.removeIf(e -> extractorId.apply(e).equals(id));
        if (eliminado) {
            guardarTodos(lista);
        }
        return eliminado;
    }
}
