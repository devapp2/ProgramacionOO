package co.edu.udistrital.estatuto.controlador;

import co.edu.udistrital.estatuto.excepcion.MatriculaException;
import co.edu.udistrital.estatuto.modelo.EstadoEstudiante;
import co.edu.udistrital.estatuto.modelo.Estudiante;
import co.edu.udistrital.estatuto.modelo.Matricula;
import co.edu.udistrital.estatuto.persistencia.JsonDAO;
import co.edu.udistrital.estatuto.util.Validador;
import com.google.gson.reflect.TypeToken;

import java.io.IOException;
import java.lang.reflect.Type;
import java.util.ArrayList;
import java.util.List;
import java.util.Optional;

/**
 * Controlador para la gestión de Estudiantes y Matrículas.
 *
 * <p>Aplica las reglas de negocio del proceso de matrícula: verifica
 * que el estudiante no esté suspendido, que no exista matrícula duplicada
 * para el mismo periodo, y que el periodo tenga formato válido.</p>
 *
 * @author Roberto Albeiro Pava-Díaz
 * @version 1.0
 */
public class EstudianteControlador {

    private final JsonDAO<Estudiante, String> daoEstudiante;
    private final JsonDAO<Matricula, String>  daoMatricula;

    private static final String RUTA_ESTUDIANTES = "datos/estudiantes.json";
    private static final String RUTA_MATRICULAS  = "datos/matriculas.json";

    /**
     * Construye el controlador con las rutas de datos por defecto.
     */
    public EstudianteControlador() {
        Type tipoEst = new TypeToken<List<Estudiante>>() {}.getType();
        Type tipoMat = new TypeToken<List<Matricula>>()  {}.getType();
        this.daoEstudiante = new JsonDAO<>(RUTA_ESTUDIANTES, tipoEst,
                                           Estudiante::getCodigoEstudiantil);
        this.daoMatricula  = new JsonDAO<>(RUTA_MATRICULAS,  tipoMat,
                                           m -> m.getEstudianteId() + "|"
                                              + m.getProgramaCodigo() + "|"
                                              + m.getPeriodo());
    }

    /**
     * Registra un nuevo estudiante.
     *
     * @param nombre             nombre completo
     * @param identificacion     número de identificación
     * @param correo             correo electrónico
     * @param codigoEstudiantil  código estudiantil
     * @param programaCodigo     código del programa
     * @return el Estudiante registrado
     * @throws IOException si ocurre error de persistencia
     */
    public Estudiante registrar(String nombre, String identificacion, String correo,
                                 String codigoEstudiantil, String programaCodigo)
            throws IOException {
        Estudiante e = new Estudiante(
            Validador.normalizarNombre(nombre),
            identificacion.trim(),
            correo,
            codigoEstudiantil.trim(),
            programaCodigo,
            1,
            EstadoEstudiante.ADMITIDO
        );
        daoEstudiante.guardar(e);
        return e;
    }

    /**
     * Procesa la matrícula de un estudiante en un programa y periodo.
     *
     * @param codigoEstudiantil código estudiantil del estudiante
     * @param programaCodigo    código del programa
     * @param periodo           periodo académico (formato "AAAA-N")
     * @return la Matrícula registrada
     * @throws MatriculaException si la matrícula no puede procesarse
     * @throws IOException        si ocurre un error de persistencia
     */
    public Matricula matricular(String codigoEstudiantil, String programaCodigo,
                                 String periodo)
            throws MatriculaException, IOException {
        // Validar formato del periodo
        if (!Validador.esPeriodoValido(periodo)) {
            throw new MatriculaException(
                "Periodo inválido: '" + periodo + "'. Use formato 'AAAA-N'.", "MAT-002");
        }

        // Verificar que el estudiante existe
        Optional<Estudiante> optEst = daoEstudiante.buscarPorId(codigoEstudiantil);
        if (optEst.isEmpty()) {
            throw new MatriculaException(
                "Estudiante '" + codigoEstudiantil + "' no encontrado.", "MAT-003");
        }

        Estudiante est = optEst.get();

        // Verificar que el estudiante no está suspendido
        if (EstadoEstudiante.SUSPENDIDO.equals(est.getEstado())) {
            throw new MatriculaException(
                "El estudiante '" + codigoEstudiantil + "' se encuentra suspendido.",
                "MAT-004");
        }

        // Verificar matrícula duplicada en el mismo periodo
        String idMatricula = codigoEstudiantil + "|" + programaCodigo + "|" + periodo;
        Optional<Matricula> duplicada = daoMatricula.buscarPorId(idMatricula);
        if (duplicada.isPresent()) {
            throw new MatriculaException(
                "El estudiante ya tiene matrícula registrada en el periodo " + periodo + ".",
                "MAT-005");
        }

        Matricula mat = new Matricula(codigoEstudiantil, programaCodigo, periodo,
                                      EstadoEstudiante.ACTIVO);
        daoMatricula.guardar(mat);

        // Actualizar estado del estudiante
        est.setEstado(EstadoEstudiante.ACTIVO);
        daoEstudiante.actualizar(est);

        return mat;
    }

    /**
     * Lista todos los estudiantes registrados.
     * @return lista de estudiantes
     */
    public List<Estudiante> listarTodos() {
        try {
            return daoEstudiante.buscarTodos();
        } catch (IOException e) {
            return new ArrayList<>();
        }
    }

    /**
     * Busca un estudiante por su código estudiantil.
     *
     * @param codigo código estudiantil
     * @return {@link Optional} con el estudiante
     */
    public Optional<Estudiante> buscarPorCodigo(String codigo) {
        try {
            return daoEstudiante.buscarPorId(codigo.trim());
        } catch (IOException e) {
            return Optional.empty();
        }
    }

    /**
     * Lista las matrículas de un estudiante.
     *
     * @param codigoEstudiantil código del estudiante
     * @return lista de matrículas del estudiante
     */
    public List<Matricula> listarMatriculas(String codigoEstudiantil) {
        try {
            return daoMatricula.buscarTodos().stream()
                               .filter(m -> m.getEstudianteId().equals(codigoEstudiantil))
                               .toList();
        } catch (IOException e) {
            return new ArrayList<>();
        }
    }

    /**
     * Elimina un estudiante por su código estudiantil.
     *
     * @param codigoEstudiantil código del estudiante a eliminar
     * @return {@code true} si se eliminó
     */
    public boolean eliminar(String codigoEstudiantil) {
        try {
            return daoEstudiante.eliminar(codigoEstudiantil.trim());
        } catch (IOException e) {
            return false;
        }
    }
}
