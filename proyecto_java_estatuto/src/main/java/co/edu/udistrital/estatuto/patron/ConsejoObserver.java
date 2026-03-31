package co.edu.udistrital.estatuto.patron;

import java.time.LocalDateTime;
import java.time.format.DateTimeFormatter;
import java.util.ArrayList;
import java.util.Collections;
import java.util.List;
import java.util.Objects;

/**
 * Interfaz Observer para los eventos del Consejo universitario.
 *
 * <p>Implementa el patrón Observer: cuando el Consejo registra un acuerdo
 * o decisión, todos los observadores suscritos son notificados automáticamente,
 * sin que el Consejo conozca los detalles de cada observador.</p>
 *
 * @author Roberto Albeiro Pava-Díaz
 * @version 1.0
 */
public interface ConsejoObserver {

    /**
     * Notifica al observador sobre un evento ocurrido en el Consejo.
     *
     * @param evento  descripción del evento (tipo y contenido del acuerdo)
     * @param consejo nombre del consejo que generó el evento
     */
    void actualizar(String evento, String consejo);

    // ── Sujeto Observable ─────────────────────────────────────────────────

    /**
     * Consejo observable. Mantiene la lista de observadores y
     * los notifica ante cada nuevo acuerdo.
     */
    final class ConsejoObservable {

        private final String                 nombre;
        private final List<ConsejoObserver>  observadores;
        private String                       ultimoAcuerdo;
        private int                          numeroAcuerdo;

        /**
         * Construye un Consejo Observable.
         * @param nombre nombre del consejo (e.g., "Consejo de Facultad de Ingeniería")
         */
        public ConsejoObservable(String nombre) {
            this.nombre        = Objects.requireNonNull(nombre);
            this.observadores  = new ArrayList<>();
            this.numeroAcuerdo = 0;
        }

        /**
         * Suscribe un observador al consejo.
         * @param obs observador a suscribir (no nulo)
         */
        public void suscribir(ConsejoObserver obs) {
            observadores.add(Objects.requireNonNull(obs));
        }

        /**
         * Cancela la suscripción de un observador.
         * @param obs observador a retirar
         */
        public void desuscribir(ConsejoObserver obs) {
            observadores.remove(obs);
        }

        /**
         * Registra un nuevo acuerdo y notifica a todos los observadores.
         * @param acuerdo texto del acuerdo aprobado
         */
        public void registrarAcuerdo(String acuerdo) {
            this.numeroAcuerdo++;
            this.ultimoAcuerdo = acuerdo;
            String evento = String.format("ACUERDO-%03d: %s", numeroAcuerdo, acuerdo);
            notificarTodos(evento);
        }

        private void notificarTodos(String evento) {
            for (ConsejoObserver obs : List.copyOf(observadores)) {
                obs.actualizar(evento, nombre);
            }
        }

        /** @return texto del último acuerdo registrado */
        public String getUltimoAcuerdo() { return ultimoAcuerdo; }

        /** @return número correlativo del último acuerdo */
        public int getNumeroAcuerdo()    { return numeroAcuerdo; }

        /** @return nombre del consejo */
        public String getNombre()        { return nombre; }
    }

    // ── Implementaciones concretas de Observer ────────────────────────────

    /**
     * Observador de auditoría: registra todos los eventos en una bitácora.
     */
    final class AuditoriaObserver implements ConsejoObserver {

        private final List<String> bitacora = new ArrayList<>();
        private static final DateTimeFormatter FMT =
            DateTimeFormatter.ofPattern("yyyy-MM-dd HH:mm:ss");

        @Override
        public void actualizar(String evento, String consejo) {
            String registro = String.format("[%s] [AUDITORIA] %s | Consejo: %s",
                                            LocalDateTime.now().format(FMT),
                                            evento, consejo);
            bitacora.add(registro);
            System.out.println(registro);
        }

        /**
         * Retorna la bitácora de auditoría (solo lectura).
         * @return lista inmutable de registros de auditoría
         */
        public List<String> getBitacora() {
            return Collections.unmodifiableList(bitacora);
        }
    }

    /**
     * Observador de notificaciones: simula el envío de correos a los miembros.
     */
    final class NotificacionObserver implements ConsejoObserver {

        private final String destinatario;

        /** @param destinatario dirección de correo del destinatario */
        public NotificacionObserver(String destinatario) {
            this.destinatario = destinatario;
        }

        @Override
        public void actualizar(String evento, String consejo) {
            System.out.printf("[NOTIF] → %s | Asunto: Nuevo acuerdo del %s | %s%n",
                              destinatario, consejo, evento);
        }
    }
}
