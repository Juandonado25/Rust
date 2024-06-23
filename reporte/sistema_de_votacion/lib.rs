#![cfg_attr(not(feature = "std"), no_std, no_main)]
#[allow(clippy::arithmetic_side_effects)]
pub use self::sistema_de_votacion::SistemaDeVotacionRef;
#[ink::contract]
pub mod sistema_de_votacion {
    use ink::prelude::string::String;
    use ink::prelude::vec::Vec;
    use scale_info::prelude::vec;

    #[derive(scale::Decode, scale::Encode,Debug,Default,Clone,Copy)]
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
    #[derive(scale::Decode, scale::Encode,Debug,Default,Clone)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub struct Eleccion{
        cargo:String,//se detalla el cargo que sera elegido en esta eleccion, informacion que puede ser relevante para el reporte.
        inicio:Fecha,
        fin:Fecha,
        abierta:bool,
        finalizada:bool, //sirve para saber si esta cerrada porque finalizo o nunca empezo.
        postulados_a_votantes:Vec<Votante>,
        votantes:Vec<Votante>,
        postulados_a_candidatos:Vec<Candidato>,
        candidatos:Vec<Candidato>,
    }

    impl Eleccion{
        pub fn new(cargo:String,inicio:&Fecha, fin:&Fecha)->Self{
            Self{cargo,inicio:*inicio,fin:*fin,abierta:false,postulados_a_votantes:Vec::new(),postulados_a_candidatos:Vec::new(), finalizada:false ,votantes:Vec::new(),candidatos:Vec::new()}
        }
    }
    #[derive(scale::Decode, scale::Encode,Debug,Clone,Default,PartialEq)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]   
    pub struct Persona{
        nombre:String,
        apellido:String,
        dni:String,

    }

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
    pub struct Usuario{
        datos:Persona,
        participacion:Vec<bool>,//vector usado para controlar si la persona esta participando de una eleccion, 
                                //debido a que el id de la eleccion se corresponde con su posicion en el vector, este seria contendria las misma longitud,
                                //si es true participa en esa eleccion, false si no. por ejemplo si pos1=true participa en la eleccion de id 1.
                                //lo hacemos para no inscribir mas de una vez al usuario en una misma eleccion,
    }
/////////////////A revisar inicializacion del vector
    impl Usuario{
        fn new(nombre:String, apellido:String, dni:String, longitud:u8)->Self{
            Self{datos:Persona::new(nombre,apellido,dni),participacion:vec!{false;longitud as usize}}
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
        usuarios_registrados:Vec<Usuario>,//todos los usuarios regitrados en el sistema, pueden participar de una elecion o no
        elecciones:Vec<Eleccion>,
    }
    impl SistemaDeVotacion {
        // Constructor
        #[ink(constructor)]
        pub fn new() -> Self {
            Self { 
                usuarios_registrados:Vec::new(),
                elecciones:Vec::new(),
            }
        }
        
        //METODOS ADMINISTRADOR

        //Crea una eleccion y la pushea en la structura principal, el id de cada eleccion es la posicion en el vector +1.
        #[ink(message)]
        pub fn crear_eleccion(&mut self,cargo:String,dia_inicio:u16,mes_inicio:u16,anio_inicio:u16,dia_fin:u16,mes_fin:u16,anio_fin:u16 )->bool{
            let fecha_de_inicio = Fecha::new(dia_inicio,mes_inicio,anio_inicio);
            let fecha_de_fin = Fecha::new(dia_fin,mes_fin,anio_fin);
            if fecha_de_inicio.es_fecha_valida() && fecha_de_fin.es_fecha_valida(){
                let elec = Eleccion::new(cargo,&fecha_de_inicio,&fecha_de_fin);
                self.elecciones.push(elec);
                for e in self.usuarios_registrados.iter_mut(){
                    e.participacion.push(false);
                }
                return true;
            }
            false
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

        //retorna true se puede inscribir un usuario a esa eleccion porque existe esta cerrada y no finalizada.
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

        //Si existe la eleccion y hay mas de un candidato la inicializa.
        #[ink(message)]
        pub fn iniciar_eleccion(&mut self,id:u8)->bool{
            if self.existe_eleccion(id){
                let eleccion=self.elecciones.get_mut(id as usize -1).unwrap();
                if eleccion.candidatos.len()>=2{
                    eleccion.abierta=true;
                    return true;
                }
            }
            false
        }

        //Cierra la eleccion y la marca como finalizada.
        #[ink(message)]
        pub fn finalizar_eleccion(&mut self,id:u8)->bool{
            if self.existe_eleccion(id){
                let eleccion=self.elecciones.get_mut(id as usize -1).unwrap();
                eleccion.abierta=false;
                eleccion.finalizada=true;
                return true;
            }
            false
        }

        //Devuelve una eleccion, util para el reporte.
        #[ink(message)]
        pub fn get_eleccion(&self, eleccion_id:u8)->Option<Eleccion>{
            if eleccion_id!=0 && eleccion_id>=self.elecciones.len() as u8{
                let elec = self.elecciones.get(eleccion_id as usize -1).unwrap();
                return Some(elec.clone())
            }
            None
        }

        //retorna true si se pudo validar con exito, false en caso contrario.
        #[ink(message)]
        pub fn validar_usuario(&mut self, id_usuario:u8, id_eleccion:u8, valido:bool)->bool{
            if self.existe_eleccion(id_eleccion) && self.existe_ususario(id_usuario) && valido && self.eleccion_no_empezada(id_eleccion) 
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
            if id_usuario>=self.usuarios_registrados.len() as u8{
                return Some(self.usuarios_registrados.get(id_usuario as usize -1).unwrap().clone());
            }
            None
        }

        #[ink(message)]
        pub fn get_usuarios_registrados(&self)-> Vec<Usuario>{
            self.usuarios_registrados.clone()
        }

        #[ink(message)]
        pub fn get_todas_las_elecciones(&self)-> Vec<Eleccion>{
            self.elecciones.clone()
        }

        // Devuelve los datos de una eleccion, solo si esta esta cerrada y finalizada.
        #[ink(message)]
        pub fn get_reporte_de_eleccion(&self, id_eleccion:u8)->Option<Eleccion>{
            if self.existe_eleccion(id_eleccion){
                let eleccion = self.elecciones.get(id_eleccion as usize -1).unwrap();
                if eleccion.finalizada{
                    return Some(eleccion.clone())
                }
            }
            None
        }

        //METODOS DE USUARIO
        
        #[ink(message)]
        pub fn crear_ususario(&mut self, nombre:String, apellido:String, dni:String){
            let usuario = Usuario::new(nombre, apellido, dni, self.elecciones.len() as u8);
            self.usuarios_registrados.push(usuario);
        }
        
        //si es_votante es true lo inscribe como votante, en caso contrario como candidato y ademas cambia a true
        // la participacion del usuario en dicha eleccion para que no pueda inscribirse 2 veces en la misma eleccion.
        #[ink(message)]
        pub fn postulacion_de_ususario(&mut self, id_usuario:u8, id_eleccion:u8, es_votante:bool)->bool{
            if self.existe_eleccion(id_eleccion) && self.existe_ususario(id_usuario) && self.eleccion_no_empezada(id_eleccion){
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

        //el id_usuario es la posicion del votante en el vector de usuarios registrados en el sistema de votacion.
        //el id_candidato es la posicion del candidato en el vector candidatos adentro de la eleccion.
        #[ink(message)]
        pub fn votar_canditato(&mut self, id_usuario:u8, id_eleccion:u8, id_candidato:u8)->bool{
            let eleccion = self.elecciones.get_mut(id_eleccion as usize -1).unwrap();
            let votante = Votante::new(self.usuarios_registrados[id_usuario as usize].datos.clone());
            if eleccion.votantes.contains(&votante)&& (eleccion.candidatos.len() as u8 >= id_candidato){
                eleccion.candidatos[id_candidato as usize].cant_votos += 1;
                return true;
            }
            false
        }
    }
}
/*
    Preguntas del planteamiento:
        1- para registrarse como candidato se debe pedir mas datos ademas de su info personal? como años de antiguedad en la empresa o cantidad de titulos obtenidos.
        
    Preguntas del deploy:
        1- El metodo crear_eleccion acertadamente no agrega la eleccion si la fecha no es valida, pero el contract deberia mostrarte directamente que no se puede hacer. Panic?


    Notas:
        !- Tener en cuenta que si la eleccion tiene un solo candidato no se va a poder inicializar y 
        en el reporte se marcara como ganador al unico candidato. si no existe ningun candidato retornara eleccion invalida.
*/

// #[cfg(test)]
//     mod tests {
//         use super::*;

        
//         #[ink::test]
//         fn default_works() {
//             let reporte = Reporte::default();
//             assert_eq!(reporte.get(), false);
//         }
//     }