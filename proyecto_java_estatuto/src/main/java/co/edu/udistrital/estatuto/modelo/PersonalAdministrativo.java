package co.edu.udistrital.estatuto.modelo;

import java.util.Objects;

/**
 * Personal administrativo de la universidad.
 *
 * <p>Comprende empleados públicos y trabajadores oficiales adscritos a
 * dependencias administrativas de la Universidad Distrital, conforme
 * al artículo 8 del Acuerdo 004 de 2025.</p>
 *
 * @author Roberto Albeiro Pava-Díaz
 * @version 1.0
 */
public class PersonalAdministrativo extends Persona {

    /** Cargo que ocupa en la estructura administrativa. */
    private String cargo;

    /** Dependencia a la que está adscrito. */
    private String dependencia;

    /**
     * Construye una instancia de personal administrativo.
     *
     * @param nombre         nombre completo
     * @param identificacion número de identificación
     * @param correo         correo electrónico institucional
     * @param cargo          cargo que desempeña
     * @param dependencia    dependencia a la que está adscrito
     */
    public PersonalAdministrativo(String nombre, String identificacion,
                                   String correo, String cargo, String dependencia) {
        super(nombre, identificacion, correo);
        this.cargo       = cargo;
        this.dependencia = dependencia;
    }

    @Override
    public String getRol() {
        return "PersonalAdministrativo";
    }

    // ── Getters y Setters ──────────────────────────────────────────────────

    /** @return cargo que desempeña */
    public String getCargo() { return cargo; }

    /** @param cargo nuevo cargo */
    public void setCargo(String cargo) { this.cargo = cargo; }

    /** @return dependencia adscrita */
    public String getDependencia() { return dependencia; }

    /** @param dependencia nueva dependencia */
    public void setDependencia(String dependencia) { this.dependencia = dependencia; }

    @Override
    public String toString() {
        return String.format("PersonalAdm.[id=%s, nombre=%s, cargo=%s, dependencia=%s]",
                             getIdentificacion(), getNombre(), cargo, dependencia);
    }
}
