package co.edu.udistrital.estatuto.excepcion;

/**
 * Excepción raíz del sistema de información estatutario.
 *
 * <p>Todas las excepciones de negocio del sistema heredan de esta clase
 * checked. Obliga a los clientes a tratar explícitamente los errores del
 * dominio, evitando que pasen silenciosamente a capas superiores.</p>
 *
 * @author Roberto Albeiro Pava-Díaz
 * @version 1.0
 */
public class EstatutoException extends Exception {

    /** Código de error para identificación programática. */
    private final String codigoError;

    /**
     * Construye una excepción con mensaje y código de error.
     *
     * @param mensaje     descripción del error
     * @param codigoError código identificador del tipo de error
     */
    public EstatutoException(String mensaje, String codigoError) {
        super(mensaje);
        this.codigoError = codigoError;
    }

    /**
     * Construye una excepción encadenando otra causa.
     *
     * @param mensaje     descripción del error
     * @param codigoError código identificador del tipo de error
     * @param causa       excepción original que provocó esta
     */
    public EstatutoException(String mensaje, String codigoError, Throwable causa) {
        super(mensaje, causa);
        this.codigoError = codigoError;
    }

    /**
     * Retorna el código de error para uso programático.
     * @return código de error
     */
    public String getCodigoError() {
        return codigoError;
    }

    @Override
    public String toString() {
        return String.format("EstatutoException[%s]: %s", codigoError, getMessage());
    }
}
