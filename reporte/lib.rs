#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod reporte {
    use ink::prelude::vec::Vec;  
    use ink::prelude::string::String; 
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
        /// crea una instancia departicipacion
        pub fn new() -> Self {
            Participacion{
                cantidad_votos_emitidos:0,
                porcentaje_de_votacion:0,
            }
        }
        /// settea la cantidad de votos 
        pub fn agregar_cantidad_votos_emitidos(&mut self, cantidad:i16) {
            self.cantidad_votos_emitidos = cantidad;
        }
        ///setea el pocentaje de votacion 
        pub fn agregar_porcentaje_de_votacion(&mut self, porcentaje:i16) {
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
        /// instancia de los votantes
        pub fn new() -> Self {
            Votantes {
                registrados: Vec::new(),
                aprobados: Vec::new(),
            }
        }
        ///setea los votantes registrados
        pub fn agregar_registrados(&mut self, registrados: Vec<sistema_de_votacion::sistema_de_votacion::Votante>) {
            self.registrados = registrados;
        }
        ///setea los votantes aprobados 
        pub fn agregar_aprobados(&mut self, aprobados: Vec<sistema_de_votacion::sistema_de_votacion::Votante>) {
            self.aprobados = aprobados;
        }
    }
    
    #[ink(storage)]
    pub struct Reporte {
        sistema_de_votacion:SistemaDeVotacionRef,
    }

    
    impl Reporte {
        /// instancia de el reporte
        #[ink(constructor)]
        pub fn new(sistema_de_votacion:SistemaDeVotacionRef) -> Self {   
            Self { sistema_de_votacion }
        }

        #[ink(message)]
        pub fn get_reporte_de_eleccion(&self,id_eleccion:i16) -> Result<sistema_de_votacion::sistema_de_votacion::Eleccion, String>{
            let eleccion = self.sistema_de_votacion.obtener_reporte_de_eleccion(id_eleccion);
            let eleccion = match eleccion{
                Ok(dato) => dato,
                Err(e) => return Err(e),
            };
            Ok(eleccion)
        }

        
        #[ink(message)]
        pub fn reporte_de_eleccion(&self,id_eleccion:i16) ->Result<Votantes,String>{
            let reporte=self.sistema_de_votacion.obtener_reportes_aprobados();
            let reporte = match reporte{
                Ok(dato) => dato,
                Err(e) => return Err(e),
            };
            if !reporte.contains(&Self::env().caller()){
                return Err(String::from("El contract no tiene permiso para obtener el reporte"));
            };

            let mut votantes=Votantes::new();
            let eleccion = self.sistema_de_votacion.obtener_reporte_de_eleccion(id_eleccion);
            match eleccion {
                Ok(elec) => {
                    votantes.agregar_registrados(elec.get_postulados_a_votantes());
                    votantes.agregar_aprobados(elec.get_votantes());
                    return Ok(votantes)
                }
                Err(e)=> return Err(e),
            }
        }


        #[ink(message)]
        pub fn reporte_de_participacion(&self,id_eleccion:i16) -> Result<Participacion,String>{
            let reporte=self.sistema_de_votacion.obtener_reportes_aprobados();
            let reporte = match reporte{
                Ok(dato) => dato,
                Err(e) => return Err(e),
            };
            if !reporte.contains(&Self::env().caller()){
                return Err(String::from("El contract no tiene permiso para obtener el reporte"));
            };
            let mut participacion=Participacion::new();
            let eleccion = self.sistema_de_votacion.obtener_reporte_de_eleccion(id_eleccion);
            match eleccion {
                Ok(elec) => {
                    let cantidad=elec.get_cantidad_de_votos_emitidos();
                    participacion.agregar_cantidad_votos_emitidos(cantidad);
                    let porcentaje:i16=elec.get_cantidad_de_votantes().checked_div(cantidad).unwrap();
                    participacion.agregar_porcentaje_de_votacion(porcentaje);
                    return Ok(participacion)
                }
                Err(e)=>return Err(e),
            }
        }
        
        #[ink(message)]
        pub fn reporte_de_resultado(&self,id_eleccion:i16) -> Result<Vec<sistema_de_votacion::sistema_de_votacion::Candidato>,String>{
            let reporte=self.sistema_de_votacion.obtener_reportes_aprobados();
            let reporte = match reporte{
                Ok(dato) => dato,
                Err(e) => return Err(e),
            };
            if !reporte.contains(&Self::env().caller()){
                return Err(String::from("El contract no tiene permiso para obtener el reporte"));
            };

            let mut resultado: Vec<sistema_de_votacion::sistema_de_votacion::Candidato>=Vec::new();
            let eleccion = self.sistema_de_votacion.obtener_reporte_de_eleccion(id_eleccion);
            match eleccion {
                Ok(elec) => {
                    resultado=elec.get_candidatos();
                    resultado.sort_unstable_by_key(|candi| candi.get_cantidad_votos());
                    return Ok(resultado)
                }
                Err(e)=> return Err(e),
            }
        }
        
    }

    /*No pudimos implementar los tests en nuestro reporte y por ende tampoco los tests de la documentacion. A pesar de todos nuestros 
      esfuerzos, todas las pruebas que intentamos no funcionaron.Nos encontramos con varios problemas, pero el más frustrante fue que al 
      usar SistemaDeVotacion::new(), se creaba un createBuilder<> que no pudimos instanciar de ninguna manera para que nos diera un 
      sistemaDeVotacionRef y con eso instanciar el Reporte. Intentamos las soluciones que nos sugeriste, así como algunas que encontramos 
      por nuestra cuenta y que nos recomendó ChatGPT, pero ninguna dio resultado. */

}