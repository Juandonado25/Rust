#![cfg_attr(not(feature = "std"), no_std, no_main)]
#![allow(clippy::arithmetic_side_effects)]

#[ink::contract]
mod reporte {
    use ink::prelude::vec::Vec;  
    use sistema_de_votacion::SistemaDeVotacionRef;
    #[derive(scale::Decode, scale::Encode,Debug,Default,Clone)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub struct Participacion{
        cantidad_votos_emitidos:i16,
        porcentaje_de_votacion:i16,
    }
    impl Participacion{
        pub fn new() -> Self {
            let cantidad_votos_emitidos=0;
            let porcentaje_de_votacion=0;
            Participacion{
                cantidad_votos_emitidos,
                porcentaje_de_votacion,
            }
        }
        pub fn set_cantidad_votos_emitidos(&mut self, cantidad:i16) {
            self.cantidad_votos_emitidos = cantidad;
        }
        pub fn set_porcentaje_de_votacion(&mut self, porcentaje:i16) {
            self.porcentaje_de_votacion = porcentaje;
        }
    }
    #[derive(scale::Decode, scale::Encode,Debug,Clone)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub struct Votantes {
        registrados:Vec<sistema_de_votacion::sistema_de_votacion::Votante>,
        aprobados:Vec<sistema_de_votacion::sistema_de_votacion::Votante>,
    }
    impl Votantes{
        pub fn new() -> Self {
            Votantes {
                registrados: Vec::new(),
                aprobados: Vec::new(),
            }
        }
        pub fn set_registrados(&mut self, registrados: Vec<sistema_de_votacion::sistema_de_votacion::Votante>) {
            self.registrados = registrados;
        }
        pub fn set_aprobados(&mut self, aprobados: Vec<sistema_de_votacion::sistema_de_votacion::Votante>) {
            self.aprobados = aprobados;
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
        pub fn reporte_de_eleccion(&mut self,id_eleccion:i16) ->Option<Votantes>{
            let eleccion = self.sistema_de_votacion.get_reporte_de_eleccion(id_eleccion);
            let mut votantes=Votantes::new();
            match eleccion {
                Some(elec) => {
                    votantes.set_registrados(elec.get_postulados_a_votantes());
                    votantes.set_aprobados(elec.get_votantes());
                    return Some(votantes)
                }
                None=>None,
            }        
        }
        /*Reporte de Participación: Indica la cantidad de votos emitidos y el porcentaje de
        participación, una vez cerrada la elección. */
        #[ink(message)]
        pub fn reporte_de_participacion(&self,id_eleccion:i16) -> Option<Participacion>{
            let eleccion = self.sistema_de_votacion.get_reporte_de_eleccion(id_eleccion);
            let mut participacion=Participacion::new();
            match eleccion {
                Some(elec) => {
                    let cantidad=elec.get_cantidad_de_votos_emitidos();
                    participacion.set_cantidad_votos_emitidos(cantidad);
                    let porcentaje=elec.get_cantidad_de_votantes()/cantidad;
                    participacion.set_porcentaje_de_votacion(porcentaje);
                    return Some(participacion)
                }
                None=>return None,
            }
        }
        /* Reporte de Resultado:: Muestra el número de votos recibidos por cada candidato y
        los resultados finales, una vez cerrada la elección. Este reporte deberá mostrar de
        manera descendente los votos, donde el primer candidato será el ganador de la
        elección. */
        #[ink(message)]
        pub fn reporte_de_resultado(&self,id_eleccion:i16) -> Option<Vec<sistema_de_votacion::sistema_de_votacion::Candidato>>{
            let eleccion = self.sistema_de_votacion.get_reporte_de_eleccion(id_eleccion);
            let mut resultado: Vec<sistema_de_votacion::sistema_de_votacion::Candidato>=Vec::new();
            match eleccion {
                Some(elec) => {
                    resultado=elec.get_candidatos();
                    resultado.sort_unstable_by_key(|candi| candi.get_cantidad_votos());
                    return Some(resultado)
                }
                None=> return None,
            }
        }
    
    }
}