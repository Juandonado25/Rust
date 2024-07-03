#![cfg_attr(not(feature = "std"), no_std, no_main)]
#[ink::contract]
mod reporte {

use sistema_de_votacion::SistemaDeVotacionRef;
    pub struct Votantes {
        registrados:Vec<Votante>,
        aprobados:Vec<Votante>,
    }
    impl Votantes{
        pub fn new() -> Self {
            Votantes {
                registrados: Vec::new(),
                aprobados: Vec::new(),
            }
        }

    }

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
        pub fn reporte_de_eleccion(&self,id_eleccion:i16) -> Votantes{
            let eleccion = self.sistema_de_votacion.get_reporte_de_eleccion(id_eleccion);
            let votantes=Votantes::new();
            match eleccion {
                Some(elec) => {
                    votantes.registrados = elec.get_postulados_a_votantes();
                    votantes.aprobados = elec.get_votantes();
                }
                None=>(),
            }
            votantes
        }
    }
}
