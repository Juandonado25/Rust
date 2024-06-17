#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod votacion {
    use ink::prelude::string::String;
    use ink::prelude::vec::Vec;
    use rand::prelude::*;

    #[derive(scale::Decode, scale::Encode,Debug)]
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
    #[derive(scale::Decode, scale::Encode,Debug)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    struct Eleccion{
        id:u8,
        inicio:Fecha,
        fin:Fecha,
        abierta:bool,
        votantes:Vec<Votante>,
        candidatos:Vec<Candidato>,
        cantidad_votos:u8,//A revisar
        cantidad_votantes:u8,
    }
    impl Eleccion{

        #[ink{message}]
        pub fn registar_usuario(&mut self, user:Persona){
            //Temporal
            let num = rand::thread_rng().gen_range(1..=2);
            if num == 2 && self.candidatos.len()<8{
                self.candidatos.push(Candidato::new(user))
            }else{
                self.votantes.push(Votante::new(user));
            }
        }
    }
    #[derive(scale::Decode, scale::Encode,Debug,Clone)]
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
    pub struct Votacion {
        votaciones:Vec<Eleccion>,
    }
    impl Votacion {
        // Constructor
        #[ink(constructor)]
        pub fn new() -> Self {
            Self { 
                votaciones:Vec::new(),
            }
        }
    }
}