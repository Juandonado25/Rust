#![cfg_attr(not(feature = "std"), no_std, no_main)]
pub use self::sistema_de_votacion::SistemaDeVotacionRef;
#[ink::contract]
pub mod sistema_de_votacion {
    use ink::prelude::string::String;
    use ink::prelude::vec::Vec;
    use ink::env::DefaultEnvironment;

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
    #[derive(scale::Decode, scale::Encode,Debug,Default,Clone)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
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
        pub fn set_registrados(&mut self, registrados: Vec<Votante>) {
            self.registrados = registrados;
        }
        pub fn set_aprobados(&mut self, aprobados: Vec<Votante>) {
            self.aprobados = aprobados;
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
        
        fn _new() -> Self {
            Self { 
                admin: Persona::new(String::from("admin"), String::from("admin"), String::from("admin"),Self::env().account_id()),
                usuarios_registrados:Vec::new(),
                elecciones:Vec::new(),
            }
        }
        
        //METODOS ADMINISTRADOR

        ///Crea una eleccion y la pushea en la estructura principal, el id de cada eleccion es la posicion en el vector +1.
        #[ink(message)]
        pub fn crear_eleccion(&mut self,cargo:String,dia_inicio:i32,mes_inicio:i32,anio_inicio:i32,dia_fin:i32,mes_fin:i32,anio_fin:i32 )->bool{
            let fecha_de_inicio = Self::timestamp(anio_inicio, mes_inicio, dia_inicio, 0, 0, 0);
            let fecha_de_inicio = match fecha_de_inicio{
                Ok(dato) =>  dato,
                Err(e) => panic!("{e}"),
            };
            let fecha_de_fin = Self::timestamp(anio_fin, mes_fin, dia_fin, 0, 0, 0);
            let fecha_de_fin = match fecha_de_fin{
                Ok(dato) =>  dato,
                Err(e) => panic!("{e}"),
            };

            if fecha_de_inicio >= fecha_de_fin{
                return false;
            }
            if Self::env().account_id()!=self.admin.accountid{
                return false
            }
            
            let elec = Eleccion::new(cargo,&fecha_de_inicio,&fecha_de_fin);
            self.elecciones.push(elec);
            for e in self.usuarios_registrados.iter_mut(){
                e.participacion.push(false);
            }
            true
        }

        fn _crear_eleccion(&mut self,cargo:String,dia_inicio:i32,mes_inicio:i32,anio_inicio:i32,dia_fin:i32,mes_fin:i32,anio_fin:i32 )->bool{
            let fecha_de_inicio = Self::timestamp(anio_inicio, mes_inicio, dia_inicio, 0, 0, 0);
            let fecha_de_inicio = match fecha_de_inicio{
                Ok(dato) =>  dato,
                Err(e) => panic!("{e}"),
            };
            let fecha_de_fin = Self::timestamp(anio_fin, mes_fin, dia_fin, 0, 0, 0);
            let fecha_de_fin = match fecha_de_fin{
                Ok(dato) =>  dato,
                Err(e) => panic!("{e}"),
            };

            if fecha_de_inicio >= fecha_de_fin{
                return false;
            }
            if Self::env().account_id()!=self.admin.accountid{
                return false
            }
            
            let elec = Eleccion::new(cargo,&fecha_de_inicio,&fecha_de_fin);
            self.elecciones.push(elec);
            for e in self.usuarios_registrados.iter_mut(){
                e.participacion.push(false);
            }
            true
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

        fn _iniciar_eleccion(&mut self,id:i16)->bool{
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

        fn _get_eleccion(&self, eleccion_id:i16)->Option<Eleccion>{
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
            && self.usuarios_registrados[id_usuario as usize].participacion[id_eleccion as usize]{
                let vot = Votante::new(self.usuarios_registrados[id_usuario as usize].datos.clone());
                let eleccion = self.elecciones.get_mut(id_eleccion.checked_sub(1).unwrap() as usize).unwrap();
                if let Some(position) = eleccion.postulados_a_votantes.iter().position(|x| *x == vot.clone()) {
                    eleccion.postulados_a_votantes.remove(position);
                    eleccion.votantes.push(vot);
                } else {
                    let can = Candidato::new(self.usuarios_registrados[id_usuario as usize].datos.clone());
                    if let Some(position) = eleccion.postulados_a_candidatos.iter().position(|x| *x == can.clone()) {
                        eleccion.candidatos.push(can);
                        eleccion.postulados_a_candidatos.remove(position);
                    }
                }
                return true;
            }
            false
        }

        fn _validar_usuario(&mut self, id_usuario:i16, id_eleccion:i16, valido:bool)->bool{
            if Self::env().account_id()==self.admin.accountid && self.existe_eleccion(id_eleccion) && self.existe_usuario(id_usuario) && valido && self.eleccion_no_empezada(id_eleccion) 
            && self.usuarios_registrados[id_usuario as usize].participacion[id_eleccion as usize]{
                let vot = Votante::new(self.usuarios_registrados[id_usuario as usize].datos.clone());
                let eleccion = self.elecciones.get_mut(id_eleccion.checked_sub(1).unwrap() as usize).unwrap();
                if let Some(position) = eleccion.postulados_a_votantes.iter().position(|x| *x == vot.clone()) {
                    eleccion.postulados_a_votantes.remove(position);
                    eleccion.votantes.push(vot);
                } else {
                    let can = Candidato::new(self.usuarios_registrados[id_usuario as usize].datos.clone());
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

        fn _get_usuario(&self, id_usuario:i16)->Option<Usuario>{
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

        fn _get_usuarios_registrados(&self)-> Vec<Usuario>{
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

        fn _get_todas_las_elecciones(&self)-> Vec<Eleccion>{
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

        fn _get_reporte_de_eleccion(&self, id_eleccion:i16)->Option<Eleccion>{
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

        fn _crear_usuario(&mut self, nombre:String, apellido:String, dni:String){
            let usuario = Usuario::new(nombre, apellido, dni,Self::env().account_id(), self.elecciones.len() as i16);
            self.usuarios_registrados.push(usuario);
        }
        
        ///si es_votante es true lo inscribe como votante, en caso contrario como candidato y ademas cambia a true
        /// la participacion del usuario en dicha eleccion para que no pueda inscribirse 2 veces en la misma eleccion.
        #[ink(message)]
        pub fn postulacion_de_usuario(&mut self, id_usuario:i16, id_eleccion:i16, es_votante:bool)->bool{
            let id = id_usuario.checked_sub(1).unwrap();
            let id_elec = id_usuario.checked_sub(1).unwrap();
            if Self::env().account_id()==self.usuarios_registrados[id_usuario as usize].datos.accountid && self.existe_eleccion(id_eleccion) && 
            self.existe_usuario(id_usuario) && self.eleccion_no_empezada(id_eleccion){
                let eleccion = self.elecciones.get_mut(id_elec as usize).unwrap();
                let usuario = self.usuarios_registrados.get_mut(id as usize).unwrap();
                if usuario.participacion[id_eleccion as usize]==false{
                    if es_votante{
                        eleccion.postulados_a_votantes.push(Votante::new(usuario.clone().datos));
                    }else{
                        eleccion.postulados_a_candidatos.push(Candidato::new(usuario.clone().datos));
                    }
                    
                    self.usuarios_registrados[id as usize].participacion[id as usize] = true;
                    return true;
                }
            }
            false
        }

        fn _postulacion_de_usuario(&mut self, id_usuario:i16, id_eleccion:i16, es_votante:bool)->bool{
            let id = id_usuario.checked_sub(1).unwrap();
            let id_elec = id_usuario.checked_sub(1).unwrap();
            if Self::env().account_id()==self.usuarios_registrados[id_usuario as usize].datos.accountid && self.existe_eleccion(id_eleccion) && 
            self.existe_usuario(id_usuario) && self.eleccion_no_empezada(id_eleccion){
                let eleccion = self.elecciones.get_mut(id_elec as usize).unwrap();
                let usuario = self.usuarios_registrados.get_mut(id as usize).unwrap();
                if usuario.participacion[id_eleccion as usize]==false{
                    if es_votante{
                        eleccion.postulados_a_votantes.push(Votante::new(usuario.clone().datos));
                    }else{
                        eleccion.postulados_a_candidatos.push(Candidato::new(usuario.clone().datos));
                    }
                    
                    self.usuarios_registrados[id as usize].participacion[id as usize] = true;
                    return true;
                }
            }
            false
            
        }

        ///el id_usuario es la posicion del votante en el vector de usuarios registrados en el sistema de votacion.
        ///el id_candidato es la posicion del candidato en el vector candidatos adentro de la eleccion.
        #[ink(message)]
        pub fn votar_canditdato(&mut self, id_usuario:i16, id_eleccion:i16, id_candidato:i16)->bool{
            let eleccion = self.elecciones.get_mut(id_eleccion.checked_sub(1).unwrap() as usize).unwrap();
            let votante = Votante::new(self.usuarios_registrados[id_usuario as usize].datos.clone());
            if Self::env().account_id()==self.usuarios_registrados[id_usuario as usize].datos.accountid && eleccion.votantes.contains(&votante)&& 
            (eleccion.candidatos.len() as i16 >= id_candidato) && (Self::env().block_timestamp()>eleccion.inicio as u64) && (Self::env().clone().block_timestamp()>eleccion.fin as u64){
                eleccion.candidatos[id_candidato as usize].cant_votos.checked_add(1).unwrap();
                return true;
            }
            false
        }

        fn _votar_canditdato(&mut self, id_usuario:i16, id_eleccion:i16, id_candidato:i16)->bool{
            let eleccion = self.elecciones.get_mut(id_eleccion.checked_sub(1).unwrap() as usize).unwrap();
            let votante = Votante::new(self.usuarios_registrados[id_usuario as usize].datos.clone());
            if Self::env().account_id()==self.usuarios_registrados[id_usuario as usize].datos.accountid && eleccion.votantes.contains(&votante)&& 
            (eleccion.candidatos.len() as i16 >= id_candidato) && (Self::env().block_timestamp()>eleccion.inicio as u64) && (Self::env().clone().block_timestamp()>eleccion.fin as u64){
                eleccion.candidatos[id_candidato as usize].cant_votos.checked_add(1).unwrap();
                return true;
            }
            false
        }
    }

    #[cfg(test)]
    mod tests {
    use super::*;
    
    #[ink::test]
    fn instanciar_sistema_de_votacion_y_probar_valores_iniciales(){
        let sistema = SistemaDeVotacion::_new();
        //Prueba el AccountId guardado con uno capturado del ambiente (entiendo que deberia ser el mismo)
        assert_eq!(sistema.admin.accountid, ink::env::test::default_accounts::<ink::env::DefaultEnvironment>().alice);
        assert_eq!(sistema.elecciones.len(),0);
        assert_eq!(sistema.usuarios_registrados.len(),0);
    }

    #[ink::test]
    fn crear_eleccion_valida(){
        let mut sistema = SistemaDeVotacion::_new();
        let account = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>().alice;
        let res = sistema._crear_eleccion(String::from("CEO de Intel"), 15, 05, 2024, 20, 05, 2024);
        let res = sistema._crear_eleccion(String::from("CEO de X"), 15, 03, 2024, 20, 03, 2024);
        assert_eq!(res,true);
        assert_eq!(sistema.elecciones.len(),2);
    }

    #[ink::test]
    fn crear_eleccion_fecha_invalida(){
        let mut sistema = SistemaDeVotacion::_new();
        let account = sistema.admin.accountid;
        let res = sistema._crear_eleccion(String::from("CEO de X"), 15, 05, 2024, 20, 03, 2024);
        assert_eq!(res,false);
    }

    #[ink::test]
    fn crear_eleccion_admin_invalido(){
        let mut sistema = SistemaDeVotacion::_new();
        let account = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>().bob;// el admin es alice, por eso uso el accountid de bob.
        let res = sistema._crear_eleccion(String::from("CEO de X"), 15, 05, 2024, 20, 03, 2024);
        assert_eq!(res,false);
    }
    }
    
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

