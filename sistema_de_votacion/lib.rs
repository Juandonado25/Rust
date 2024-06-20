#![cfg_attr(not(feature = "std"), no_std, no_main)]
#[allow(clippy::arithmetic_side_effects)]
#[ink::contract]
mod votacion {
    use ink::prelude::string::String;
    use ink::prelude::vec::Vec;

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
        pub fn new(dia:u16,mes:u16,anio:u16)->Self{
            Self{dia,mes,anio}
        }

        pub fn es_fecha_valida(&self) -> bool{//probar con varios
            let aux:bool = match self.mes {
                1 => self.dia>0 && self.dia<=31,
                2 => (!self.es_bisiesto() && self.dia>0 && self.dia<=28) || (self.es_bisiesto() && self.dia>0 && self.dia<=29),
                3 => self.dia>0 && self.dia<=31,
                4 => self.dia>0 && self.dia<=30,
                5 => self.dia>0 && self.dia<=31,
                6 => self.dia>0 && self.dia<=30,
                7 => self.dia>0 && self.dia<=31,
                8 => self.dia>0 && self.dia<=31,
                9 => self.dia>0 && self.dia<=30,
                10 => self.dia>0 && self.dia<=31,
                11 => self.dia>0 && self.dia<=30,
                12 => self.dia>0 && self.dia<=31,
                _ => false,
            };
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
    struct Eleccion{
        inicio:Fecha,
        fin:Fecha,
        abierta:bool,
        participantes:Vec<Persona>,
        votantes:Vec<Votante>,
        candidatos:Vec<Candidato>,
    }

    impl Eleccion{
        pub fn new(inicio:Fecha, fin:Fecha)->Self{
            Self{inicio,fin,abierta:false,participantes:Vec::new() ,votantes:Vec::new(),candidatos:Vec::new()}
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
        dni:String,

    }
/////////////////A revisar inicializacion del vector
    impl Persona{
        fn new(nombre:String, apellido:String, dni:String)->Self{
            Self{nombre,apellido,dni}
        }
    }
    #[derive(scale::Decode, scale::Encode,Debug,Clone)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    struct Usuario{
        datos:Persona,
        participacion:Vec<bool>,
    }
    impl Usuario{
        fn new(nombre:String, apellido:String, dni:String, longitud:u8)->Self{
            Self{datos:Persona::new(nombre,apellido,dni),participacion:vec!{false;longitud as usize}}
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
        usuarios_registrados:Vec<Usuario>,
        elecciones:Vec<Eleccion>,
    }
    impl Administracion {
        // Constructor
        #[ink(constructor)]
        pub fn new() -> Self {
            Self { 
                usuarios_registrados:Vec::new(),
                elecciones:Vec::new(),
            }
        }
        
        //Crea una eleccion y la pushea en la structura principal, el id de cada eleccion es la posicion en el vector +1.
        #[ink(message)]
        pub fn crear_eleccion(&mut self,dia_inicio:u16,mes_inicio:u16,anio_inicio:u16,dia_fin:u16,mes_fin:u16,anio_fin:u16 ){
            let elec = Eleccion::new(Fecha::new(dia_inicio,mes_inicio,anio_inicio),Fecha::new(dia_fin,mes_fin,anio_fin));
            self.elecciones.push(elec);
            for e in self.usuarios_registrados.iter_mut(){
                e.participacion.push(false);
            }
        }

        
        #[ink(message)]
        pub fn crear_ususario(&mut self, nombre:String, apellido:String, dni:String){
            let usuario = Usuario::new(nombre, apellido, dni, self.elecciones.len() as u8);
            self.usuarios_registrados.push(usuario);
        }
        
        //si es_votante es true lo inscribe como votante, en caso contrario como candidato y ademas cambia a true
        // la participacion del usuario en dicha eleccion para que no pueda inscribirse 2 veces en la misma eleccion.
        #[ink(message)]
        pub fn postulacion_de_ususario(&mut self, id_usuario:u8, id_eleccion:u8, es_votante:bool){
            if self.existe_eleccion(id_eleccion) && self.existe_ususario(id_usuario) && self.verificar_estado_eleccion(id_eleccion){
                let eleccion = self.elecciones.get_mut(id_eleccion as usize -1).unwrap();
                let usuario = self.usuarios_registrados.get_mut(id_usuario as usize -1).unwrap();
                if usuario.participacion[id_eleccion as usize]==false{
                    if es_votante{
                        eleccion.votantes.push(Votante::new(usuario.clone().datos));
                    }else{
                        eleccion.candidatos.push(Candidato::new(usuario.clone().datos));
                    }
                    self.usuarios_registrados[id_usuario as usize -1].participacion[id_eleccion as usize -1] = true;
                }
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

        fn existe_ususario(&self,id:u8)->bool{
            if id==0{
                false
            }
            else{
                self.usuarios_registrados.len()>=id as usize
            }
        }

        //retorna true se puede inscribir un usuario a esa eleccion porque existe y esta cerrada.
        fn verificar_estado_eleccion(&self,id:u8)->bool{
            if id==0{
                false
            }
            else{
                if self.elecciones.len()>=id as usize {
                    let eleccion=self.elecciones.get(id as usize -1).unwrap();
                    if eleccion.abierta==false{
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
        pub fn iniciar_eleccion(&mut self,id:u8)->bool{
            if self.existe_eleccion(id){
                let eleccion=self.elecciones.get_mut(id as usize -1).unwrap();
                eleccion.abierta=true;
                return true;
            }
            false
        }
        #[ink(message)]
        pub fn finalizar_eleccion(&mut self,id:u8)->bool{
            if self.existe_eleccion(id){
                let eleccion=self.elecciones.get_mut(id as usize -1).unwrap();
                eleccion.abierta=false;
                return true;
            }
            false
        }
        
    }
}
/*
    Preguntas:

    
*/
    
/*
    Notas:
    
*/
