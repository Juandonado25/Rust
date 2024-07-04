#![cfg_attr(not(feature = "std"), no_std, no_main)]
pub use self::sistema_de_votacion::SistemaDeVotacionRef;
#[ink::contract]
pub mod sistema_de_votacion {
    use ink::prelude::string::String;
    use ink::prelude::vec::Vec;
    #[derive(scale::Decode, scale::Encode,Debug,Clone)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub enum Error{
        FechaInvalida,
        AdminInvalido,
    }

    #[derive(scale::Decode, scale::Encode,Debug,Default,Clone)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub struct Eleccion{
        cargo:String,//se detalla el cargo que sera elegido en esta eleccion, informacion que puede ser relevante para el reporte.
        inicio:i64,
        fin:i64,
        abierta:bool,
        finalizada:bool, //sirve para saber si esta cerrada porque finalizo o nunca empezo.
        postulados_a_votantes:Vec<Votante>,
        votantes:Vec<Votante>,
        postulados_a_candidatos:Vec<Candidato>,
        candidatos:Vec<Candidato>,
    }

    impl Eleccion{
        pub fn new(cargo:String,inicio:&i64, fin:&i64)->Self{
            Self{cargo,inicio:*inicio,fin:*fin,abierta:false,postulados_a_votantes:Vec::new(),postulados_a_candidatos:Vec::new(), finalizada:false ,votantes:Vec::new(),candidatos:Vec::new()}
        }

        pub fn get_postulados_a_votantes(&self)->Vec<Votante>{
            let postulados_votantes=self.postulados_a_votantes.clone();
            postulados_votantes
        }
        pub fn get_votantes(&self)->Vec<Votante>{
            let votantes=self.votantes.clone();
            votantes
        }
        pub fn get_cantidad_de_votantes(&self)->i16{
            self.votantes.len() as i16
        }
        pub fn get_cantidad_de_votos_emitidos(&self)->i16{
            let mut cantidad=0;
            for i in &self.candidatos{
                cantidad=i.cant_votos;
            }
            cantidad
        }
        pub fn get_candidatos(&self)->Vec<Candidato>{
            self.candidatos.clone()
        }
    }
    #[derive(scale::Decode, scale::Encode,Debug,Clone,PartialEq)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]   
    pub struct Persona{
        nombre:String,
        apellido:String,
        dni:String,
        accountid:AccountId,
    }

    impl Persona{
        fn new(nombre:String, apellido:String, dni:String, accountid:AccountId)->Self{
            Self{nombre,apellido,dni,accountid}
        }
    }

    
    #[derive(scale::Decode, scale::Encode,Debug,Clone)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    
    pub struct Usuario{
        datos:Persona,
        participacion:Vec<bool>,//vector usado para controlar si la persona esta participando de una eleccion, 
                                //debido a que el id de la eleccion se corresponde con su posicion en el vector, este seria contendria las misma longitud,
                                //si es true participa en esa eleccion, false si no. por ejemplo si pos1=true participa en la eleccion de id 1.
                                //lo hacemos para no inscribir mas de una vez al usuario en una misma eleccion,
    }
/////////////////A revisar inicializacion del vector
    impl Usuario{
        fn new(nombre:String, apellido:String, dni:String,accountid:AccountId, longitud:i16)->Self{
            Self{datos:Persona::new(nombre,apellido,dni,accountid),participacion:(0..longitud).map(|_| false).collect()}
        }
    }
    #[derive(scale::Decode, scale::Encode,Debug,Clone,PartialEq)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub struct Votante{
        dato: Persona,
        estado_del_voto: bool,//para controlar si ya voto.
    }
    impl Votante{
        pub fn new(dato:Persona)->Self{
            Self{dato,estado_del_voto:false}
        }
    }
    
    #[derive(scale::Decode, scale::Encode,Debug,Clone,PartialEq)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub struct Candidato{
        dato: Persona,
        cant_votos:i16,
    }
    impl Candidato{
        pub fn new(dato:Persona)->Self{
            Self{dato,cant_votos:0}
        }
        pub fn get_cantidad_votos(&self)->i16{
            self.cant_votos
        }
    }
    #[ink(storage)]
    pub struct SistemaDeVotacion{
        admin:Persona,
        usuarios_registrados:Vec<Usuario>,//todos los usuarios regitrados en el sistema, pueden participar de una elecion o no
        elecciones:Vec<Eleccion>,
    }
    impl SistemaDeVotacion {
        // Constructor
        #[ink(constructor)]
        pub fn new() -> Self {
            Self { 
                admin: Persona::new(String::from("admin"), String::from("admin"), String::from("admin"),Self::env().account_id()),
                usuarios_registrados:Vec::new(),
                elecciones:Vec::new(),
            }
        }
        
        //METODOS ADMINISTRADOR

        ///Crea una eleccion y la pushea en la estructura principal, el id de cada eleccion es la posicion en el vector +1.
        #[ink(message)]
        pub fn crear_eleccion(&mut self, cargo: String, dia_inicio: i32, mes_inicio: i32, anio_inicio: i32, dia_fin: i32, mes_fin: i32, anio_fin: i32) -> Result<(), String> {
            let fecha_de_inicio = Self::timestamp(anio_inicio, mes_inicio, dia_inicio, 0, 0, 0);
            let fecha_de_inicio = match fecha_de_inicio {
                Ok(dato) => dato,
                Err(_e) => return Err(String::from("No se puede convertir fecha de inicio ")),
            };
            let fecha_de_fin = Self::timestamp(anio_fin, mes_fin, dia_fin, 0, 0, 0);
            let fecha_de_fin = match fecha_de_fin {
                Ok(dato) => dato,
                Err(_e) => return Err(String::from("No se puede convertir fecha de fin ")),
                };
        
            if fecha_de_inicio >= fecha_de_fin {
                return Err(String::from("La fecha de inicio debe ser anterior a la fecha de fin"));
            }
            if Self::env().account_id() != self.admin.accountid {
                return Err(String::from("No tienes permiso para crear una elección"));
            }
        
            let elec = Eleccion::new(cargo, &fecha_de_inicio, &fecha_de_fin);
            self.elecciones.push(elec);
            for e in self.usuarios_registrados.iter_mut() {
                e.participacion.push(false);
            }
            Ok(())
        }

        fn existe_eleccion(&self,id:i16)->bool{
            if id==0{
                false
            }
            else{
                self.elecciones.len()>=id as usize
            }
        }

        fn existe_usuario(&self,id:i16)->bool{
            if id==0{
                false
            }
            else{
                self.usuarios_registrados.len()>=id as usize
            }
        }

        ///retorna true se puede inscribir un usuario a esa eleccion porque existe esta cerrada y no finalizada.
        fn eleccion_no_empezada(&self,id:i16)->bool{
            if id==0{
                false
            }
            else{
                if self.elecciones.len()>=id as usize {
                    let eleccion=self.elecciones.get(id.checked_sub(1).unwrap() as usize).unwrap();
                    if eleccion.abierta==false && eleccion.finalizada==false{
                        true
                    }
                    else{
                        false
                    }
                }
                else{
                    false
                }
            }
        }

        ///Si existe la eleccion y hay mas de un candidato la inicializa.
        #[ink(message)]
        pub fn iniciar_eleccion(&mut self,id:i16)->bool{
            if self.existe_eleccion(id) && Self::env().account_id()==self.admin.accountid{
                let eleccion=self.elecciones.get_mut(id.checked_sub(1).unwrap() as usize).unwrap();
                if eleccion.candidatos.len()>=2{
                    eleccion.abierta=true;
                    return true;
                }
            }
            false
        }

        ///Devuelve una eleccion, util para el reporte.
        #[ink(message)]
        pub fn get_eleccion(&self, eleccion_id:i16)->Option<Eleccion>{
            if Self::env().account_id()==self.admin.accountid && eleccion_id!=0 && eleccion_id<=self.elecciones.len() as i16{
                let elec = self.elecciones.get(eleccion_id.checked_sub(1).unwrap() as usize).unwrap();
                return Some(elec.clone())
            }
            None
        }

        ///retorna true si se pudo validar con exito, false en caso contrario.
        ///Valida solo si el usuario esta postulado para esa eleccion.
        #[ink(message)]
        pub fn validar_usuario(&mut self, id_usuario:i16, id_eleccion:i16, valido:bool)->bool{
            if Self::env().account_id()==self.admin.accountid && self.existe_eleccion(id_eleccion) && self.existe_usuario(id_usuario) && valido && self.eleccion_no_empezada(id_eleccion) 
            && self.usuarios_registrados[id_usuario.checked_sub(1).unwrap() as usize].participacion[id_eleccion.checked_sub(1).unwrap() as usize]{
                let vot = Votante::new(self.usuarios_registrados[id_usuario.checked_sub(1).unwrap() as usize].datos.clone());
                let eleccion = self.elecciones.get_mut(id_eleccion.checked_sub(1).unwrap() as usize).unwrap();
                if let Some(position) = eleccion.postulados_a_votantes.iter().position(|x| *x == vot.clone()) {
                    eleccion.postulados_a_votantes.remove(position);
                    eleccion.votantes.push(vot);
                } else {
                    let can = Candidato::new(self.usuarios_registrados[id_usuario.checked_sub(1).unwrap() as usize].datos.clone());
                    if let Some(position) = eleccion.postulados_a_candidatos.iter().position(|x| *x == can.clone()) {
                        eleccion.candidatos.push(can);
                        eleccion.postulados_a_candidatos.remove(position);
                    }
                }
                return true;
            }
            false
        }

        #[ink(message)]
        pub fn get_usuario(&self, id_usuario:i16)->Option<Usuario>{
            if Self::env().account_id()==self.admin.accountid && id_usuario<=self.usuarios_registrados.len() as i16{
                return Some(self.usuarios_registrados.get(id_usuario.checked_sub(1).unwrap() as usize).unwrap().clone());
            }
            None
        }

        #[ink(message)]
        pub fn get_usuarios_registrados(&self)-> Vec<Usuario>{
            if Self::env().account_id()==self.admin.accountid{
                self.usuarios_registrados.clone()
            }else{
                panic!("admin invalido")
            }
        }

        #[ink(message)]
        pub fn get_todas_las_elecciones(&self)-> Vec<Eleccion>{
            if Self::env().account_id()==self.admin.accountid{
                self.elecciones.clone()
            }else{
                panic!("admin invalido")
            }
        }

        // Devuelve los datos de una eleccion, solo si esta esta cerrada y finalizada.
        #[ink(message)]
        pub fn get_reporte_de_eleccion(&self, id_eleccion:i16)->Option<Eleccion>{
            if Self::env().account_id()==self.admin.accountid && self.existe_eleccion(id_eleccion){
                let eleccion = self.elecciones.get(id_eleccion.checked_sub(1).unwrap() as usize).unwrap();
                if eleccion.finalizada{
                    return Some(eleccion.clone())
                }
            }
            None
        }

        fn es_bisiesto(anio: i32) -> bool {
            (anio % 4 == 0 && anio % 100 != 0) || (anio % 400 == 0)
        }
        
        fn dias_en_mes(anio: i32, mes: i32) -> i32 {
            match mes {
                1 => 31,
                2 => if Self::es_bisiesto(anio) { 29 } else { 28 },
                3 => 31,
                4 => 30,
                5 => 31,
                6 => 30,
                7 => 31,
                8 => 31,
                9 => 30,
                10 => 31,
                11 => 30,
                12 => 31,
                _ => 0,
            }
        }
        
        fn dias_desde_1970_hasta_anio(anio: i32) -> Result<i32, &'static str> {
            let mut dias: i32 = 0;
            for a in 1970..anio {
                dias = dias.checked_add(if Self::es_bisiesto(a) { 366 } else { 365 })
                    .ok_or("Overflow in dias_desde_1970_hasta_anio")?;
            }
            Ok(dias)
        }
        
        fn timestamp(anio: i32, mes: i32, dia: i32, hora: i32, minuto: i32, segundo: i32) -> Result<i64, &'static str> {
            let dias_desde_1970 = Self::dias_desde_1970_hasta_anio(anio)?;
        
            let mut dias_hasta_mes: i32 = 0;
            for m in 1..mes {
                dias_hasta_mes = dias_hasta_mes.checked_add(Self::dias_en_mes(anio, m))
                    .ok_or("Overflow in dias_hasta_mes")?;
            }
        
            let dias_totales = dias_desde_1970
                .checked_add(dias_hasta_mes)
                .and_then(|v| v.checked_add(dia.checked_sub(1).ok_or("Underflow in dia").ok()?))
                .ok_or("Overflow in dias_totales")?;
                
            let segundos_totales = (dias_totales as i64)
                .checked_mul(24)
                .and_then(|v| v.checked_mul(3600))
                .and_then(|v| v.checked_add((hora as i64).checked_mul(3600).ok_or("Overflow in hora").ok()?))
                .and_then(|v| v.checked_add((minuto as i64).checked_mul(60).ok_or("Overflow in minuto").ok()?))
                .and_then(|v| v.checked_add(segundo as i64))
                .ok_or("Overflow in segundos_totales")?;
            
            Ok(segundos_totales.try_into().unwrap())
        }

        //METODOS DE USUARIO
        
        #[ink(message)]
        pub fn crear_usuario(&mut self, nombre:String, apellido:String, dni:String){
            let usuario = Usuario::new(nombre, apellido, dni,Self::env().account_id(), self.elecciones.len() as i16);
            self.usuarios_registrados.push(usuario);
        }
        
        ///si es_votante es true lo inscribe como votante, en caso contrario como candidato y ademas cambia a true
        /// la participacion del usuario en dicha eleccion para que no pueda inscribirse 2 veces en la misma eleccion.
        #[ink(message)]
        pub fn postulacion_de_usuario(&mut self, id_usuario:i16, id_eleccion:i16, es_votante:bool)->bool{
            let id_user = id_usuario.checked_sub(1).unwrap();
            let id_elec = id_eleccion.checked_sub(1).unwrap();
            if Self::env().account_id()==self.usuarios_registrados[id_user as usize].datos.accountid && self.existe_eleccion(id_eleccion) && 
            self.existe_usuario(id_usuario) && self.eleccion_no_empezada(id_eleccion){
                let eleccion = self.elecciones.get_mut(id_elec as usize).unwrap();
                let usuario = self.usuarios_registrados.get_mut(id_user as usize).unwrap();
                if usuario.participacion[id_elec as usize]==false{
                    if es_votante{
                        eleccion.postulados_a_votantes.push(Votante::new(usuario.clone().datos));
                    }else{
                        eleccion.postulados_a_candidatos.push(Candidato::new(usuario.clone().datos));
                    }
                    
                    self.usuarios_registrados[id_user as usize].participacion[id_elec as usize] = true;
                    return true;
                }
            }
            false
            
        }

        ///el id_usuario es la posicion del votante en el vector de usuarios registrados en el sistema de votacion.
        ///el id_candidato es la posicion del candidato en el vector candidatos adentro de la eleccion.
        #[ink(message)]
        pub fn votar_candidato(&mut self, id_usuario:i16, id_eleccion:i16, id_candidato:i16)->Result<(), String> {
            let eleccion = self.elecciones.get_mut(id_eleccion.checked_sub(1).unwrap() as usize).unwrap();
            let votante = Votante::new(self.usuarios_registrados[id_usuario.checked_sub(1).unwrap() as usize].datos.clone());

            if Self::env().account_id()!=self.usuarios_registrados[id_usuario.checked_sub(1).unwrap() as usize].datos.accountid {
                return Err(String::from("No tiene permiso de administrador"));
            }
            
            if !eleccion.votantes.contains(&votante){
                return Err(String::from("No contiene este votante "));
            }

            if (eleccion.candidatos.len() as i16) < id_candidato{
                return Err(String::from("no existe el candidato "));
            }
            
            if (Self::env().block_timestamp()>eleccion.inicio as u64) && (Self::env().clone().block_timestamp()>eleccion.fin as u64){
                return Err(String::from("Votacion fuera de fecha "));
            }
            
            eleccion.candidatos[id_candidato.checked_sub(1).unwrap() as usize].cant_votos = eleccion.candidatos[id_candidato.checked_sub(1).unwrap() as usize].cant_votos.checked_add(1).unwrap();
            Ok(())
        }
        
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use ink::env::test;

        // #[ink::test]
        // fn crear_eleccion_admin_invalido() {
        //     let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
        //     ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.alice);
        //     let mut sistema = SistemaDeVotacion::new();
        //     ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.bob);
        //     let res = sistema.crear_eleccion(String::from("CEO de X"), 15, 3, 2024, 20, 3, 2024);
        //     match res {
        //         Ok(()) => ink::env::debug_message("SE PUEDE "),
        //         Err(ref e) => ink::env::debug_message(&e),
        //     }
        //     assert!(matches!(res, Err(ref e) if e == "No tienes permiso para crear una elección"));
        // }

        
        #[ink::test]
        fn instanciar_sistema_de_votacion_y_probar_valores_iniciales(){
            let sistema = SistemaDeVotacion::new();
            //Prueba el AccountId guardado con uno capturado del ambiente (entiendo que deberia ser el mismo)
            assert_eq!(sistema.admin.accountid, ink::env::test::default_accounts::<ink::env::DefaultEnvironment>().alice);
            assert_eq!(sistema.elecciones.len(),0);
            assert_eq!(sistema.usuarios_registrados.len(),0);
        }
        fn instanciar_sistema_de_votacion_y_probar_valores_iniciales_otro_account_de_admin(){
            let sistema = SistemaDeVotacion::new();
            //Prueba el AccountId guardado con uno capturado del ambiente (entiendo que No deberia ser el mismo)
            assert_ne!(sistema.admin.accountid, ink::env::test::default_accounts::<ink::env::DefaultEnvironment>().bob);
            assert_eq!(sistema.elecciones.len(),0);
            assert_eq!(sistema.usuarios_registrados.len(),0);
        }

        #[ink::test]
        fn crear_eleccion_valida(){
            let mut sistema = SistemaDeVotacion::new();            
            let res = sistema.crear_eleccion(String::from("CEO de Intel"), 15, 01, 2024, 20, 02, 2024);
            let res = sistema.crear_eleccion(String::from("CEO de X"), 15, 03, 2024, 20, 04, 2024);
            assert!(res.is_ok());
            assert_eq!(sistema.elecciones.len(),2);
        }

        #[ink::test]
        fn rompre_crear_eleccion_timestamp_invalido(){
            let mut sistema = SistemaDeVotacion::new();            
            let res = sistema.crear_eleccion(String::from("CEO de Intel"), 15, 13, 1969, 20, 14, 2024);
           
        }

        #[ink::test]
        fn crear_eleccion_fecha_invalida(){
            let mut sistema = SistemaDeVotacion::new();
            let res = sistema.crear_eleccion(String::from("CEO de X"), 15, 05, 2024, 20, 03, 2024);
            assert!(matches!(res,Err(e) if e == "La fecha de inicio debe ser anterior a la fecha de fin"))
        }

        #[ink::test]
        fn crear_usuarios(){
            let mut sistema = SistemaDeVotacion::new();
            sistema.crear_usuario(String::from("Carlos"), String::from("Sanchez"),String::from("7654456"));
            sistema.crear_usuario(String::from("Pablo"), String::from("Gonzales"),String::from("1234567"));
            sistema.crear_usuario(String::from("Jose"), String::from("Peres"),String::from("1928492"));
            sistema.crear_usuario(String::from("Ana"), String::from("Erazo"),String::from("1245623"));
            sistema.crear_usuario(String::from("Maria"), String::from("Leon"),String::from("43554456"));
            assert_eq!(sistema.usuarios_registrados.len(),5);
        }

        #[ink::test]
        fn probando_acceso_con_getter_a_eleccion_en_la_posicion_deseada(){
            let mut sistema = SistemaDeVotacion::new();
            let res = sistema.crear_eleccion(String::from("CEO de Intel"), 15, 05, 2024, 20, 06, 2024);
            let res = sistema.crear_eleccion(String::from("CEO de X"), 15, 07, 2024, 20, 08, 2024);
            sistema.crear_usuario(String::from("Carlos"), String::from("Sanchez"),String::from("7654456"));
            sistema.crear_usuario(String::from("Pablo"), String::from("Gonzales"),String::from("1234567"));
            sistema.crear_usuario(String::from("Jose"), String::from("Peres"),String::from("1928492"));
            sistema.crear_usuario(String::from("Ana"), String::from("Erazo"),String::from("1245623"));
            sistema.crear_usuario(String::from("Maria"), String::from("Leon"),String::from("43554456"));
            let aux = sistema.get_eleccion(2);
            assert_eq!(aux.unwrap().cargo,sistema.elecciones[1].cargo);
        }

        #[ink::test]
        fn postulacion_de_usuario(){
            let mut sistema = SistemaDeVotacion::new();
            let res = sistema.crear_eleccion(String::from("CEO de Intel"), 15, 10, 2024, 20, 11, 2024);//elec 1
            sistema.crear_usuario(String::from("Carlos"), String::from("Sanchez"),String::from("7654456"));//user 1
            sistema.postulacion_de_usuario(1,1,false);
            assert_eq!(sistema.elecciones[0].postulados_a_candidatos[0].dato.nombre,String::from("Carlos"));
        }

        #[ink::test]
        fn vallidacion_de_usuario(){
            let mut sistema = SistemaDeVotacion::new();
            let res = sistema.crear_eleccion(String::from("CEO de Intel"), 15, 11, 2024, 20, 12, 2024);//elec 1
            sistema.crear_usuario(String::from("Carlos"), String::from("Sanchez"),String::from("7654456"));//user 1
            sistema.postulacion_de_usuario(1,1,false);
            sistema.validar_usuario(1, 1, true);
            assert_eq!(sistema.elecciones[0].candidatos[0].dato.nombre,String::from("Carlos"));
        }

        #[ink::test]
        fn probar_iniciar_votacion(){
            let mut sistema = SistemaDeVotacion::new();
            let res = sistema.crear_eleccion(String::from("CEO de Intel"), 15, 05, 2024, 20, 05, 2024);//elec 1
            let res = sistema.crear_eleccion(String::from("CEO de X"), 15, 03, 2024, 20, 03, 2024);//elec 2
            sistema.crear_usuario(String::from("Carlos"), String::from("Sanchez"),String::from("7654456"));//user 1
            sistema.postulacion_de_usuario(1,1,false);
            sistema.postulacion_de_usuario(1,2,true);
            sistema.crear_usuario(String::from("Pablo"), String::from("Gonzales"),String::from("1234567"));//user2
            sistema.postulacion_de_usuario(2,1,false);
            sistema.postulacion_de_usuario(2,2,true);
            sistema.crear_usuario(String::from("Jose"), String::from("Peres"),String::from("1928492"));//user3
            sistema.postulacion_de_usuario(3,1,true);
            sistema.postulacion_de_usuario(3,2,true);
            sistema.crear_usuario(String::from("Ana"), String::from("Erazo"),String::from("1245623"));//user4
            sistema.postulacion_de_usuario(4,1,true);
            sistema.postulacion_de_usuario(4,2,false);
            sistema.crear_usuario(String::from("Maria"), String::from("Leon"),String::from("43554456"));//user5
            sistema.postulacion_de_usuario(5,1,true);
            sistema.postulacion_de_usuario(5,2,false);
            sistema.validar_usuario(1, 1, true);
            sistema.validar_usuario(1, 2, true);
            sistema.validar_usuario(2, 1, true);
            sistema.validar_usuario(2, 2, true);
            sistema.validar_usuario(3, 1, true);
            sistema.validar_usuario(3, 2, true);
            sistema.validar_usuario(4, 1, true);
            sistema.validar_usuario(4, 2, true);
            sistema.validar_usuario(5, 1, true);
            sistema.validar_usuario(5, 2, true);
            sistema.iniciar_eleccion(1);
            assert!(sistema.elecciones[0].abierta);
        }

        #[ink::test]
        fn probar_votar(){
            let mut sistema = SistemaDeVotacion::new();
            let res = sistema.crear_eleccion(String::from("CEO de Intel"), 15, 05, 2024, 20, 05, 2024);//elec 1
            let res = sistema.crear_eleccion(String::from("CEO de X"), 15, 03, 2024, 20, 03, 2024);//elec 2
            sistema.crear_usuario(String::from("Carlos"), String::from("Sanchez"),String::from("7654456"));//user 1
            sistema.postulacion_de_usuario(1,1,false);
            sistema.postulacion_de_usuario(1,2,true);
            sistema.crear_usuario(String::from("Pablo"), String::from("Gonzales"),String::from("1234567"));//user2
            sistema.postulacion_de_usuario(2,1,false);
            sistema.postulacion_de_usuario(2,2,true);
            sistema.crear_usuario(String::from("Jose"), String::from("Peres"),String::from("1928492"));//user3
            sistema.postulacion_de_usuario(3,1,true);
            sistema.postulacion_de_usuario(3,2,true);
            sistema.crear_usuario(String::from("Ana"), String::from("Erazo"),String::from("1245623"));//user4
            sistema.postulacion_de_usuario(4,1,true);
            sistema.postulacion_de_usuario(4,2,false);
            sistema.crear_usuario(String::from("Maria"), String::from("Leon"),String::from("43554456"));//user5
            sistema.postulacion_de_usuario(5,1,true);
            sistema.postulacion_de_usuario(5,2,false);
            sistema.validar_usuario(1, 1, true);
            sistema.validar_usuario(1, 2, true);
            sistema.validar_usuario(2, 1, true);
            sistema.validar_usuario(2, 2, true);
            sistema.validar_usuario(3, 1, true);
            sistema.validar_usuario(3, 2, true);
            sistema.validar_usuario(4, 1, true);
            sistema.validar_usuario(4, 2, true);
            sistema.validar_usuario(5, 1, true);
            sistema.validar_usuario(5, 2, true);
            sistema.iniciar_eleccion(1);
            let res = sistema.votar_candidato(3, 1, 2);
                match res {
                Ok(()) => ink::env::debug_message("SE PUEDE "),
                Err(ref e) => ink::env::debug_message(&e),
            }
            assert_eq!(sistema.elecciones[0].candidatos[1].cant_votos,1);
        }

        fn probar_votar_fuera_de_fecha(){
            let mut sistema = SistemaDeVotacion::new();
            let res = sistema.crear_eleccion(String::from("CEO de Intel"), 15, 12, 2024, 20, 12, 2024);//elec 1
            let res = sistema.crear_eleccion(String::from("CEO de X"), 15, 02, 2024, 20, 02, 2024);//elec 2
            sistema.crear_usuario(String::from("Carlos"), String::from("Sanchez"),String::from("7654456"));//user 1
            sistema.postulacion_de_usuario(1,1,false);
            sistema.postulacion_de_usuario(1,2,true);
            sistema.crear_usuario(String::from("Pablo"), String::from("Gonzales"),String::from("1234567"));//user2
            sistema.postulacion_de_usuario(2,1,false);
            sistema.postulacion_de_usuario(2,2,true);
            sistema.crear_usuario(String::from("Jose"), String::from("Peres"),String::from("1928492"));//user3
            sistema.postulacion_de_usuario(3,1,true);
            sistema.postulacion_de_usuario(3,2,true);
            sistema.crear_usuario(String::from("Ana"), String::from("Erazo"),String::from("1245623"));//user4
            sistema.postulacion_de_usuario(4,1,true);
            sistema.postulacion_de_usuario(4,2,false);
            sistema.crear_usuario(String::from("Maria"), String::from("Leon"),String::from("43554456"));//user5
            sistema.postulacion_de_usuario(5,1,true);
            sistema.postulacion_de_usuario(5,2,false);
            sistema.validar_usuario(1, 1, true);
            sistema.validar_usuario(1, 2, true);
            sistema.validar_usuario(2, 1, true);
            sistema.validar_usuario(2, 2, true);
            sistema.validar_usuario(3, 1, true);
            sistema.validar_usuario(3, 2, true);
            sistema.validar_usuario(4, 1, true);
            sistema.validar_usuario(4, 2, true);
            sistema.validar_usuario(5, 1, true);
            sistema.validar_usuario(5, 2, true);
            sistema.iniciar_eleccion(1);
            let res = sistema.votar_candidato(3, 1, 2);
                match res {
                Ok(()) => ink::env::debug_message("SE PUEDE "),
                Err(ref e) => ink::env::debug_message(&e),
            }
            assert_eq!(sistema.elecciones[0].candidatos[1].cant_votos,1);
        }
    }
    //cargo tarpaulin --target-dir src/coverage --skip-clean --exclude-files = target/debug/* --out html
    
}
/*
    Preguntas del planteamiento:
        1- para registrarse como candidato se debe pedir mas datos ademas de su info personal? como años de antiguedad en la empresa o cantidad de titulos obtenidos.
        2- desborde aritmetico, como se soluciona? usar #![allow(clippy::arithmetic_side_effects)] es valido?
        
    Preguntas del deploy:


    Notas:
        !- Tener en cuenta que si la eleccion tiene un solo candidato no se va a poder inicializar y 
        en el reporte se marcara como ganador al unico candidato. si no existe ningun candidato retornara eleccion invalida.
*/