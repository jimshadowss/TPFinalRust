#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod report {
    use crate::report::string::String;
    use crate::report::vec::Vec;
    use ink::prelude::string::ToString;
    use ink::prelude::*;
    use sistema::SistemaRef;

    ///Referencia al sistema principal
    #[ink(storage)]
    pub struct Report {
        #[cfg(not(test))]
        sistema: SistemaRef,
        #[cfg(test)]
        sistem: Mockeo,
    }
    #[derive(scale::Decode, scale::Encode, PartialEq, Eq, Debug, Default)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub struct Mockeo {
        dato: bool,
    }

    #[derive(scale::Decode, scale::Encode, PartialEq, Eq, Debug, Default)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub struct SocioReport {
        pub dni: u32,
        pub nombre: String,
        pub categoria: Categorias,
    }

    impl SocioReport {
        fn new(dni: u32, nombre: String, categoria: Categorias) -> SocioReport {
            SocioReport {
                dni,
                nombre,
                categoria,
            }
        }
    }
    #[derive(scale::Decode, scale::Encode, PartialEq, Eq, Debug, Default)]
    #[cfg_attr(
        feature = "std",
        derive(scale_info::TypeInfo, ink::storage::traits::StorageLayout)
    )]
    pub enum Categorias {
        #[default]
        A,
        B,
        C,
    }
    impl Funciones for Mockeo {
        fn verificar_pagos_pendientes(&self) -> Result<Vec<(u32, String, String)>, String> {
            if self.dato {
                let mut vec = Vec::new();
                let socio = (1, "nombre".to_string(), "A".to_string());
                vec.push(socio);
                let socio = (2, "nombre2".to_string(), "C".to_string());
                vec.push(socio);
                Ok(vec)
            } else {
                Err("No es contrato".to_string())
            }
        }
        fn get_no_morosos_act_report(
            &self,
            actividad: String,
        ) -> Result<Vec<(u32, String, String)>, String> {
            let mut vec = Vec::new();
            let mut res = Err("No es una actividad válida".to_string());
            if actividad.eq("Gimnasio") {
                let s1 = (1, "nombre".to_string(), "A".to_string());
                vec.push(s1);
                let s1 = (2, "nombre2".to_string(), "C".to_string());
                vec.push(s1);
                res = Ok(vec);
            } else if actividad.eq("Futbol")
                || actividad.eq("Basquet")
                || actividad.eq("Rugby")
                || actividad.eq("Hockey")
                || actividad.eq("Natacion")
                || actividad.eq("Tenis")
                || actividad.eq("Paddle")
            {
                let s1 = (1, "nombre".to_string(), "A".to_string());
                vec.push(s1);
                res = Ok(vec);
            }
            res
        }
    }
    impl Funciones for SistemaRef {
        fn verificar_pagos_pendientes(&self) -> Result<Vec<(u32, String, String)>, String> {
            self.verificacion_pagos_pendientes()
        }
        fn get_no_morosos_act_report(
            &self,
            actividad: String,
        ) -> Result<Vec<(u32, String, String)>, String> {
            self.get_no_morosos_act(actividad)
        }
    }

    pub trait Funciones {
        fn verificar_pagos_pendientes(&self) -> Result<Vec<(u32, String, String)>, String>;
        fn get_no_morosos_act_report(
            &self,
            actividad: String,
        ) -> Result<Vec<(u32, String, String)>, String>;
    }

    impl Report {
        #[ink(constructor)]
        #[cfg(not(test))]
        pub fn new(sistema: SistemaRef) -> Self {
            #[cfg(test)]
            let dato = true;
            #[cfg(test)]
            let sistem = Mockeo { dato };
            Self {
                #[cfg(not(test))]
                sistema,
                #[cfg(test)]
                sistem,
            }
        }
        #[cfg(test)]
        fn new2(#[cfg(not(test))] sistema: SistemaRef) -> Self {
            let dato = true;
            let sistem = Mockeo { dato };
            Self {
                #[cfg(not(test))]
                sistema,
                sistem,
            }
        }
        ///El report solicita permiso para poder recopilar la informacion de manera segura, siempre lo debe ejecutar el dueño del sistema luego de instanciar el report
        #[ink(message)]
        #[cfg(not(test))]
        pub fn solicitar_permiso(&mut self) -> Result<String, String> {
            self.sistema.solicitar_permiso(self.env().caller())
        }

        #[cfg(test)]
        fn verificacion_pagos_pendientes_with_mock(&self) -> Result<Vec<SocioReport>, String> {
            self.verificacion_pagos_pendientes2(self.sistem.verificar_pagos_pendientes())
        }
        ///Muestra una lista de socios que tienen pagos pendientes ya vencidos
        #[ink(message)]
        #[cfg(not(test))]
        pub fn verificacion_pagos_pendientes(&self) -> Result<Vec<SocioReport>, String> {
            self.verificacion_pagos_pendientes2(self.sistema.verificar_pagos_pendientes())
        }
        pub fn verificacion_pagos_pendientes2(
            &self,
            aux: Result<Vec<(u32, String, String)>, String>,
        ) -> Result<Vec<SocioReport>, String> {
            let mut vec = Vec::new();
            let res;
            match aux {
                Ok(a) => {
                    for i in a {
                        let cat: Categorias;
                        if i.2.eq("A") {
                            cat = Categorias::A;
                        } else if i.2.eq("B") {
                            cat = Categorias::B;
                        } else {
                            cat = Categorias::C;
                        }
                        let socio = SocioReport::new(i.0, i.1.clone(), cat);
                        vec.push(socio)
                    }
                    res = Ok(vec)
                }
                Err(e) => res = Err(e),
            }
            res
        }
        ///Muestra el monto total de la recaudacion mensual de una categoria dada (A, B o C)
        #[ink(message)]
        #[cfg(not(test))]
        pub fn informe_recaudacion_mensual(&self, categoria: String) -> Result<u32, String> {
            match &self.sistema.informe_recaudacion_mensual(categoria) {
                Ok(a) => Ok(*a),
                Err(e) => Err(e.clone()),
            }
        }
        #[cfg(test)]
        fn informe_socios_no_morosos_actividad_with_mock(
            &self,
            actividad: String,
        ) -> Result<Vec<SocioReport>, String> {
            self.informe_socios_no_morosos_actividad2(
                self.sistem.get_no_morosos_act_report(actividad),
            )
        }
        ///Muestra una lista de socios no morosos, que tengan acceso a una actividad especifica dada (Futbol, Gimnasio, Basquet, Rugby, Hockey, Natacion, Tenis o Paddle)
        #[ink(message)]
        #[cfg(not(test))]
        pub fn informe_socios_no_morosos_actividad(
            &self,
            actividad: String,
        ) -> Result<Vec<SocioReport>, String> {
            self.informe_socios_no_morosos_actividad2(
                self.sistema.get_no_morosos_act_report(actividad),
            )
        }
        fn informe_socios_no_morosos_actividad2(
            &self,
            aux: Result<Vec<(u32, String, String)>, String>,
        ) -> Result<Vec<SocioReport>, String> {
            let mut vec = Vec::new();
            match aux {
                Ok(a) => {
                    for i in a {
                        let aux = if i.2.eq("A") {
                            Categorias::A
                        } else if i.2.eq("B") {
                            Categorias::B
                        } else if i.2.eq("C") {
                            Categorias::C
                        } else {
                            break;
                        };
                        let socio = SocioReport::new(i.0, i.1, aux);
                        vec.push(socio);
                    }
                    Ok(vec)
                }
                Err(e) => Err(e),
            }
        }
    }

    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        #[ink::test]
        fn new2_test() {
            let rep = Report::new2();
            assert!(rep.sistem.dato)
        }
        #[ink::test]
        fn informe_socios_no_morosos_actividad2_test() {
            let rep = Report::new2();
            let res = rep.informe_socios_no_morosos_actividad_with_mock("Futbol".to_string());
            let mut bien = true;
            match res {
                Ok(a) => {
                    if a[0].dni != 1
                        || !a[0].nombre.eq("nombre")
                        || !a[0].categoria.eq(&Categorias::A)
                    {
                        bien = false;
                    }
                }
                Err(_) => bien = false,
            }
            assert!(bien)
        }
        #[ink::test]
        fn informe_socios_no_morosos_actividad2_test2() {
            let rep = Report::new2();
            let res = rep.informe_socios_no_morosos_actividad_with_mock("Gimnasio".to_string());
            let mut bien = true;
            match res {
                Ok(a) => {
                    if a[0].dni != 1
                        || !a[0].nombre.eq("nombre")
                        || !a[0].categoria.eq(&Categorias::A)
                    {
                        bien = false;
                    }
                    if a[1].dni != 2
                        || !a[1].nombre.eq("nombre2")
                        || !a[1].categoria.eq(&Categorias::C)
                    {
                        bien = false;
                    }
                }
                Err(_) => bien = false,
            }
            assert!(bien)
        }
        #[ink::test]
        fn informe_socios_no_morosos_actividad2_test3() {
            let rep = Report::new2();
            let res = rep.informe_socios_no_morosos_actividad_with_mock("OtraCosa".to_string());

            match res {
                Ok(_) => assert!(false),
                Err(e) => assert!(e.eq("No es una actividad válida")),
            }
        }
        #[ink::test]
        pub fn verificacion_pagos_pendientes2_test() {
            let rep = Report::new2();
            let mut bien = true;
            let res = rep.verificacion_pagos_pendientes_with_mock();
            match res {
                Ok(a) => {
                    if a[0].dni != 1
                        || !a[0].nombre.eq("nombre")
                        || !a[0].categoria.eq(&Categorias::A)
                    {
                        bien = false;
                    }
                    if a[1].dni != 2
                        || !a[1].nombre.eq("nombre2")
                        || !a[1].categoria.eq(&Categorias::C)
                    {
                        bien = false;
                    }
                }
                Err(_) => bien = false,
            }
            assert!(bien)
        }
        #[ink::test]
        pub fn verificacion_pagos_pendientes2_test2() {
            let mut rep = Report::new2();
            rep.sistem.dato = false;
            let res = rep.verificacion_pagos_pendientes_with_mock();
            match res {
                Ok(_) => assert!(false),
                Err(e) => assert!(e.eq("No es contrato")),
            }
        }
    }
}
