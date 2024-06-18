#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod votacion {

    use ink::prelude::string::String;
    use ink::prelude::vec::Vec;
    use rand::prelude::*;

    #[derive(scale::Decode, scale::Encode,Debug,Default)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub struct Fecha{
        dia:u16,
        mes:u16,
        anio:u16
    }
    impl Fecha{
        pub fn es_fecha_valida(&self) -> bool{//probar con varios
            let aux:bool;
            match self.mes {
                1 => aux = if self.dia>0 && self.dia<=31{true} else {false},
                2 => aux = if(!self.es_bisiesto() && self.dia>0 && self.dia<=28) || (self.es_bisiesto() && self.dia>0 && self.dia<=29){true} else {false},
                3 => aux = if self.dia>0 && self.dia<=31{true} else {false},
                4 => aux = if self.dia>0 && self.dia<=30{true} else {false},
                5 => aux = if self.dia>0 && self.dia<=31{true} else {false},
                6 => aux = if self.dia>0 && self.dia<=30{true} else {false},
                7 => aux = if self.dia>0 && self.dia<=31{true} else {false},
                8 => aux = if self.dia>0 && self.dia<=31{true} else {false},
                9 => aux = if self.dia>0 && self.dia<=30{true} else {false},
                10 => aux = if self.dia>0 && self.dia<=31{true} else {false},
                11 => aux = if self.dia>0 && self.dia<=30{true} else {false},
                12 => aux = if self.dia>0 && self.dia<=31{true} else {false},
                _ => aux = false,
            }
            aux
        }
        pub fn es_bisiesto(&self) -> bool{
            let aux:bool;
            if &self.anio % 4 == 0{
                if &self.anio % 100 == 0{
                    if &self.anio % 400 == 0{
                        aux = true;
                    }else{
                        aux = false;
                    }
                }else{
                    aux = true;
                }
            }else{
                aux = false;
            }
            aux
        }
    }
    #[derive(scale::Decode, scale::Encode,Debug,Default)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    struct Votacion{
        id:u8,
        inicio:Fecha,
        fin:Fecha,
        abierta:bool,
        usuarios_registrados:Vec<Persona>,
        //indice_auxiliar_usuarios:u8,
        votantes:Vec<Votante>,
        candidatos:Vec<Candidato>,
    }

    impl Votacion{
        pub fn new(id:u8, inicio:Fecha, fin:Fecha)->Self{
            let en_blanco = Candidato::new(Persona::new("En blanco".to_string(), "".to_string(), 0));
            Self{id,inicio,fin,abierta:false,usuarios_registrados:Vec::new(), votantes:Vec::new(),candidatos:vec![en_blanco]}
        }
        pub fn agregar_usuario(&mut self, user:Persona){
            self.usuarios_registrados.push(user)
        }
    }

    #[derive(scale::Decode, scale::Encode,Debug,Clone,Default)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]    
    pub struct Persona{
        nombre:String,
        apellido:String,
        dni:u128,
    }
    impl Persona{
        fn new(nombre:String, apellido:String, dni:u128)->Self{
            Self{nombre,apellido,dni}
        }
    }
    #[derive(scale::Decode, scale::Encode,Debug)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    struct Votante{
        dato: Persona,
        estado_del_voto: bool,
    }
    impl Votante{
        pub fn new(dato:Persona)->Self{
            Self{dato,estado_del_voto:false}
        }
    }
    #[derive(scale::Decode, scale::Encode,Debug)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    struct Candidato{
        dato: Persona,
        cant_votos:u8,
    }
    impl Candidato{
        pub fn new(dato:Persona)->Self{
            Self{dato,cant_votos:0}
        }
    }
    #[ink(storage)]
    pub struct Administracion {
        votaciones:Vec<Votacion>,
    }
    impl Administracion {
        // Constructor
        #[ink(constructor)]
        pub fn new() -> Self {
            Self { 
                votaciones:Vec::new(),
            }
        }

        #[ink(message)]
        pub fn crear_Votacion(&mut self,id:u8,inicio:Fecha, fin:Fecha){
            let id=self.votaciones.len() as u8 +1;
            let mut elec = Votacion::new(id, inicio, fin);
            self.votaciones.push(elec);
        }
        fn existe_votacion(&self,id:u8)->bool{
            if id==0{
                false
            }
            else{
                self.votaciones.len()>=id as usize
            }
        }
        fn verificar_estado_votacion(&self,id:u8)->bool{
            if id==0{
                false
            }
            else{
                if self.votaciones.len()>=id as usize {
                    let votacion=self.votaciones.get(id as usize -1).unwrap();
                    if votacion.abierta==false{
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
        #[ink(message)]
        pub fn agregar_usuario(&mut self,id:u8,nombre:String,apellido:String,dni:u128)->bool{
            if self.existe_votacion(id)&&self.verificar_estado_votacion(id){
                let  user=Persona::new(nombre,apellido,dni);
                let votacion=self.votaciones.get_mut(id as usize -1).unwrap();
                votacion.usuarios_registrados.push(user);
                return true;
            }
            false
        }
        #[ink(message)]
        pub fn iniciar_votacion(&mut self,id:u8)->bool{
            if self.existe_votacion(id){
                let votacion=self.votaciones.get_mut(id as usize -1).unwrap();
                votacion.abierta=true;
                return true;
            }
            false
        }
        #[ink(message)]
        pub fn finalizar_votacion(&mut self,id:u8)->bool{
            if self.existe_votacion(id){
                let votacion=self.votaciones.get_mut(id as usize -1).unwrap();
                votacion.abierta=false;
                return true;
            }
            false
        }

        #[ink(message)]
        pub fn votar(&mut self, id:u8){
            if self.existe_votacion(id)&&!self.verificar_estado_votacion(id){
                let votacion=self.votaciones.get_mut(id as usize -1).unwrap();
                for e in votacion.votantes.iter_mut(){
                    e.estado_del_voto=true;
                    let mut rng = rand::thread_rng();
                    let numero_aleatorio = rng.gen_range(0..=votacion.candidatos.len()-1);
                    let candidato = votacion.candidatos.get_mut(numero_aleatorio).unwrap();
                    candidato.cant_votos+=1;
                }
            }
        }
        
    }
}
/*
    Preguntas:
    1.Como validar el usuario? consultar metodo "agregar_usuario" 
    2. Como asignarle un rol a los usuarios? indice auxiliar?
    3. Como hacer que voten? Consultar metodo "votar"
*/
    
/*
    Notas:
    -No contar voto en blancco(Posicion 0)
*/
