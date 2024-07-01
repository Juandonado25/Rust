#![cfg_attr(not(feature = "std"), no_std, no_main)]
pub use self::sistema_de_votacion::SistemaDeVotacionRef;
#[ink::contract]
pub mod sistema_de_votacion {
    use scale_info::prelude::format;
    use ink::prelude::string::String;
    use ink::prelude::vec::Vec;
    use scale_info::prelude::vec;

    #[derive(scale::Decode, scale::Encode,Debug,Default,Clone)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub struct Eleccion{
        cargo:String,//se detalla el cargo que sera elegido en esta eleccion, informacion que puede ser relevante para el reporte.
        inicio:u64,
        fin:u64,
        abierta:bool,
        finalizada:bool, //sirve para saber si esta cerrada porque finalizo o nunca empezo.
        postulados_a_votantes:Vec<Votante>,
        votantes:Vec<Votante>,
        postulados_a_candidatos:Vec<Candidato>,
        candidatos:Vec<Candidato>,
    }

    impl Eleccion{
        pub fn new(cargo:String,inicio:&u64, fin:&u64)->Self{
            Self{cargo,inicio:*inicio,fin:*fin,abierta:false,postulados_a_votantes:Vec::new(),postulados_a_candidatos:Vec::new(), finalizada:false ,votantes:Vec::new(),candidatos:Vec::new()}
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
        fn new(nombre:String, apellido:String, dni:String,accountid:AccountId, longitud:u8)->Self{
            Self{datos:Persona::new(nombre,apellido,dni,accountid),participacion:vec!{false;longitud as usize}}
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
        cant_votos:u8,
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

        ///Crea una eleccion y la pushea en la structura principal, el id de cada eleccion es la posicion en el vector +1.
        #[ink(message)]
        pub fn crear_eleccion(&mut self,cargo:String,dia_inicio:u16,mes_inicio:u16,anio_inicio:u16,dia_fin:u16,mes_fin:u16,anio_fin:u16 )->bool{
            let calcular = |dia:u16,mes:u16,anio:u16|->u64 {
                    let dia_string;
                    let mes_string;
                    if dia<10{
                        dia_string = format!("0{}",dia);
                    }else{
                        dia_string = format!("{}",dia);
                    }
                    if mes<10{
                        mes_string = format!("0{}",mes);
                    }else{
                        mes_string = format!("{}",mes);
                    }

                    let anio_string = format!("{}", anio);
                        
                    let mut fecha_concat = anio_string;
                    fecha_concat.push_str(&mes_string);
                    fecha_concat.push_str(&dia_string);

                    let inicio:u64 = fecha_concat.parse().unwrap();
                    let mut fecha_u64= 0;
                    match inicio.checked_sub((inicio/65536)*65536) {
                        Some(result) => fecha_u64 =  result,
                        None => println!("Desbordamiento detectado"),
                    }
                    fecha_u64
                };
            let fecha_de_inicio = calcular(dia_inicio,mes_inicio,anio_inicio);
            let fecha_de_fin = calcular(dia_fin,mes_fin,anio_fin);
            if (fecha_de_inicio < fecha_de_fin) && Self::env().account_id()==self.admin.accountid{
                let elec = Eleccion::new(cargo,&fecha_de_inicio,&fecha_de_fin);
                self.elecciones.push(elec);
                for e in self.usuarios_registrados.iter_mut(){
                    e.participacion.push(false);
                }
                true
            }else{
                panic!("Usuario invalido")
            }
        }

        fn _crear_eleccion(&mut self,cargo:String,dia_inicio:u16,mes_inicio:u16,anio_inicio:u16,dia_fin:u16,mes_fin:u16,anio_fin:u16 )->bool{
            let calcular = |dia:u16,mes:u16,anio:u16|->u64 {
                let dia_string;
                let mes_string;
                if dia<10{
                    dia_string = format!("0{}",dia);
                }else{
                    dia_string = format!("{}",dia);
                }
                if mes<10{
                    mes_string = format!("0{}",mes);
                }else{
                    mes_string = format!("{}",mes);
                }

                let anio_string = format!("{}", anio);
                    
                let fecha_concat = dia_string + &mes_string + &anio_string;
                let inicio:u64 = fecha_concat.parse().unwrap();
                let fecha_u64 = inicio - ((inicio/65536)*65536);
                fecha_u64
            };
        let fecha_de_inicio = calcular(dia_inicio,mes_inicio,anio_inicio);
        let fecha_de_fin = calcular(dia_fin,mes_fin,anio_fin);
        if (fecha_de_inicio < fecha_de_fin) && Self::env().account_id()==self.admin.accountid{
            let elec = Eleccion::new(cargo,&fecha_de_inicio,&fecha_de_fin);
            self.elecciones.push(elec);
            for e in self.usuarios_registrados.iter_mut(){
                e.participacion.push(false);
            }
            return true;
        }else{
            panic!("Usuario invalido")
        }
        }

        fn existe_eleccion(&self,id:u8)->bool{
            if id==0{
                false
            }
            else{
                self.elecciones.len()>=id as usize
            }
        }

        fn existe_usuario(&self,id:u8)->bool{
            if id==0{
                false
            }
            else{
                self.usuarios_registrados.len()>=id as usize
            }
        }

        ///retorna true se puede inscribir un usuario a esa eleccion porque existe esta cerrada y no finalizada.
        fn eleccion_no_empezada(&self,id:u8)->bool{
            if id==0{
                false
            }
            else{
                if self.elecciones.len()>=id as usize {
                    let eleccion=self.elecciones.get(id as usize -1).unwrap();
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
        pub fn iniciar_eleccion(&mut self,id:u8)->bool{
            if self.existe_eleccion(id) && Self::env().account_id()==self.admin.accountid{
                let eleccion=self.elecciones.get_mut(id as usize -1).unwrap();
                if eleccion.candidatos.len()>=2{
                    eleccion.abierta=true;
                    return true;
                }
            }
            false
        }

        fn _iniciar_eleccion(&mut self,id:u8)->bool{
            if self.existe_eleccion(id) && Self::env().account_id()==self.admin.accountid{
                let eleccion=self.elecciones.get_mut(id as usize -1).unwrap();
                if eleccion.candidatos.len()>=2{
                    eleccion.abierta=true;
                    return true;
                }
            }
            false
        }

        ///Devuelve una eleccion, util para el reporte.
        #[ink(message)]
        pub fn get_eleccion(&self, eleccion_id:u8)->Option<Eleccion>{
            if Self::env().account_id()==self.admin.accountid && eleccion_id!=0 && eleccion_id<=self.elecciones.len() as u8{
                let elec = self.elecciones.get(eleccion_id as usize -1).unwrap();
                return Some(elec.clone())
            }
            None
        }

        fn _get_eleccion(&self, eleccion_id:u8)->Option<Eleccion>{
            if Self::env().account_id()==self.admin.accountid && eleccion_id!=0 && eleccion_id<=self.elecciones.len() as u8{
                let elec = self.elecciones.get(eleccion_id as usize -1).unwrap();
                return Some(elec.clone())
            }
            None
        }

        ///retorna true si se pudo validar con exito, false en caso contrario.
        ///Valida solo si el usuario esta postulado para esa eleccion.
        #[ink(message)]
        pub fn validar_usuario(&mut self, id_usuario:u8, id_eleccion:u8, valido:bool)->bool{
            if Self::env().account_id()==self.admin.accountid && self.existe_eleccion(id_eleccion) && self.existe_usuario(id_usuario) && valido && self.eleccion_no_empezada(id_eleccion) 
            && self.usuarios_registrados[id_usuario as usize].participacion[id_eleccion as usize]{
                let vot = Votante::new(self.usuarios_registrados[id_usuario as usize].datos.clone());
                let eleccion = self.elecciones.get_mut(id_eleccion as usize -1).unwrap();
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

        fn _validar_usuario(&mut self, id_usuario:u8, id_eleccion:u8, valido:bool)->bool{
            if Self::env().account_id()==self.admin.accountid && self.existe_eleccion(id_eleccion) && self.existe_usuario(id_usuario) && valido && self.eleccion_no_empezada(id_eleccion) 
            && self.usuarios_registrados[id_usuario as usize].participacion[id_eleccion as usize]{
                let vot = Votante::new(self.usuarios_registrados[id_usuario as usize].datos.clone());
                let eleccion = self.elecciones.get_mut(id_eleccion as usize -1).unwrap();
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
        pub fn get_usuario(&self, id_usuario:u8)->Option<Usuario>{
            if Self::env().account_id()==self.admin.accountid && id_usuario<=self.usuarios_registrados.len() as u8{
                return Some(self.usuarios_registrados.get(id_usuario as usize -1).unwrap().clone());
            }
            None
        }

        fn _get_usuario(&self, id_usuario:u8)->Option<Usuario>{
            if Self::env().account_id()==self.admin.accountid && id_usuario<=self.usuarios_registrados.len() as u8{
                return Some(self.usuarios_registrados.get(id_usuario as usize -1).unwrap().clone());
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
        pub fn get_reporte_de_eleccion(&self, id_eleccion:u8)->Option<Eleccion>{
            if Self::env().account_id()==self.admin.accountid && self.existe_eleccion(id_eleccion){
                let eleccion = self.elecciones.get(id_eleccion as usize -1).unwrap();
                if eleccion.finalizada{
                    return Some(eleccion.clone())
                }
            }
            None
        }

        fn _get_reporte_de_eleccion(&self, id_eleccion:u8)->Option<Eleccion>{
            if Self::env().account_id()==self.admin.accountid && self.existe_eleccion(id_eleccion){
                let eleccion = self.elecciones.get(id_eleccion as usize -1).unwrap();
                if eleccion.finalizada{
                    return Some(eleccion.clone())
                }
            }
            None
        }

        //METODOS DE USUARIO
        
        #[ink(message)]
        pub fn crear_usuario(&mut self, nombre:String, apellido:String, dni:String){
            let usuario = Usuario::new(nombre, apellido, dni,Self::env().account_id(), self.elecciones.len() as u8);
            self.usuarios_registrados.push(usuario);
        }

        fn _crear_usuario(&mut self, nombre:String, apellido:String, dni:String){
            let usuario = Usuario::new(nombre, apellido, dni,Self::env().account_id(), self.elecciones.len() as u8);
            self.usuarios_registrados.push(usuario);
        }
        
        ///si es_votante es true lo inscribe como votante, en caso contrario como candidato y ademas cambia a true
        /// la participacion del usuario en dicha eleccion para que no pueda inscribirse 2 veces en la misma eleccion.
        #[ink(message)]
        pub fn postulacion_de_usuario(&mut self, id_usuario:u8, id_eleccion:u8, es_votante:bool)->bool{
            if Self::env().account_id()==self.usuarios_registrados[id_usuario as usize].datos.accountid && self.existe_eleccion(id_eleccion) && 
            self.existe_usuario(id_usuario) && self.eleccion_no_empezada(id_eleccion){
                let eleccion = self.elecciones.get_mut(id_eleccion as usize -1).unwrap();
                let usuario = self.usuarios_registrados.get_mut(id_usuario as usize -1).unwrap();
                if usuario.participacion[id_eleccion as usize]==false{
                    if es_votante{
                        eleccion.postulados_a_votantes.push(Votante::new(usuario.clone().datos));
                    }else{
                        eleccion.postulados_a_candidatos.push(Candidato::new(usuario.clone().datos));
                    }
                    self.usuarios_registrados[id_usuario as usize -1].participacion[id_eleccion as usize -1] = true;
                    return true;
                }
            }
            false
            
        }

        fn _postulacion_de_usuario(&mut self, id_usuario:u8, id_eleccion:u8, es_votante:bool)->bool{
            if Self::env().account_id()==self.usuarios_registrados[id_usuario as usize].datos.accountid && self.existe_eleccion(id_eleccion) && 
            self.existe_usuario(id_usuario) && self.eleccion_no_empezada(id_eleccion){
                let eleccion = self.elecciones.get_mut(id_eleccion as usize -1).unwrap();
                let usuario = self.usuarios_registrados.get_mut(id_usuario as usize -1).unwrap();
                if usuario.participacion[id_eleccion as usize]==false{
                    if es_votante{
                        eleccion.postulados_a_votantes.push(Votante::new(usuario.clone().datos));
                    }else{
                        eleccion.postulados_a_candidatos.push(Candidato::new(usuario.clone().datos));
                    }
                    self.usuarios_registrados[id_usuario as usize -1].participacion[id_eleccion as usize -1] = true;
                    return true;
                }
            }
            false
            
        }

        ///el id_usuario es la posicion del votante en el vector de usuarios registrados en el sistema de votacion.
        ///el id_candidato es la posicion del candidato en el vector candidatos adentro de la eleccion.
        #[ink(message)]
        pub fn votar_canditdato(&mut self, id_usuario:u8, id_eleccion:u8, id_candidato:u8)->bool{
            let eleccion = self.elecciones.get_mut(id_eleccion as usize -1).unwrap();
            let votante = Votante::new(self.usuarios_registrados[id_usuario as usize].datos.clone());
            if Self::env().account_id()==self.usuarios_registrados[id_usuario as usize].datos.accountid && eleccion.votantes.contains(&votante)&& 
            (eleccion.candidatos.len() as u8 >= id_candidato) && Self::env().block_timestamp()>eleccion.inicio && Self::env().clone().block_timestamp()>eleccion.fin{
                eleccion.candidatos[id_candidato as usize].cant_votos += 1;
                return true;
            }
            false
        }

        fn _votar_candidato(&mut self, id_usuario:u8, id_eleccion:u8, id_candidato:u8)->bool{
            let eleccion = self.elecciones.get_mut(id_eleccion as usize -1).unwrap();
            let votante = Votante::new(self.usuarios_registrados[id_usuario as usize].datos.clone());
            if Self::env().account_id()==self.usuarios_registrados[id_usuario as usize].datos.accountid && eleccion.votantes.contains(&votante)&& 
            (eleccion.candidatos.len() as u8 >= id_candidato) && Self::env().block_timestamp()>eleccion.inicio && Self::env().clone().block_timestamp()>eleccion.fin{
                eleccion.candidatos[id_candidato as usize].cant_votos += 1;
                return true;
            }
            false
        }
    }

    #[ink::test]
    fn test_crear_eleccion() {
        let mut sistema = SistemaDeVotacion::_new();
        let result = sistema._crear_eleccion("Presidente".into(), 1, 1, 2024, 31, 12, 2024);
        assert!(result);
        assert_eq!(sistema.elecciones.len(), 1);
    }

    #[ink::test]
    fn test_crear_eleccion_u64_invalida() {
        let mut sistema = SistemaDeVotacion::_new();
        let result = sistema._crear_eleccion("Presidente".into(), 31, 2, 2024, 1, 3, 2024);
        assert!(!result);
        assert_eq!(sistema.elecciones.len(), 0);
    }

    #[ink::test]
    fn test_existe_eleccion() {
        let mut sistema = SistemaDeVotacion::new();
        sistema._crear_eleccion("Presidente".into(), 1, 1, 2024, 31, 12, 2024);
        let result = sistema.existe_eleccion(1);
        assert!(result);
    }

    #[ink::test]
    fn test_existe_eleccion_inexistente() {
        let sistema = SistemaDeVotacion::_new();
        let result = sistema.existe_eleccion(1);
        assert!(!result);
    }

    #[ink::test]
    fn test_existe_usuario() {
        let mut sistema = SistemaDeVotacion::_new();
        sistema._crear_usuario("Juan".into(), "Perez".into(), "12345678".into());
        let result = sistema.existe_usuario(1);
        assert!(result);
    }

    #[ink::test]
    fn test_existe_usuario_inexistente() {
        let sistema = SistemaDeVotacion::_new();
        let result = sistema.existe_usuario(1);
        assert!(!result);
    }

    #[ink::test]
    fn test_eleccion_no_empezada() {
        let mut sistema = SistemaDeVotacion::_new();
        sistema._crear_eleccion("Presidente".into(), 1, 1, 2024, 31, 12, 2024);
        let result = sistema.eleccion_no_empezada(1);
        assert!(result);
    }

    #[ink::test]
    fn test_eleccion_no_empezada_inexistente() {
        let sistema = SistemaDeVotacion::_new();
        let result = sistema.eleccion_no_empezada(1);
        assert!(!result);
    }

    // #[ink::test]
    // fn test_iniciar_eleccion() {
    //     let mut sistema = SistemaDeVotacion::_new();
    //     sistema._crear_eleccion("Presidente".into(), 1, 1, 2024, 31, 12, 2024);
    //     let result = sistema._iniciar_eleccion(1);
    //     assert!(result);
    // }

    #[ink::test]
    fn test_iniciar_eleccion_inexistente() {
        let mut sistema = SistemaDeVotacion::_new();
        let result = sistema._iniciar_eleccion(1);
        assert!(!result);
    }

    #[ink::test]
    fn test_get_eleccion() {
        let mut sistema = SistemaDeVotacion::_new();
        sistema._crear_eleccion("Presidente".into(), 1, 1, 2024, 31, 12, 2024);
        let eleccion = sistema._get_eleccion(1);
        assert!(eleccion.is_some());
        assert_eq!(eleccion.unwrap().cargo, "Presidente");
    }

    #[ink::test]
    fn test_get_eleccion_inexistente() {
        let sistema = SistemaDeVotacion::_new();
        let eleccion = sistema._get_eleccion(1);
        assert!(eleccion.is_none());
    }

    // #[ink::test]
    // fn test_validar_usuario() {
    //     let mut sistema = SistemaDeVotacion::_new();
    //     sistema._crear_usuario("Juan".into(), "Perez".into(), "12345678".into());
    //     sistema._crear_eleccion("Presidente".into(), 1, 1, 2024, 31, 12, 2024);
    //     sistema._postulacion_de_usuario(1, 1, true);
    //     let result = sistema._validar_usuario(1, 1, true);
    //     assert!(result);
    //     assert_eq!(sistema.elecciones[0].votantes.len(), 1);
    // }

    // #[ink::test]
    // fn test_validar_usuario_invalido() {
    //     let mut sistema = SistemaDeVotacion::_new();
    //     sistema._crear_usuario("Juan".into(), "Perez".into(), "12345678".into());
    //     sistema._crear_eleccion("Presidente".into(), 1, 1, 2024, 31, 12, 2024);
    //     let result = sistema._validar_usuario(1, 1, true);
    //     assert!(!result);
    // }

    #[ink::test]
    fn test_get_usuario() {
        let mut sistema = SistemaDeVotacion::_new();
        sistema._crear_usuario("Juan".into(), "Perez".into(), "12345678".into());
        let usuario = sistema._get_usuario(1);
        assert!(usuario.is_some());
        assert_eq!(usuario.unwrap().datos.nombre, "Juan");
    }

    #[ink::test]
    fn test_get_usuario_inexistente() {
        let sistema = SistemaDeVotacion::_new();
        let usuario = sistema._get_usuario(1);
        assert!(usuario.is_none());
    }

    #[ink::test]
    fn test_get_usuarios_registrados() {
        let mut sistema = SistemaDeVotacion::_new();
        sistema._crear_usuario("Juan".into(), "Perez".into(), "12345678".into());
        sistema._crear_usuario("Ana".into(), "Garcia".into(), "87654321".into());
        let usuarios = sistema._get_usuarios_registrados();
        assert_eq!(usuarios.len(), 2);
    }

    #[ink::test]
    fn test_get_usuarios_registrados_vacio() {
        let sistema = SistemaDeVotacion::_new();
        let usuarios = sistema._get_usuarios_registrados();
        assert_eq!(usuarios.len(), 0);
    }

    #[ink::test]
    fn test_get_todas_las_elecciones() {
        let mut sistema = SistemaDeVotacion::_new();
        sistema._crear_eleccion("Presidente".into(), 1, 1, 2024, 31, 12, 2024);
        sistema._crear_eleccion("Gobernador".into(), 1, 1, 2024, 31, 12, 2024);
        let elecciones = sistema._get_todas_las_elecciones();
        assert_eq!(elecciones.len(), 2);
    }

    #[ink::test]
    fn test_get_todas_las_elecciones_vacio() {
        let sistema = SistemaDeVotacion::_new();
        let elecciones = sistema._get_todas_las_elecciones();
        assert_eq!(elecciones.len(), 0);
    }

    // #[ink::test]
    // fn test_get_reporte_de_eleccion() {
    //     let mut sistema = SistemaDeVotacion::_new();
    //     sistema._crear_eleccion("Presidente".into(), 1, 1, 2024, 31, 12, 2024);
    //     sistema._iniciar_eleccion(1);
    //     sistema._finalizar_eleccion(1);
    //     let reporte = sistema._get_reporte_de_eleccion(1);
    //     assert!(reporte.is_some());
    //     assert_eq!(reporte.unwrap().cargo, "Presidente");
    // }

    #[ink::test]
    fn test_get_reporte_de_eleccion_inexistente() {
        let sistema = SistemaDeVotacion::_new();
        let reporte = sistema._get_reporte_de_eleccion(1);
        assert!(reporte.is_none());
    }

    #[ink::test]
    fn test_crear_usuario() {
        let mut sistema = SistemaDeVotacion::_new();
        sistema._crear_usuario("Juan".into(), "Perez".into(), "12345678".into());
        assert_eq!(sistema.usuarios_registrados.len(), 1);
        assert_eq!(sistema.usuarios_registrados[0].datos.nombre, "Juan");
    }

    #[ink::test]
    fn test_crear_usuario_multiple() {
        let mut sistema = SistemaDeVotacion::_new();
        sistema._crear_usuario("Juan".into(), "Perez".into(), "12345678".into());
        sistema._crear_usuario("Ana".into(), "Garcia".into(), "87654321".into());
        assert_eq!(sistema.usuarios_registrados.len(), 2);
        assert_eq!(sistema.usuarios_registrados[1].datos.nombre, "Ana");
    }

    // #[ink::test]
    // fn test_postulacion_de_usuario() {
    //     let mut sistema = SistemaDeVotacion::_new();
    //     sistema._crear_usuario("Juan".into(), "Perez".into(), "12345678".into());
    //     sistema._crear_eleccion("Presidente".into(), 1, 1, 2024, 31, 12, 2024);
    //     let result = sistema._postulacion_de_usuario(1, 1, true);
    //     assert!(result);
    //     assert_eq!(sistema.elecciones[0].candidatos.len(), 1);
    // }

    #[ink::test]
    fn test_postulacion_de_usuario_invalida() {
        let mut sistema = SistemaDeVotacion::_new();
        let result = sistema._postulacion_de_usuario(1, 1, true);
        assert!(!result);
    }

    // #[ink::test]
    // fn test_votar_candidato() {
    //     let mut sistema = SistemaDeVotacion::_new();
    //     sistema._crear_usuario("Juan".into(), "Perez".into(), "12345678".into());
    //     sistema._crear_eleccion("Presidente".into(), 1, 1, 2024, 31, 12, 2024);
    //     sistema._iniciar_eleccion(1);
    //     sistema._validar_usuario(1, 1, true);
    //     let result = sistema._votar_candidato(1, 1, 1);
    //     assert!(result);
    //     //assert_eq!(sistema.elecciones[0]..len(), 1);
    // }

    // #[ink::test]
    // fn test_votar_candidato_invalido() {
    //     let mut sistema = SistemaDeVotacion::_new();
    //     let result = sistema._votar_candidato(1, 1, 1);
    //     assert!(!result);
    // }
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

