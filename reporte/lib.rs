#![cfg_attr(not(feature = "std"), no_std, no_main)]
#![allow(clippy::arithmetic_side_effects)]

#[ink::contract]
mod reporte {
    use core::panic;

    use ink::prelude::string::String;
    use ink::prelude::vec::Vec;  
    use sistema_de_votacion::sistema_de_votacion::Participacion;
    use sistema_de_votacion::SistemaDeVotacionRef;
    #[ink(storage)]
    pub struct Reporte {
        sistema_de_votacion:SistemaDeVotacionRef,
    }

    
    impl Reporte {

        #[ink(constructor)]
        pub fn new(sistema_de_votacion:SistemaDeVotacionRef) -> Self {
            Self { sistema_de_votacion }
        }

        #[ink(message)]
        pub fn get_reporte_de_eleccion(&self,id_eleccion:i16) -> Option<sistema_de_votacion::sistema_de_votacion::Eleccion>{
            let eleccion = self.sistema_de_votacion.get_reporte_de_eleccion(id_eleccion);
            eleccion
        }
        #[ink(message)]
        pub fn reporte_de_eleccion(&self,id_eleccion:i16) -> sistema_de_votacion::sistema_de_votacion::Votantes{
            let eleccion = self.sistema_de_votacion.get_reporte_de_eleccion(id_eleccion);
            let mut votantes=sistema_de_votacion::sistema_de_votacion::Votantes::new();
            match eleccion {
                Some(elec) => {
                    votantes.set_registrados(elec.get_postulados_a_votantes());
                    votantes.set_aprobados(elec.get_votantes());
                }
                None=>(),
            }
            votantes
        }
        /*Reporte de Participación: Indica la cantidad de votos emitidos y el porcentaje de
        participación, una vez cerrada la elección. */
        #[ink(message)]
        pub fn reporte_de_participacion(&self,id_eleccion:i16) -> sistema_de_votacion::sistema_de_votacion::Participacion{
            let eleccion = self.sistema_de_votacion.get_reporte_de_eleccion(id_eleccion);
            let mut participacion=sistema_de_votacion::sistema_de_votacion::Participacion::new();
            match eleccion {
                Some(elec) => {
                    let cantidad=elec.get_cantidad_de_votos_emitidos();
                    participacion.set_cantidad_votos_emitidos(cantidad);
                    let porcentaje=elec.get_cantidad_de_votantes()/cantidad;
                    participacion.set_porcentaje_de_votacion(porcentaje);
                }
                None=>(),
            }
            participacion
        }
        /* Reporte de Resultado:: Muestra el número de votos recibidos por cada candidato y
        los resultados finales, una vez cerrada la elección. Este reporte deberá mostrar de
        manera descendente los votos, donde el primer candidato será el ganador de la
        elección. */
        #[ink(message)]
        pub fn reporte_de_resultado(&self,id_eleccion:i16) -> sistema_de_votacion::sistema_de_votacion::Participacion{
            let eleccion = self.sistema_de_votacion.get_reporte_de_eleccion(id_eleccion);
            let mut participacion=sistema_de_votacion::sistema_de_votacion::Participacion::new();
            match eleccion {
                Some(elec) => {
                    let cantidad=elec.get_cantidad_de_votos_emitidos();
                    participacion.set_cantidad_votos_emitidos(cantidad);
                    let porcentaje=elec.get_cantidad_de_votantes()/cantidad;
                    participacion.set_porcentaje_de_votacion(porcentaje);
                }
                None=>(),
            }
            participacion
        }
    }
}