package co.edu.udistrital.estatuto.controlador;

import co.edu.udistrital.estatuto.modelo.NivelPrograma;
import co.edu.udistrital.estatuto.modelo.ProgramaAcademico;
import co.edu.udistrital.estatuto.persistencia.JsonDAO;
import co.edu.udistrital.estatuto.util.Validador;
import com.google.gson.reflect.TypeToken;

import java.io.IOException;
import java.lang.reflect.Type;
import java.util.ArrayList;
import java.util.List;
import java.util.Optional;

/**
 * Controlador para la gestión de Programas Académicos.
 *
 * @author Roberto Albeiro Pava-Díaz
 * @version 1.0
 */
public class ProgramaControlador {

    private final JsonDAO<ProgramaAcademico, String> dao;
    private static final String RUTA_DATOS = "datos/programas.json";

    /**
     * Construye el controlador con la ruta de datos por defecto.
     */
    public ProgramaControlador() {
        Type tipo = new TypeToken<List<ProgramaAcademico>>() {}.getType();
        this.dao = new JsonDAO<>(RUTA_DATOS, tipo, ProgramaAcademico::getCodigo);
    }

    /**
     * Registra un nuevo programa académico.
     *
     * @param nombre        nombre del programa
     * @param codigo        código SNIES o institucional
     * @param nivel         nivel de formación
     * @param modalidad     modalidad de oferta
     * @param facultadCodigo código de la facultad que lo ofrece
     * @return el programa registrado
     * @throws IOException si ocurre un error de persistencia
     */
    public ProgramaAcademico registrar(String nombre, String codigo,
                                        NivelPrograma nivel, String modalidad,
                                        String facultadCodigo) throws IOException {
        if (!Validador.noEsVacia(nombre)) {
            throw new IllegalArgumentException("El nombre del programa no puede estar vacío.");
        }
        if (!Validador.noEsVacia(codigo)) {
            throw new IllegalArgumentException("El código del programa no puede estar vacío.");
        }
        ProgramaAcademico p = new ProgramaAcademico(
            Validador.normalizarNombre(nombre),
            codigo.trim().toUpperCase(),
            nivel,
            modalidad,
            facultadCodigo
        );
        dao.guardar(p);
        return p;
    }

    /**
     * Lista todos los programas registrados.
     * @return lista de programas académicos
     */
    public List<ProgramaAcademico> listarTodos() {
        try {
            return dao.buscarTodos();
        } catch (IOException e) {
            return new ArrayList<>();
        }
    }

    /**
     * Busca un programa por su código.
     *
     * @param codigo código del programa
     * @return {@link Optional} con el programa
     */
    public Optional<ProgramaAcademico> buscarPorCodigo(String codigo) {
        try {
            return dao.buscarPorId(codigo.trim().toUpperCase());
        } catch (IOException e) {
            return Optional.empty();
        }
    }

    /**
     * Lista programas de un nivel específico.
     *
     * @param nivel nivel de formación a filtrar
     * @return lista de programas del nivel dado
     */
    public List<ProgramaAcademico> listarPorNivel(NivelPrograma nivel) {
        try {
            return dao.buscarTodos().stream()
                      .filter(p -> nivel.equals(p.getNivel()))
                      .toList();
        } catch (IOException e) {
            return new ArrayList<>();
        }
    }

    /**
     * Lista programas ofrecidos por una facultad específica.
     *
     * @param facultadCodigo código de la facultad
     * @return lista de programas de esa facultad
     */
    public List<ProgramaAcademico> listarPorFacultad(String facultadCodigo) {
        try {
            return dao.buscarTodos().stream()
                      .filter(p -> facultadCodigo.equals(p.getFacultadCodigo()))
                      .toList();
        } catch (IOException e) {
            return new ArrayList<>();
        }
    }

    /**
     * Actualiza los datos de un programa.
     *
     * @param programa programa con datos actualizados
     * @throws IOException si ocurre un error de persistencia
     */
    public void actualizar(ProgramaAcademico programa) throws IOException {
        dao.actualizar(programa);
    }

    /**
     * Elimina un programa por su código.
     *
     * @param codigo código del programa a eliminar
     * @return {@code true} si se eliminó
     */
    public boolean eliminar(String codigo) {
        try {
            return dao.eliminar(codigo.trim().toUpperCase());
        } catch (IOException e) {
            return false;
        }
    }
}
