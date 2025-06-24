use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Fecha {
    dia: u32,
    mes: u32,
    ano: u32,
}
#[allow(dead_code)]
impl Fecha {
    pub fn new(dia: u32, mes: u32, ano: u32) -> Fecha {
        return Fecha { dia, mes, ano };
    }

    pub fn hoy() -> Self {
        let mut f = Fecha::new(1, 1, 1999);
        f.setear_hoy();
        f
    }

    pub fn es_fecha_valida(&self) -> bool {
        //Se chequean limites generales
        if !(1..=31).contains(&self.dia) || !(1..=13).contains(&self.mes) {
            return false;
        }
        //Condicion si el mes no es febrero
        if self.mes != 2 {
            //Si es mes largo
            if self.es_mes_corto() && self.dia > 30 {
                return false;
            //Si es mes corto
            } else if self.dia > 31 {
                return false;
            }
        //Condicion si el mes es febrero
        } else {
            // Si no es bisiesto y tiene 29
            if self.dia > 28 && !self.es_bisiesto() {
                return false;
            }
        }
        return true;
    }

    pub fn es_mes_corto(&self) -> bool {
        return [4, 6, 9, 11].contains(&self.mes);
    }

    pub fn es_bisiesto(&self) -> bool {
        return (self.ano % 4 == 0 && self.ano % 100 != 0) || (self.ano % 400 == 0);
    }

    pub fn es_mayor(&self, otra_fecha: Fecha) -> bool {
        if self.ano > otra_fecha.ano {
            return true;
        } else if self.mes > otra_fecha.mes {
            return true;
        } else if self.dia > otra_fecha.dia {
            return true;
        }
        return false;
    }

    pub fn restar_dias(&mut self, mut cantidad_dias: i32) {
        while cantidad_dias != 0 {
            self.dia -= 1;
            cantidad_dias -= 1;

            if self.dia == 0 {
                self.mes -= 1;
                if self.mes == 0 {
                    self.ano -= 1;
                    self.mes = 12;
                }
                self.dia = 30;
                if self.mes == 2 {
                    if self.es_bisiesto() {
                        self.dia -= 1;
                    } else {
                        self.dia -= 2;
                    }
                } else if !self.es_mes_corto() {
                    self.dia += 1;
                }
            }
        }
    }

    pub fn sumar_dias(&mut self, mut cantidad_dias: u32) {
        while cantidad_dias != 0 {
            let mut lim_dias = 30;
            if self.mes != 2 {
                if !self.es_mes_corto() {
                    lim_dias += 1;
                }
            } else {
                if !self.es_bisiesto() {
                    lim_dias -= 2
                } else {
                    lim_dias -= 1;
                }
            }

            while self.dia <= lim_dias {
                self.dia += 1;
                cantidad_dias -= 1;
                if cantidad_dias == 0 {
                    break;
                }
            }

            if cantidad_dias == 0 {
                break;
            }

            self.mes += 1;
            if self.mes == 13 {
                self.ano += 1;
                self.mes = 1;
            }
            self.dia = 1;
        }
    }

    pub fn set_fecha(&mut self, dia: u32, mes: u32, ano: u32) {
        self.dia = dia;
        self.mes = mes;
        self.ano = ano;
    }

    pub fn setear_hoy(&mut self) {
        let ahora = Local::now().date_naive();
        self.dia = ahora.day();
        self.mes = ahora.month();
        self.ano = ahora.year() as u32;
    }

    pub fn to_string(&self) -> String {
        format!("{}/{}/{}", self.dia, self.mes, self.ano)
    }
}

impl fmt::Display for Fecha {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}/{}/{}", self.dia, self.mes, self.ano)
    }
}

#[test]
fn test_fecha_es_bisiesto() {
    let fecha: Fecha = Fecha::new(29, 02, 2024);
    assert_eq!(true, fecha.es_bisiesto());

    let fecha: Fecha = Fecha::new(29, 02, 2020);
    assert_eq!(true, fecha.es_bisiesto());

    let fecha: Fecha = Fecha::new(29, 02, 1900);
    assert_eq!(false, fecha.es_bisiesto());

    let fecha: Fecha = Fecha::new(29, 02, 2021);
    assert_eq!(false, fecha.es_bisiesto());
}

#[test]
fn test_fecha_es_mayor() {
    // TEST 29-02-2024 > 28-02-2024 ✅
    let fecha: Fecha = Fecha::new(29, 02, 2024);
    let otra_fecha: Fecha = Fecha::new(28, 02, 2024);
    assert_eq!(true, fecha.es_mayor(otra_fecha));

    // TEST 01-02-2024 > 28-02-2024 ❌
    let fecha: Fecha = Fecha::new(1, 02, 2024);
    let otra_fecha: Fecha = Fecha::new(28, 02, 2024);
    assert_eq!(false, fecha.es_mayor(otra_fecha));

    // TEST 01-02-2024 > 28-02-1985 ✅
    let fecha: Fecha = Fecha::new(1, 02, 2024);
    let otra_fecha: Fecha = Fecha::new(28, 02, 1985);
    assert_eq!(true, fecha.es_mayor(otra_fecha));

    // TEST 01-02-2024 > 28-01-2024 ✅
    let fecha: Fecha = Fecha::new(1, 02, 2024);
    let otra_fecha: Fecha = Fecha::new(28, 01, 2024);
    assert_eq!(true, fecha.es_mayor(otra_fecha));
}

#[test]
fn test_fecha_es_fecha_valida() {
    // fecha -> 1/1/2025 ✅ - TEST LIMITES NORMALES
    let fecha: Fecha = Fecha::new(1, 1, 2025);
    assert_eq!(true, fecha.es_fecha_valida());

    // fecha -> 30/4/1985 ✅ - TEST LIMITE SUPERIOR PARA MES CORTO
    let fecha: Fecha = Fecha::new(30, 4, 1985);
    assert_eq!(true, fecha.es_fecha_valida());

    // fecha -> 31/12/2001 ✅ - TEST LIMITE SUPERIOR PARA MES LARGO
    let fecha: Fecha = Fecha::new(31, 12, 2001);
    assert_eq!(true, fecha.es_fecha_valida());

    // fecha -> 29/2/2024 ✅ - TEST LIMITE SUPERIOR PARA FEBRERO BISIESTO
    let fecha: Fecha = Fecha::new(29, 2, 2024);
    assert_eq!(true, fecha.es_fecha_valida());

    // fecha -> 29/2/1900 ❌ - TEST LIMITE SUPERIOR PARA FEBRERO NO BISIESTO
    let fecha: Fecha = Fecha::new(29, 02, 1900);
    assert_eq!(false, fecha.es_fecha_valida());

    // fecha -> 0/1/2025 ❌ - TEST DIA 0
    let fecha: Fecha = Fecha::new(0, 1, 2025);
    assert_eq!(false, fecha.es_fecha_valida());

    // fecha -> 31/02/2024 ❌ - TEST FEBRERO CON 31 DIAS
    let fecha: Fecha = Fecha::new(31, 2, 2025);
    assert_eq!(false, fecha.es_fecha_valida());

    // fecha -> 31/4/1690 ❌ - TEST MES CORTO CON 31 DIAS
    let fecha: Fecha = Fecha::new(31, 4, 2025);
    assert_eq!(false, fecha.es_fecha_valida());

    // fecha -> 1/1/2026 ✅ - TEST AÑO MAYOR AL ACTUAL
    let fecha: Fecha = Fecha::new(1, 1, 2026);
    assert_eq!(true, fecha.es_fecha_valida());
}

#[test]
fn test_fecha_to_string() {
    let fecha: Fecha = Fecha::new(1, 1, 2025);
    assert_eq!("1/1/2025".to_string(), fecha.to_string());
}

#[test]
fn test_fecha_restar_dias() {
    let mut fecha: Fecha = Fecha::new(1, 02, 2024);
    fecha.restar_dias(5);
    assert_eq!("27/1/2024", fecha.to_string());

    let mut fecha: Fecha = Fecha::new(1, 02, 2024);
    fecha.restar_dias(600);
    assert_eq!("11/6/2022", fecha.to_string());

    let mut fecha: Fecha = Fecha::new(4, 05, 2025);
    fecha.restar_dias(5498);
    assert_eq!("15/4/2010", fecha.to_string());

    let mut fecha: Fecha = Fecha::new(1, 02, 2024);
    fecha.restar_dias(0);
    assert_eq!("1/2/2024", fecha.to_string());
}

#[test]
fn test_fecha_sumar_dias() {
    let mut fecha: Fecha = Fecha::new(1, 02, 2024);
    fecha.sumar_dias(5);
    assert_eq!("6/2/2024", fecha.to_string());

    let mut fecha: Fecha = Fecha::new(4, 05, 2025);
    fecha.sumar_dias(600);
    assert_eq!("25/12/2026", fecha.to_string());

    let mut fecha: Fecha = Fecha::new(4, 05, 2025);
    fecha.sumar_dias(5498);
    assert_eq!("23/5/2040", fecha.to_string());

    let mut fecha: Fecha = Fecha::new(1, 02, 2024);
    fecha.sumar_dias(0);
    assert_eq!("1/2/2024", fecha.to_string());
}
