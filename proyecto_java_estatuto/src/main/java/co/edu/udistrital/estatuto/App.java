package co.edu.udistrital.estatuto;

import co.edu.udistrital.estatuto.vista.VistaPrincipal;
import javafx.application.Application;

/**
 * Punto de entrada principal del Sistema de Información Estatutario.
 *
 * <p>Lanza la aplicación JavaFX. Si JavaFX no está disponible en el
 * classpath, imprime un mensaje de error descriptivo.</p>
 *
 * <p><b>Ejecución con Maven:</b></p>
 * <pre>
 *   mvn javafx:run
 * </pre>
 *
 * <p><b>Ejecución con java directamente (requiere módulo JavaFX):</b></p>
 * <pre>
 *   java --module-path $PATH_TO_FX --add-modules javafx.controls,javafx.fxml \
 *        -jar target/sistema-estatuto-ud-1.0.0-jar-with-dependencies.jar
 * </pre>
 *
 * @author Roberto Albeiro Pava-Díaz
 * @version 1.0
 */
public class App {

    /**
     * Método principal. Delega el inicio de la aplicación a JavaFX.
     *
     * @param args argumentos de línea de comandos (no utilizados)
     */
    public static void main(String[] args) {
        System.out.println("=================================================");
        System.out.println("  Sistema de Información Estatutario");
        System.out.println("  Universidad Distrital Francisco José de Caldas");
        System.out.println("  Autor: Roberto Albeiro Pava-Díaz, Ph.D.");
        System.out.println("=================================================");

        try {
            Application.launch(VistaPrincipal.class, args);
        } catch (Exception e) {
            System.err.println("Error al iniciar JavaFX: " + e.getMessage());
            System.err.println("Asegúrese de tener JavaFX 21+ en el classpath.");
            System.err.println("Con Maven: mvn javafx:run");
            e.printStackTrace();
        }
    }
}
