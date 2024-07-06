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
        pub fn set_cantidad_votos_emitidos(&mut self, cantidad:i16) {
            self.cantidad_votos_emitidos = cantidad;
        }
        ///setea el pocentaje de votacion 
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
        /// instancia de los votantes
        pub fn new() -> Self {
            Votantes {
                registrados: Vec::new(),
                aprobados: Vec::new(),
            }
        }
        ///setea los votantes registrados
        pub fn set_registrados(&mut self, registrados: Vec<sistema_de_votacion::sistema_de_votacion::Votante>) {
            self.registrados = registrados;
        }
        ///setea los votantes aprobados 
        pub fn set_aprobados(&mut self, aprobados: Vec<sistema_de_votacion::sistema_de_votacion::Votante>) {
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
          /// Devuelve los datos de una eleccion, solo si esta esta cerrada y finalizada.
        /// EJEMPLO
        /// ```
        /// use sistema_de_votacion::sistema_de_votacion::SistemaDeVotacion;
        /// let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
        /// ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.alice);
        /// let sistema = SistemaDeVotacion::new();
        /// let r = sistema.get_reporte_de_eleccion(3);
        /// ```
        /// 
        ///Obtiene una eleccion del sistema(se le pasa el id de una eleccion y busca si existe), tiene que estar cerrada
        #[ink(message)]
        pub fn get_reporte_de_eleccion(&self,id_eleccion:i16) -> Result<sistema_de_votacion::sistema_de_votacion::Eleccion, String>{
            let eleccion = self.sistema_de_votacion.get_reporte_de_eleccion(id_eleccion);
            let eleccion = match eleccion{
                Ok(dato) => dato,
                Err(e) => return Err(e),
            };
            Ok(eleccion)
        }

        /// Reporta los votantes registrados y aprobados de una eleccion de una eleccion cerrada, se le pasa por parametro el id de la eleccion a reportar
        /// - Parámetros:
        ///   - `id_eleccion`: El ID de la elección para la cual se desea obtener el reporte.
        ///
        /// - Devuelve:
        ///   - `OK(Votantes)`: Una instancia de la estructura `Votantes` con los votantes registrados y aprobados.
        ///   - `Err(String): Si no se encuentra el reporte de elección o el llamador no está en la lista de reportes aprobados.
        ///
        /// ## Ejemplo de uso
        ///
        /// ```
        /// // Crear una instancia del contrato de votación
        /// let mut sistema = SistemaDeVotacion::new();
        ///
        /// // Obtener el reporte de elección con ID 42
        /// let id_eleccion = 42;
        /// if let Some(votantes_reporte) = sistema.reporte_de_eleccion(id_eleccion) {
        ///     println!("Votantes registrados: {}", votantes_reporte.registrados);
        ///     println!("Votantes aprobados: {}", votantes_reporte.aprobados);
        /// } else {
        ///     println!("No se encontró el reporte de elección.");
        /// }
        /// ```
        #[ink(message)]
        pub fn reporte_de_eleccion(&self,id_eleccion:i16) ->Result<Votantes,String>{
            let reporte=self.sistema_de_votacion.get_reportes_aprobados();
            let reporte = match reporte{
                Ok(dato) => dato,
                Err(e) => return Err(e),
            };
            if !reporte.contains(&Self::env().caller()){
                return Err(String::from("El contract no tiene permiso para obtener el reporte"));
            };

            let mut votantes=Votantes::new();
            let eleccion = self.sistema_de_votacion.get_reporte_de_eleccion(id_eleccion);
            match eleccion {
                Ok(elec) => {
                    votantes.set_registrados(elec.get_postulados_a_votantes());
                    votantes.set_aprobados(elec.get_votantes());
                    return Ok(votantes)
                }
                Err(e)=> return Err(e),
            }
        }
        /// Reporta la cantidad de votos emitidos y el porcentaje de una eleccion que este cerrada, buscandola por el id pasado por parametro
        /// - Parámetros:
        ///   - `id_eleccion`: El ID de la elección para la cual se desea obtener el reporte de participación.
        ///
        /// - Devuelve:
        ///   - `Result(Participacion)`: Una instancia de la estructura `Participacion` con la cantidad de votos emitidos y el porcentaje de votación.
        ///   - `Err(String): Si no se encuentra el reporte de elección o el llamador no está en la lista de reportes aprobados.
        /// ## Ejemplo de uso
        ///
        /// Supongamos que tenemos un contrato de votación con varios reportes de elección. Aquí está un ejemplo simplificado:
        ///
        /// ```
        /// // Crear una instancia del contrato de votación
        /// let mut sistema = SistemaDeVotacion::new();
        ///
        /// // Obtener el reporte de participación para la elección con ID 42
        /// let id_eleccion = 42;
        /// if let Some(participacion_reporte) = sistema.reporte_de_participacion(id_eleccion) {
        ///     println!("Cantidad de votos emitidos: {}", participacion_reporte.cantidad_votos_emitidos);
        ///     println!("Porcentaje de votación: {:.2}%", participacion_reporte.porcentaje_de_votacion * 100.0);
        /// } else {
        ///     println!("No se encontró el reporte de participación.");
        /// }
        /// ```
        #[ink(message)]
        pub fn reporte_de_participacion(&self,id_eleccion:i16) -> Result<Participacion,String>{
            let reporte=self.sistema_de_votacion.get_reportes_aprobados();
            let reporte = match reporte{
                Ok(dato) => dato,
                Err(e) => return Err(e),
            };
            if !reporte.contains(&Self::env().caller()){
                return Err(String::from("El contract no tiene permiso para obtener el reporte"));
            };
            let mut participacion=Participacion::new();
            let eleccion = self.sistema_de_votacion.get_reporte_de_eleccion(id_eleccion);
            match eleccion {
                Ok(elec) => {
                    let cantidad=elec.get_cantidad_de_votos_emitidos();
                    participacion.set_cantidad_votos_emitidos(cantidad);
                    let porcentaje=elec.get_cantidad_de_votantes().checked_div(cantidad).unwrap();
                    participacion.set_porcentaje_de_votacion(porcentaje);
                    return Ok(participacion)
                }
                Err(e)=>return Err(e),
            }
        }
        /// Reporte de resultados, muestra los datos de los candidatos de manera descendente 
        /// - Parámetros:
        ///   - `id_eleccion`: El ID de la elección para la cual se desea obtener el reporte de resultados.
        ///
        /// - Devuelve:
        ///   - `Some(Vec<Candidato>)`: Un vector de instancias de la estructura `Candidato`, ordenado por la cantidad de votos.
        ///   - `None`: Si no se encuentra el reporte de elección o el llamador no está en la lista de reportes aprobados.
        /// 
        /// ```
        /// // Crear una instancia del contrato de votación
        /// let mut sistema = SistemaDeVotacion::new();
        ///
        /// // Obtener el reporte de resultados para la elección con ID 42
        /// let id_eleccion = 42;
        /// if let Some(resultados_reporte) = sistema.reporte_de_resultado(id_eleccion) {
        ///     for candidato in resultados_reporte {
        ///         println!("Candidato: {} - Votos: {}", candidato.nombre, candidato.cantidad_votos);
        ///     }
        /// } else {
        ///     println!("No se encontró el reporte de resultados.");
        /// }
        /// ```
        #[ink(message)]
        pub fn reporte_de_resultado(&self,id_eleccion:i16) -> Result<Vec<sistema_de_votacion::sistema_de_votacion::Candidato>,String>{
            let reporte=self.sistema_de_votacion.get_reportes_aprobados();
            let reporte = match reporte{
                Ok(dato) => dato,
                Err(e) => return Err(e),
            };
            if !reporte.contains(&Self::env().caller()){
                return Err(String::from("El contract no tiene permiso para obtener el reporte"));
            };

            let mut resultado: Vec<sistema_de_votacion::sistema_de_votacion::Candidato>=Vec::new();
            let eleccion = self.sistema_de_votacion.get_reporte_de_eleccion(id_eleccion);
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
    #[cfg(test)]
    mod tests{
        use super::*;
    //     #[ink::test]
    //     fn instanciar_reporte() {
    //     // Emular entorno de ejecución con cuentas predeterminadas
    //     let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
        
    //     // Crear una instancia del sistema de votación (simulado)
    //     let sistema_de_votacion = sistema_de_votacion::SistemaDeVotacionRef::new();

    //     // Convertir la instancia a una referencia del contrato
    //     let sistema_de_votacion_ref = SistemaDeVotacionRef::from_account_id(accounts.alice);

    //     // Instanciar el contrato Reporte con la referencia al sistema de votación
    //     let reporte = Reporte::new(sistema_de_votacion_ref);

    //     // Ejemplo de aserción: Verificar alguna propiedad del contrato Reporte
    //     assert_eq!(reporte.sistema_de_votacion, sistema_de_votacion_ref);
    // }
    }
}