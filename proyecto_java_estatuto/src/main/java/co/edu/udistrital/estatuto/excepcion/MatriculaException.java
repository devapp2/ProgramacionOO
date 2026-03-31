package co.edu.udistrital.estatuto.excepcion;

/**
 * Excepción lanzada cuando una operación de matrícula es inválida.
 *
 * <p>Casos de uso: matrícula en programa cerrado, doble matrícula
 * en el mismo periodo, saldo pendiente, estudiante suspendido.</p>
 *
 * @author Roberto Albeiro Pava-Díaz
 * @version 1.0
 */
public class MatriculaException extends EstatutoException {

    /**
     * Construye con el mensaje de error y el código por defecto "MAT-001".
     * @param mensaje descripción del error de matrícula
     */
    public MatriculaException(String mensaje) {
        super(mensaje, "MAT-001");
    }

    /**
     * Construye con mensaje y código de error específico.
     *
     * @param mensaje     descripción del error
     * @param codigoError código de error (e.g., "MAT-002" para duplicado)
     */
    public MatriculaException(String mensaje, String codigoError) {
        super(mensaje, codigoError);
    }

    /**
     * Construye con mensaje, código y causa.
     *
     * @param mensaje     descripción del error
     * @param codigoError código de error
     * @param causa       excepción original
     */
    public MatriculaException(String mensaje, String codigoError, Throwable causa) {
        super(mensaje, codigoError, causa);
    }
}
