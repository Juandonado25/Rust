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
    }
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
    #[derive(scale::Decode, scale::Encode,Debug)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]

    struct Persona{
        nombre:String,
        apellido:String,
        dni:u128,
    }
    impl Persona{
        fn new(nombre:String, apellido:String, dni:u128)->Self{
            Self{nombre,apellido,dni}
        }
    }
    enum Rol{
        Votante,
        Candidato,
    }

    struct Votante{
        dato: Persona,
        estado_del_voto:boolean,
        rol:Rol,
    }
    #[derive(scale::Decode, scale::Encode,Debug)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    struct Candidato{
        dato: Persona,
        rol:Rol,
        cant_votos:u8,
    }
    #[ink(storage)]
    pub struct Votacion {
        votaciones:Vec<Eleccion>,
    }
    impl Votacion {
        /// Constructor
        #[ink(constructor)]
        pub fn new() -> Self {
            Self { 
                votaciones:Vec::new(),
            }
        }

        #[ink{message}]
        pub registar_usuario($mut self, user:&Persona){

        }
    }
}