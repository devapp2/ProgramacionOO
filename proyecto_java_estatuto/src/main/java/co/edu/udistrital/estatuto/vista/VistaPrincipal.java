package co.edu.udistrital.estatuto.vista;

import co.edu.udistrital.estatuto.controlador.DocenteControlador;
import co.edu.udistrital.estatuto.controlador.EstudianteControlador;
import co.edu.udistrital.estatuto.controlador.FacultadControlador;
import co.edu.udistrital.estatuto.controlador.ProgramaControlador;
import co.edu.udistrital.estatuto.modelo.*;
import javafx.application.Application;
import javafx.beans.property.SimpleIntegerProperty;
import javafx.beans.property.SimpleStringProperty;
import javafx.collections.FXCollections;
import javafx.collections.ObservableList;
import javafx.geometry.Insets;
import javafx.geometry.Pos;
import javafx.scene.Scene;
import javafx.scene.control.*;
import javafx.scene.layout.*;
import javafx.stage.Stage;

import java.util.Optional;

/**
 * Vista principal de la aplicación JavaFX.
 *
 * <p>Organiza la interfaz en un {@link TabPane} con pestañas para las
 * entidades principales del dominio: Facultades, Docentes, Estudiantes
 * y Programas Académicos. Cada pestaña contiene una tabla de resultados
 * y botones para operaciones CRUD.</p>
 *
 * <p>La vista no accede directamente al modelo; toda operación se
 * delega al controlador correspondiente, respetando el patrón MVC.</p>
 *
 * @author Roberto Albeiro Pava-Díaz
 * @version 1.0
 */
public class VistaPrincipal extends Application {

    // ── Controladores ──────────────────────────────────────────────────────
    private final FacultadControlador  ctrlFacultad  = new FacultadControlador();
    private final DocenteControlador   ctrlDocente   = new DocenteControlador();
    private final EstudianteControlador ctrlEstudiante = new EstudianteControlador();
    private final ProgramaControlador  ctrlPrograma  = new ProgramaControlador();

    // ── Datos observables para las tablas ──────────────────────────────────
    private ObservableList<Facultad>         datosFacultades;
    private ObservableList<Docente>          datosDocentes;
    private ObservableList<Estudiante>       datosEstudiantes;
    private ObservableList<ProgramaAcademico> datosProgramas;

    @Override
    public void start(Stage stage) {
        // Cargar datos iniciales
        datosFacultades  = FXCollections.observableArrayList(ctrlFacultad.listarTodos());
        datosDocentes    = FXCollections.observableArrayList(ctrlDocente.listarTodos());
        datosEstudiantes = FXCollections.observableArrayList(ctrlEstudiante.listarTodos());
        datosProgramas   = FXCollections.observableArrayList(ctrlPrograma.listarTodos());

        // Estructura principal: BorderPane
        BorderPane raiz = new BorderPane();
        raiz.setTop(crearMenuBar(stage));
        raiz.setCenter(crearTabPane());
        raiz.setBottom(crearBarraEstado());

        Scene escena = new Scene(raiz, 1100, 720);
        stage.setTitle("Sistema de Información Estatutario — Universidad Distrital");
        stage.setScene(escena);
        stage.setMinWidth(800);
        stage.setMinHeight(500);
        stage.show();
    }

    // ── Menú principal ──────────────────────────────────────────────────────

    private MenuBar crearMenuBar(Stage stage) {
        Menu menuArchivo = new Menu("Archivo");
        MenuItem miExportar = new MenuItem("Exportar datos...");
        MenuItem miSalir    = new MenuItem("Salir");
        miSalir.setOnAction(e -> stage.close());
        menuArchivo.getItems().addAll(miExportar, new SeparatorMenuItem(), miSalir);

        Menu menuVer = new Menu("Ver");
        MenuItem miActualizar = new MenuItem("Actualizar tablas");
        miActualizar.setOnAction(e -> actualizarTablas());
        menuVer.getItems().add(miActualizar);

        Menu menuAyuda = new Menu("Ayuda");
        MenuItem miAcerca = new MenuItem("Acerca de...");
        miAcerca.setOnAction(e -> mostrarAcercaDe());
        menuAyuda.getItems().add(miAcerca);

        return new MenuBar(menuArchivo, menuVer, menuAyuda);
    }

    // ── TabPane principal ───────────────────────────────────────────────────

    private TabPane crearTabPane() {
        TabPane tabPane = new TabPane();
        tabPane.getTabs().addAll(
            crearPestaniaFacultades(),
            crearPestaniaDocentes(),
            crearPestaniaEstudiantes(),
            crearPestaniaProgramas()
        );
        return tabPane;
    }

    // ── Pestaña: Facultades ─────────────────────────────────────────────────

    @SuppressWarnings("unchecked")
    private Tab crearPestaniaFacultades() {
        Tab tab = new Tab("Facultades");
        tab.setClosable(false);

        TableView<Facultad> tabla = new TableView<>(datosFacultades);

        TableColumn<Facultad, String> colCodigo = new TableColumn<>("Código");
        colCodigo.setCellValueFactory(c -> new SimpleStringProperty(c.getValue().getCodigo()));
        colCodigo.setPrefWidth(100);

        TableColumn<Facultad, String> colNombre = new TableColumn<>("Nombre");
        colNombre.setCellValueFactory(c -> new SimpleStringProperty(c.getValue().getNombre()));
        colNombre.setPrefWidth(300);

        TableColumn<Facultad, String> colDecano = new TableColumn<>("Decano/a");
        colDecano.setCellValueFactory(c -> new SimpleStringProperty(c.getValue().getDecano()));
        colDecano.setPrefWidth(250);

        TableColumn<Facultad, String> colEscuelas = new TableColumn<>("Escuelas");
        colEscuelas.setCellValueFactory(c ->
            new SimpleStringProperty(String.valueOf(c.getValue().getNumeroEscuelas())));
        colEscuelas.setPrefWidth(80);

        tabla.getColumns().addAll(colCodigo, colNombre, colDecano, colEscuelas);

        // Barra de búsqueda
        TextField tfBusqueda = new TextField();
        tfBusqueda.setPromptText("Buscar por nombre...");
        tfBusqueda.textProperty().addListener((obs, oldVal, newVal) -> {
            if (newVal.isBlank()) {
                datosFacultades.setAll(ctrlFacultad.listarTodos());
            } else {
                datosFacultades.setAll(ctrlFacultad.buscarPorNombre(newVal));
            }
        });

        // Botones CRUD
        Button btnNuevo    = new Button("Nueva Facultad");
        Button btnEditar   = new Button("Editar");
        Button btnEliminar = new Button("Eliminar");

        btnNuevo.setOnAction(e -> mostrarFormFacultad(null));
        btnEditar.setOnAction(e -> {
            Facultad sel = tabla.getSelectionModel().getSelectedItem();
            if (sel != null) {
                mostrarFormFacultad(sel);
            } else {
                mostrarAlerta("Selección requerida", "Seleccione una facultad para editar.");
            }
        });
        btnEliminar.setOnAction(e -> {
            Facultad sel = tabla.getSelectionModel().getSelectedItem();
            if (sel != null) {
                confirmarYEliminarFacultad(sel);
            } else {
                mostrarAlerta("Selección requerida", "Seleccione una facultad para eliminar.");
            }
        });

        HBox botones   = new HBox(8, btnNuevo, btnEditar, btnEliminar);
        botones.setPadding(new Insets(5, 0, 0, 0));

        HBox barraBusqueda = new HBox(8, new Label("Buscar:"), tfBusqueda);
        barraBusqueda.setAlignment(Pos.CENTER_LEFT);
        barraBusqueda.setPadding(new Insets(5, 0, 5, 0));

        VBox contenido = new VBox(6, barraBusqueda, tabla, botones);
        contenido.setPadding(new Insets(12));
        VBox.setVgrow(tabla, Priority.ALWAYS);

        tab.setContent(contenido);
        return tab;
    }

    // ── Formulario: Facultad ────────────────────────────────────────────────

    private void mostrarFormFacultad(Facultad facultadExistente) {
        Dialog<Facultad> dialog = new Dialog<>();
        dialog.setTitle(facultadExistente == null ? "Nueva Facultad" : "Editar Facultad");
        dialog.setHeaderText("Complete los datos de la Facultad:");

        TextField tfCodigo = new TextField(facultadExistente != null
                                           ? facultadExistente.getCodigo() : "");
        TextField tfNombre = new TextField(facultadExistente != null
                                           ? facultadExistente.getNombre() : "");
        TextField tfDecano = new TextField(facultadExistente != null
                                           ? facultadExistente.getDecano() : "");

        tfCodigo.setDisable(facultadExistente != null); // No editar código en modo edición

        GridPane grid = new GridPane();
        grid.setHgap(10); grid.setVgap(10);
        grid.addRow(0, new Label("Código (*):"), tfCodigo);
        grid.addRow(1, new Label("Nombre (*):"), tfNombre);
        grid.addRow(2, new Label("Decano/a:"),   tfDecano);
        grid.setPrefWidth(400);
        tfCodigo.setPrefWidth(220);
        tfNombre.setPrefWidth(220);
        tfDecano.setPrefWidth(220);

        dialog.getDialogPane().setContent(grid);
        ButtonType btnOk     = new ButtonType("Guardar", ButtonBar.ButtonData.OK_DONE);
        ButtonType btnCancel = ButtonType.CANCEL;
        dialog.getDialogPane().getButtonTypes().addAll(btnOk, btnCancel);

        dialog.setResultConverter(bt -> {
            if (bt == btnOk) {
                return ctrlFacultad.crearOActualizar(
                    tfCodigo.getText().trim(),
                    tfNombre.getText().trim(),
                    tfDecano.getText().trim()
                );
            }
            return null;
        });

        Optional<Facultad> resultado = dialog.showAndWait();
        resultado.ifPresent(f -> {
            // Refrescar lista
            datosFacultades.setAll(ctrlFacultad.listarTodos());
        });
    }

    private void confirmarYEliminarFacultad(Facultad f) {
        Alert confirm = new Alert(Alert.AlertType.CONFIRMATION);
        confirm.setTitle("Confirmar eliminación");
        confirm.setHeaderText("¿Eliminar la facultad '" + f.getNombre() + "'?");
        confirm.setContentText("Esta acción no puede deshacerse.");
        confirm.showAndWait().ifPresent(tipo -> {
            if (tipo == ButtonType.OK) {
                if (ctrlFacultad.eliminar(f.getCodigo())) {
                    datosFacultades.remove(f);
                }
            }
        });
    }

    // ── Pestaña: Docentes ───────────────────────────────────────────────────

    @SuppressWarnings("unchecked")
    private Tab crearPestaniaDocentes() {
        Tab tab = new Tab("Docentes");
        tab.setClosable(false);

        TableView<Docente> tabla = new TableView<>(datosDocentes);

        TableColumn<Docente, String> colId     = new TableColumn<>("Identificación");
        colId.setCellValueFactory(c -> new SimpleStringProperty(c.getValue().getIdentificacion()));
        colId.setPrefWidth(140);

        TableColumn<Docente, String> colNombre = new TableColumn<>("Nombre");
        colNombre.setCellValueFactory(c -> new SimpleStringProperty(c.getValue().getNombre()));
        colNombre.setPrefWidth(280);

        TableColumn<Docente, String> colTipo   = new TableColumn<>("Vinculación");
        colTipo.setCellValueFactory(c ->
            new SimpleStringProperty(c.getValue().getTipoVinculacion().getDescripcion()));
        colTipo.setPrefWidth(130);

        TableColumn<Docente, String> colEscuela = new TableColumn<>("Escuela");
        colEscuela.setCellValueFactory(c ->
            new SimpleStringProperty(c.getValue().getEscuelaCodigo()));
        colEscuela.setPrefWidth(120);

        tabla.getColumns().addAll(colId, colNombre, colTipo, colEscuela);

        Button btnActualizar = new Button("Actualizar");
        btnActualizar.setOnAction(e ->
            datosDocentes.setAll(ctrlDocente.listarTodos()));

        HBox botones = new HBox(8, btnActualizar);
        botones.setPadding(new Insets(5, 0, 0, 0));

        VBox contenido = new VBox(6, tabla, botones);
        contenido.setPadding(new Insets(12));
        VBox.setVgrow(tabla, Priority.ALWAYS);

        tab.setContent(contenido);
        return tab;
    }

    // ── Pestaña: Estudiantes ────────────────────────────────────────────────

    @SuppressWarnings("unchecked")
    private Tab crearPestaniaEstudiantes() {
        Tab tab = new Tab("Estudiantes");
        tab.setClosable(false);

        TableView<Estudiante> tabla = new TableView<>(datosEstudiantes);

        TableColumn<Estudiante, String> colCod     = new TableColumn<>("Código");
        colCod.setCellValueFactory(c ->
            new SimpleStringProperty(c.getValue().getCodigoEstudiantil()));
        colCod.setPrefWidth(120);

        TableColumn<Estudiante, String> colNombre  = new TableColumn<>("Nombre");
        colNombre.setCellValueFactory(c ->
            new SimpleStringProperty(c.getValue().getNombre()));
        colNombre.setPrefWidth(280);

        TableColumn<Estudiante, String> colPrograma = new TableColumn<>("Programa");
        colPrograma.setCellValueFactory(c ->
            new SimpleStringProperty(c.getValue().getProgramaCodigo()));
        colPrograma.setPrefWidth(120);

        TableColumn<Estudiante, Number> colSemestre = new TableColumn<>("Semestre");
        colSemestre.setCellValueFactory(c ->
            new SimpleIntegerProperty(c.getValue().getSemestre()));
        colSemestre.setPrefWidth(90);

        TableColumn<Estudiante, String> colEstado  = new TableColumn<>("Estado");
        colEstado.setCellValueFactory(c ->
            new SimpleStringProperty(c.getValue().getEstado().getDescripcion()));
        colEstado.setPrefWidth(100);

        tabla.getColumns().addAll(colCod, colNombre, colPrograma, colSemestre, colEstado);

        Button btnActualizar = new Button("Actualizar");
        btnActualizar.setOnAction(e ->
            datosEstudiantes.setAll(ctrlEstudiante.listarTodos()));

        HBox botones = new HBox(8, btnActualizar);
        botones.setPadding(new Insets(5, 0, 0, 0));

        VBox contenido = new VBox(6, tabla, botones);
        contenido.setPadding(new Insets(12));
        VBox.setVgrow(tabla, Priority.ALWAYS);

        tab.setContent(contenido);
        return tab;
    }

    // ── Pestaña: Programas Académicos ───────────────────────────────────────

    @SuppressWarnings("unchecked")
    private Tab crearPestaniaProgramas() {
        Tab tab = new Tab("Programas Académicos");
        tab.setClosable(false);

        TableView<ProgramaAcademico> tabla = new TableView<>(datosProgramas);

        TableColumn<ProgramaAcademico, String> colCod     = new TableColumn<>("Código");
        colCod.setCellValueFactory(c ->
            new SimpleStringProperty(c.getValue().getCodigo()));
        colCod.setPrefWidth(100);

        TableColumn<ProgramaAcademico, String> colNombre  = new TableColumn<>("Nombre");
        colNombre.setCellValueFactory(c ->
            new SimpleStringProperty(c.getValue().getNombre()));
        colNombre.setPrefWidth(280);

        TableColumn<ProgramaAcademico, String> colNivel   = new TableColumn<>("Nivel");
        colNivel.setCellValueFactory(c ->
            new SimpleStringProperty(c.getValue().getNivel().getDescripcion()));
        colNivel.setPrefWidth(130);

        TableColumn<ProgramaAcademico, String> colFacultad = new TableColumn<>("Facultad");
        colFacultad.setCellValueFactory(c ->
            new SimpleStringProperty(c.getValue().getFacultadCodigo()));
        colFacultad.setPrefWidth(100);

        tabla.getColumns().addAll(colCod, colNombre, colNivel, colFacultad);

        Button btnActualizar = new Button("Actualizar");
        btnActualizar.setOnAction(e ->
            datosProgramas.setAll(ctrlPrograma.listarTodos()));

        HBox botones = new HBox(8, btnActualizar);
        botones.setPadding(new Insets(5, 0, 0, 0));

        VBox contenido = new VBox(6, tabla, botones);
        contenido.setPadding(new Insets(12));
        VBox.setVgrow(tabla, Priority.ALWAYS);

        tab.setContent(contenido);
        return tab;
    }

    // ── Barra de estado ─────────────────────────────────────────────────────

    private HBox crearBarraEstado() {
        Label lblEstado = new Label("Sistema listo.");
        HBox barra = new HBox(lblEstado);
        barra.setPadding(new Insets(4, 8, 4, 8));
        barra.setStyle("-fx-background-color: #f0f0f0; -fx-border-color: #cccccc 0 0 0;");
        return barra;
    }

    // ── Utilidades ──────────────────────────────────────────────────────────

    private void actualizarTablas() {
        datosFacultades.setAll(ctrlFacultad.listarTodos());
        datosDocentes.setAll(ctrlDocente.listarTodos());
        datosEstudiantes.setAll(ctrlEstudiante.listarTodos());
        datosProgramas.setAll(ctrlPrograma.listarTodos());
    }

    private void mostrarAlerta(String titulo, String mensaje) {
        Alert alert = new Alert(Alert.AlertType.WARNING);
        alert.setTitle(titulo);
        alert.setHeaderText(null);
        alert.setContentText(mensaje);
        alert.showAndWait();
    }

    private void mostrarAcercaDe() {
        Alert alert = new Alert(Alert.AlertType.INFORMATION);
        alert.setTitle("Acerca del Sistema");
        alert.setHeaderText("Sistema de Información Estatutario");
        alert.setContentText(
            "Caso de estudio del libro:\n"
            + "\"POO: Diseño e Implementación de un Sistema de\n"
            + "Información Académico-Administrativo para la\n"
            + "Universidad Distrital Francisco José de Caldas\"\n\n"
            + "Autor: Roberto Albeiro Pava-Díaz, Ph.D.\n"
            + "Versión: 1.0 — Bogotá, 2026"
        );
        alert.showAndWait();
    }
}
