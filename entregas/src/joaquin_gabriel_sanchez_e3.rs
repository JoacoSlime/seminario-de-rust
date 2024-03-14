/// 
/// Crates necesarias:
/// - Chrono: https://crates.io/crates/chrono | cargo add chrono
/// - Serde: https://crates.io/crates/serde + Flag de derive | cargo add serde -F derive
/// - Serde_json: https://crates.io/crates/serde_json | cargo add serde_json
/// 

use std::{fs::File, io::Write, collections::HashMap};
use chrono::{Local, Datelike}; 
use serde::{Serialize, Deserialize};


#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Fecha {
    dia: u32,
    mes: u32,
    año: i32,
}

/// 
/// Struct de fecha modificada.
/// 

impl Fecha {
    pub fn new(dia: u32,mes: u32,año: i32) -> Fecha {
        Fecha{
            dia,
            mes,
            año
        }
    }
    fn get_dias_mes(mes: u32) -> u32 {
        let meses:HashMap<u32,u32> = HashMap::from([
            (1,31),
            (2,28),
            (3,31),
            (4,30),
            (5,31),
            (6,30),
            (7,31),
            (8,31),
            (9,30),
            (10,31),
            (11,30),
            (12,31),
        ]);
        meses[&mes]
    }
    pub fn es_fecha_valida(&self) -> bool {
        if self.es_biciesto() && self.mes == 2 {
            self.dia <= 29 && self.dia > 0
        } else if self.mes <= 12 && self.mes > 0 {
            self.dia <= Self::get_dias_mes(self.mes) && self.dia > 0
        } else {
            false
        }
    }
    pub fn es_biciesto(&self) -> bool{
        self.año % 4 == 0
    }
    pub fn sumar_dias (&mut self, dias: u32) {
        self.dia += dias;
        while !self.es_fecha_valida(){
            if self.es_biciesto() && self.mes == 2 {
                self.dia -= Self::get_dias_mes(self.mes) + 1;
                self.mes += 1;
            } else if self.mes < 12 {
                self.dia -= Self::get_dias_mes(self.mes);
                self.mes += 1;
            } else {
                self.dia -= Self::get_dias_mes(self.mes);
                self.año += 1;
                self.mes = 1;
            }
        }
    }

    pub fn restar_dias(&mut self, mut dias: u32) {
        while dias > 0 {
            let a_restar = if dias > self.dia {
                self.dia
            } else {
                dias
            };
            dias -= a_restar;
            self.dia -= a_restar;
            if !self.es_fecha_valida() {
                if self.mes > 1 {
                    self.mes -= 1;
                    self.dia = if self.mes == 2 && self.es_biciesto() {
                        Self::get_dias_mes(self.mes) + 1
                    } else {
                        Self::get_dias_mes(self.mes)
                    };
                } else {
                    self.año -= 1;
                    self.mes = 12;
                    self.dia = 31;
                }
            }
        }
    }

    pub fn es_mayor(&self, una_fecha: &Fecha) -> bool {
        if una_fecha.año < self.año {
            true
        } else if una_fecha.año == self.año {
            if una_fecha.mes < self.mes {
                true
            } else if una_fecha.mes == self.mes {
                una_fecha.dia < self.dia
            } else {
                false
            }
        } else {
            false
        }
    }
}

/// 
/// Ejercicio 3 TP 4 (Modificado)
/// 

#[derive(Debug, PartialEq)]
pub struct StreamingRust<'a> {
    suscripciones: Vec<Suscripcion<'a>>,
    file_path: &'a str,
}

#[derive(Serialize, Deserialize)]
#[derive(Debug, Copy,Clone, PartialEq)]
struct Suscripcion<'a> {
    #[serde(bound(deserialize = "Usuario<'a>: Deserialize<'de>"))]
    usuario: Usuario<'a>,
    tipo: TipoSuscripcion,
    costo_mensual: f64,
    duracion_meses: u8,
    fecha_inicio: Fecha,
}

#[derive(Serialize, Deserialize)]
#[derive(Debug, Copy,Clone, PartialEq)]
struct Usuario<'a> {
    nombre_usuario: &'a str,
    medio_pago: MediosDePago,
}

impl<'a> Usuario<'a> {
    fn new(nombre_usuario: &'a str, medio_pago: MediosDePago) -> Usuario<'a> {
        Usuario {
            nombre_usuario,
            medio_pago
        }
    }
}

#[derive(Serialize, Deserialize)]
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum TipoSuscripcion {
    Basic,
    Clasic,
    Super,
    Cancelado,
}

impl TipoSuscripcion {
    fn get_opciones() -> Vec<TipoSuscripcion> {
        let vec = vec![
            Self::Basic,
            Self::Clasic,
            Self::Super,
            Self::Cancelado
        ];
        vec
    }
}

#[derive(Serialize, Deserialize)]
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum MediosDePago {
    Efectivo,
    MercadoPago,
    TarjetaDeCredito,
    TransferenciaBancaria,
    Cripto,
}

impl MediosDePago {
    fn get_opciones() -> Vec<MediosDePago> {
        let vec = vec![
            Self::Efectivo,
            Self::MercadoPago,
            Self::TarjetaDeCredito,
            Self::TransferenciaBancaria,
            Self::Cripto
        ];
        vec
    }
}

impl<'a> Suscripcion<'a> {
    pub fn new(usuario: Usuario<'a>, tipo: TipoSuscripcion,
            costo_mensual: f64, duracion_meses: u8, fecha_inicio: Fecha) -> Suscripcion<'a> {
        Suscripcion {
            usuario,
            tipo,
            costo_mensual,
            duracion_meses,
            fecha_inicio
        }
    }

    fn this_ts(&self, other: &TipoSuscripcion) -> bool {
        self.tipo == *other
    }

    fn this_mp(&self, other: &MediosDePago) -> bool {
        self.usuario.medio_pago == *other
    }

    fn es_activa(&self) -> bool {
        let hoy = Fecha::new(
            Local::now().day(),
            Local::now().month(),
            Local::now().year(), 
        );
        let mut fecha = self.fecha_inicio;
        fecha.sumar_dias(self.duracion_meses as u32 * 30);
        fecha.es_mayor(&hoy) && self.tipo != TipoSuscripcion::Cancelado
    }
}

impl<'a> StreamingRust<'a> {
    pub fn new_path(path: &'a str) -> StreamingRust<'a> {
        StreamingRust { suscripciones: Vec::new(), file_path: path }
    }

    pub fn new() -> StreamingRust<'a> {
        StreamingRust { suscripciones: Vec::new(), file_path: "suscripciones.json" }
    }
    
    fn existe_usuario(&self, usuario: &'a str) -> bool {
        if self.suscripciones.iter().any(|s| s.usuario.nombre_usuario == usuario) {
            return true;
        };
        false
    }

    fn buscar_sub_usuario_mut(&mut self, nombre: &'a str) -> Option<&mut Suscripcion<'a>> {
        self.suscripciones.iter_mut().find(|s| s.usuario.nombre_usuario == nombre)
    }

    pub fn crear_usuario(&mut self, usuario: &'a str, medio_pago: MediosDePago,
    tipo: TipoSuscripcion, costo_mensual: f64, duracion_meses: u8, fecha_inicio: Fecha) {
        if self.existe_usuario(usuario) {
            return;
        } 
        let suscripcion = Suscripcion::new(Usuario::new(usuario,
            medio_pago), tipo, costo_mensual, duracion_meses, fecha_inicio);
        self.suscripciones.push(suscripcion);
        self.actualizar_archivo(); // Agregado para el inciso b
    }

    pub fn upgrade_suscription(&mut self, usuario: &'a str) {
        let sub = self.buscar_sub_usuario_mut(usuario);
        if let Some(mut sub) = sub {
            match sub.tipo {
                TipoSuscripcion::Basic => {
                    sub.tipo = TipoSuscripcion::Clasic;
                    sub.costo_mensual = 1799.99;
                    self.actualizar_archivo(); // Agregado para el inciso b
                },
                TipoSuscripcion::Clasic  => {
                    sub.tipo = TipoSuscripcion::Super;
                    sub.costo_mensual = 2899.99;
                    self.actualizar_archivo(); // Agregado para el inciso b
                },
                TipoSuscripcion::Cancelado => {
                    sub.tipo = TipoSuscripcion::Basic;
                    sub.costo_mensual = 999.99;
                    self.actualizar_archivo(); // Agregado para el inciso b
                }
                _ => ()
            }
        }
    }

    pub fn downgrade_suscription(&mut self, usuario: &'a str) {
        let sub = self.buscar_sub_usuario_mut(usuario);
        if let Some(mut sub) = sub {
            match sub.tipo {
                TipoSuscripcion::Clasic => {
                    sub.tipo = TipoSuscripcion::Basic;
                    sub.costo_mensual = 999.99;
                    self.actualizar_archivo(); // Agregado para el inciso b
                },
                TipoSuscripcion::Super  => {
                    sub.tipo = TipoSuscripcion::Clasic;
                    sub.costo_mensual = 1799.99;
                    self.actualizar_archivo(); // Agregado para el inciso b
                },
                TipoSuscripcion::Basic => {
                    sub.tipo = TipoSuscripcion::Cancelado;
                    sub.costo_mensual = 0.0;
                    sub.duracion_meses = 0;
                    self.actualizar_archivo(); // Agregado para el inciso b
                }
                _ => ()
            }
        }
    }

    pub fn cancel_suscription(&mut self, usuario: &'a str) {
        let sub = self.buscar_sub_usuario_mut(usuario);
        if let Some(mut sub) = sub {
            sub.tipo = TipoSuscripcion::Cancelado;
            sub.costo_mensual = 0.0;
            sub.duracion_meses = 0;
            self.actualizar_archivo(); // Agregado para el inciso b
        }
    }

    pub fn top_active_payment_option(&self) -> Option<MediosDePago> {
        let opciones = MediosDePago::get_opciones();
        let suscripciones = self.suscripciones
            .iter()
            .filter(|s| s.tipo != TipoSuscripcion::Cancelado);
        let mut mp_max: Option<MediosDePago> = None;
        let mut max = 0;
        for mp in opciones {
            let cont = suscripciones.clone().filter(|s| s.this_mp(&mp)).count();
            if cont > max {
                max = cont;
                mp_max = Some(mp);
            }
        }
        mp_max
    }

    pub fn top_active_suscription_option(&self) -> Option<TipoSuscripcion> {
        let opciones = TipoSuscripcion::get_opciones();
        let suscripciones = self.suscripciones
            .iter()
            .filter(|s| s.tipo != TipoSuscripcion::Cancelado);
        let mut ts_max: Option<TipoSuscripcion> = None;
        let mut max = 0;
        for ts in opciones {
            let cont = suscripciones.clone().filter(|s| s.this_ts(&ts)).count();
            if cont > max {
                max = cont;
                ts_max = Some(ts);
            }
        }
        ts_max
    }
    

    pub fn top_payment_option(&self) -> Option<MediosDePago> {
        let opciones = MediosDePago::get_opciones();
        let suscripciones = self.suscripciones
            .iter();
        let mut mp_max: Option<MediosDePago> = None;
        let mut max = 0;
        for mp in opciones {
            let cont = suscripciones.clone().filter(|s| s.this_mp(&mp)).count();
            if cont > max {
                max = cont;
                mp_max = Some(mp);
            }
        }
        mp_max
    }

    pub fn top_suscription_option(&self) -> Option<TipoSuscripcion> {
        let opciones = TipoSuscripcion::get_opciones();
        let suscripciones = self.suscripciones.iter();
        let mut ts_max: Option<TipoSuscripcion> = None;
        let mut max = 0;
        for ts in opciones {
            let cont = suscripciones.clone().filter(|s| s.this_ts(&ts)).count();
            if cont > max {
                max = cont;
                ts_max = Some(ts);
            }
        }
        ts_max
    }

    fn actualizar_archivo(&self) {
        let json = serde_json::to_string_pretty(&self.suscripciones)
            .expect("Error al serializar");
        let mut file = File::create(self.file_path)
            .expect("Error al crear archivo.");
        file.write_all(json.as_bytes()).expect("Error al escribir");
    }
}

/// 
/// Tests para Fecha
/// 

#[cfg(test)]
#[test]
fn new_fecha_test() {
    let exp = Fecha {
        dia: 1,
        mes: 2,
        año: 2003,
    };
    let res = Fecha::new(1, 2, 2003);
    assert_eq!(exp, res);
}

#[cfg(test)]
#[test]
fn get_dias_mes_test() {
    let meses:HashMap<u32,u32> = HashMap::from([
        (1,31),
        (2,28),
        (3,31),
        (4,30),
        (5,31),
        (6,30),
        (7,31),
        (8,31),
        (9,30),
        (10,31),
        (11,30),
        (12,31),
    ]);
    for i in meses {
        let exp = i.1;
        let res = Fecha::get_dias_mes(i.0);
        assert_eq!(exp, res);
    }
}

#[cfg(test)]
#[test]
fn es_fecha_valida_test() {
    let f = Fecha::new(0,0,0);
    assert!(!f.es_fecha_valida());
    let f = Fecha::new(32,12,2000);
    assert!(!f.es_fecha_valida());
    let f = Fecha::new(32,0,2000);
    assert!(!f.es_fecha_valida());
    let f = Fecha::new(0,2,2000);
    assert!(!f.es_fecha_valida());
    let f = Fecha::new(0,2,2001);
    assert!(!f.es_fecha_valida());
    let f = Fecha::new(31,13,2000);
    assert!(!f.es_fecha_valida());
    let f = Fecha::new(12,12,2002);
    assert!(f.es_fecha_valida());
    let f = Fecha::new(29,2,2003);
    assert!(!f.es_fecha_valida());
    let f = Fecha::new(29,2,2004);
    assert!(f.es_fecha_valida());
}

#[cfg(test)]
#[test]
fn es_biciesto_test() {
    let f = Fecha::new(31,13,2000);
    assert!(f.es_biciesto());
    let f = Fecha::new(12,12,2002);
    assert!(!f.es_biciesto());
    let f = Fecha::new(29,2,2003);
    assert!(!f.es_biciesto());
    let f = Fecha::new(29,2,2004);
    assert!(f.es_biciesto());
}

#[cfg(test)]
#[test]
fn sumar_dias_test() {
    let exp = Fecha::new(1,1,2024);
    let mut res = Fecha::new(20,12,2023);
    res.sumar_dias(12);
    assert_eq!(exp, res);
    let exp = Fecha::new(1,2,2024);
    let mut res = Fecha::new(20,12,2023);
    res.sumar_dias(43);
    assert_eq!(exp, res);
    let exp = Fecha::new(1,3,2004);
    let mut res = Fecha::new(28,2,2004);
    res.sumar_dias(2);
    assert_eq!(exp, res);
    let exp = Fecha::new(1,3,2003);
    let mut res = Fecha::new(28,2,2003);
    res.sumar_dias(1);
    assert_eq!(exp, res);
}

#[cfg(test)]
#[test]
fn restar_dias_test() {
    let exp = Fecha::new(20,12,2023);
    let mut res = Fecha::new(1,1,2024);
    res.restar_dias(12);
    assert_eq!(exp, res);
    let exp = Fecha::new(20,12,2023);
    let mut res = Fecha::new(1,2,2024);
    res.restar_dias(43);
    assert_eq!(exp, res);
    let exp = Fecha::new(28,2,2004);
    let mut res = Fecha::new(1,3,2004);
    res.restar_dias(2);
    assert_eq!(exp, res);
    let exp = Fecha::new(28,2,2003);
    let mut res = Fecha::new(1,3,2003);
    res.restar_dias(1);
    assert_eq!(exp, res);
}

#[cfg(test)]
#[test]
fn es_mayor_test() {
    let f1 = Fecha::new(15,5,2023);
    let f2 = Fecha::new(16,5,2023);
    assert!(f2.es_mayor(&f1));
    assert!(!f1.es_mayor(&f2));
    let f1 = Fecha::new(15,5,2023);
    let f2 = Fecha::new(15,6,2023);
    assert!(f2.es_mayor(&f1));
    assert!(!f1.es_mayor(&f2));
    let f1 = Fecha::new(15,5,2023);
    let f2 = Fecha::new(15,5,2024);
    assert!(f2.es_mayor(&f1));
    assert!(!f1.es_mayor(&f2));
}


/// 
/// Tests para TipoSuscripcion
/// 

#[cfg(test)]
#[test]
fn tipo_suscripcion_get_opciones_test() {
    let exp = vec![
        TipoSuscripcion::Basic,
        TipoSuscripcion::Clasic,
        TipoSuscripcion::Super,
        TipoSuscripcion::Cancelado
    ];
    let res = TipoSuscripcion::get_opciones();
    assert_eq!(exp, res);
}

/// 
/// Tests para MediosDePago
/// 

#[cfg(test)]
#[test]
fn medio_pago_get_opciones_test() {
    let exp = vec![
        MediosDePago::Efectivo,
        MediosDePago::MercadoPago,
        MediosDePago::TarjetaDeCredito,
        MediosDePago::TransferenciaBancaria,
        MediosDePago::Cripto
    ];
    let res = MediosDePago::get_opciones();
    assert_eq!(exp, res);
}

/// 
/// Tests para User
/// 

#[cfg(test)]
#[test]
fn new_user_test() {
    let exp = Usuario {
        nombre_usuario: "test",
        medio_pago: MediosDePago::Efectivo
    };
    let res = Usuario::new("test", MediosDePago::Efectivo);
    assert_eq!(exp, res);
}


/// 
/// Tests para Suscripcion
/// 

#[cfg(test)]
#[test]
fn new_sub_test() {
    let exp =  Suscripcion {
        usuario: Usuario {
            nombre_usuario: "test",
            medio_pago: MediosDePago::Efectivo
        },
        tipo: TipoSuscripcion::Basic,
        costo_mensual: f64::default(),
        duracion_meses: u8::default(),
        fecha_inicio: Fecha::new(1, 1, 1997)
    };
    let res: Suscripcion<'_> = Suscripcion::new(
        Usuario {
            nombre_usuario: "test",
            medio_pago: MediosDePago::Efectivo
        },
        TipoSuscripcion::Basic,
        f64::default(),
        u8::default(),
        Fecha::new(1, 1, 1997),
    );
    assert_eq!(exp,res);
}

#[cfg(test)]
#[test]
fn this_ts_test() {
    let s = Suscripcion::new(
        Usuario {
            nombre_usuario: "test",
            medio_pago: MediosDePago::Efectivo
        },
        TipoSuscripcion::Basic,
        f64::default(),
        u8::default(),
        Fecha::new(1, 1, 1997),
    );
    assert!(s.this_ts(&TipoSuscripcion::Basic));
}

#[cfg(test)]
#[test]
fn this_mp_test() {
    let s: Suscripcion<'_> = Suscripcion::new(
        Usuario {
            nombre_usuario: "test",
            medio_pago: MediosDePago::Efectivo
        },
        TipoSuscripcion::Basic,
        f64::default(),
        u8::default(),
        Fecha::new(1, 1, 1997),
    );
    assert!(s.this_mp(&MediosDePago::Efectivo));
}

#[cfg(test)]
#[test]
fn es_activa_test() {
    let s1 = Suscripcion::new(
        Usuario {
            nombre_usuario: "test",
            medio_pago: MediosDePago::Efectivo
        },
        TipoSuscripcion::Basic,
        f64::default(),
        2,
        Fecha::new(1, 1, 1997),
    );
    let s2 = Suscripcion::new(
        Usuario {
            nombre_usuario: "test",
            medio_pago: MediosDePago::Efectivo
        },
        TipoSuscripcion::Basic,
        f64::default(),
        1,
        Fecha::new(Local::now().day(), Local::now().month(), Local::now().year()),
    );
    assert!(!s1.es_activa());
    assert!(s2.es_activa());
}

/// 
/// Tests para StreamingRust
/// 

#[cfg(test)]
#[test]
fn new_sr_test() {
    let exp = StreamingRust{
        suscripciones: Vec::new(),
        file_path: "suscripciones.json",
    };
    let res = StreamingRust::new();
    assert_eq![exp,res];
}

#[cfg(test)]
#[test]
fn new_path_sr_test() {
    let exp = StreamingRust{
        suscripciones: Vec::new(),
        file_path: "new_sr_test.json",
    };
    let res = StreamingRust::new_path("new_sr_test.json");
    assert_eq![exp,res];
}

#[cfg(test)]
#[test]
fn existe_usuario_test() {
    let rs = StreamingRust {
        suscripciones: vec![
            Suscripcion::new(
                Usuario {
                    nombre_usuario: "test",
                    medio_pago: MediosDePago::Efectivo
                },
                TipoSuscripcion::Basic,
                999.99,
                1,
                Fecha::new(Local::now().day(), Local::now().month(), Local::now().year()),
            )
        ],
        file_path: "existe_usuario_test.json",
    };
    let res = rs.existe_usuario("no_existe");
    assert!(!res);
    let res = rs.existe_usuario("test");
    assert!(res);
}

#[cfg(test)]
#[test]
fn buscar_sub_usuario_mut_test() {
    let mut rs = StreamingRust {
        suscripciones: vec![
            Suscripcion::new(
                Usuario {
                    nombre_usuario: "test",
                    medio_pago: MediosDePago::Efectivo
                },
                TipoSuscripcion::Basic,
                999.99,
                1,
                Fecha::new(Local::now().day(), Local::now().month(), Local::now().year()),
            )
        ],
        file_path: "buscar_sub_usuario_mut_test.json",
    };
    let res = rs.buscar_sub_usuario_mut("no_existe");
    assert!(res.is_none());
    let res = rs.buscar_sub_usuario_mut("test");
    assert!(res.is_some());
}

#[cfg(test)]
#[test]
fn crear_usuario_test() {
    let exp = StreamingRust {
        suscripciones: vec![
            Suscripcion::new(
                Usuario {
                    nombre_usuario: "test",
                    medio_pago: MediosDePago::Efectivo
                },
                TipoSuscripcion::Basic,
                999.99,
                1,
                Fecha::new(Local::now().day(), Local::now().month(), Local::now().year()),
            )
        ],
        file_path: "crear_usuario_test.json",
    };
    let mut res = StreamingRust {
        suscripciones: Vec::new(),
        file_path: "crear_usuario_test.json",
    };
    res.crear_usuario(
        "test",
        MediosDePago::Efectivo,
        TipoSuscripcion::Basic,
        999.99, 1,
        Fecha::new(Local::now().day(), Local::now().month(), Local::now().year())
    );
    assert_eq!(exp,res);
    res.crear_usuario(
        "test",
        MediosDePago::Efectivo,
        TipoSuscripcion::Basic,
        999.99, 1,
        Fecha::new(Local::now().day(), Local::now().month(), Local::now().year())
    );
    assert_eq!(exp,res);
    
    let exp = StreamingRust {
        suscripciones: vec![
            Suscripcion::new(
                Usuario {
                    nombre_usuario: "test",
                    medio_pago: MediosDePago::Efectivo
                },
                TipoSuscripcion::Basic,
                999.99,
                1,
                Fecha::new(Local::now().day(), Local::now().month(), Local::now().year()),
            ),
            Suscripcion::new(
                Usuario {
                    nombre_usuario: "test2",
                    medio_pago: MediosDePago::TarjetaDeCredito
                },
                TipoSuscripcion::Super,
                2899.99,
                3,
                Fecha::new(Local::now().day(), Local::now().month(), Local::now().year()),
            )
        ],
        file_path: "crear_usuario_test.json",
    };
    res.crear_usuario(
        "test2",
        MediosDePago::TarjetaDeCredito,
        TipoSuscripcion::Super,
        2899.99, 3,
        Fecha::new(Local::now().day(), Local::now().month(), Local::now().year())
    );
    assert_eq!(exp,res);
}

#[cfg(test)]
#[test]
fn upgrade_suscription_test() {
    let estado1 = StreamingRust {
        suscripciones: vec![
            Suscripcion::new(
                Usuario {
                    nombre_usuario: "test",
                    medio_pago: MediosDePago::Efectivo
                },
                TipoSuscripcion::Basic,
                999.99,
                1,
                Fecha::new(Local::now().day(), Local::now().month(), Local::now().year()),
            )
        ],
        file_path: "upgrade_suscription_test.json",
    };
    let estado2 = StreamingRust {
        suscripciones: vec![
            Suscripcion::new(
                Usuario {
                    nombre_usuario: "test",
                    medio_pago: MediosDePago::Efectivo
                },
                TipoSuscripcion::Clasic,
                1799.99,
                1,
                Fecha::new(Local::now().day(), Local::now().month(), Local::now().year()),
            )
        ],
        file_path: "upgrade_suscription_test.json",
    };
    let estado3 = StreamingRust {
        suscripciones: vec![
            Suscripcion::new(
                Usuario {
                    nombre_usuario: "test",
                    medio_pago: MediosDePago::Efectivo
                },
                TipoSuscripcion::Super,
                2899.99,
                1,
                Fecha::new(Local::now().day(), Local::now().month(), Local::now().year()),
            )
        ],
        file_path: "upgrade_suscription_test.json",
    };
    let mut res = StreamingRust {
        suscripciones: vec![
            Suscripcion::new(
                Usuario {
                    nombre_usuario: "test",
                    medio_pago: MediosDePago::Efectivo
                },
                TipoSuscripcion::Cancelado,
                0.0,
                1,
                Fecha::new(Local::now().day(), Local::now().month(), Local::now().year()),
            )
        ],
        file_path: "upgrade_suscription_test.json",
    };
    let u = "test";
    
    res.upgrade_suscription(u);
    assert_eq!(estado1, res);
    res.upgrade_suscription(u);
    assert_eq!(estado2, res);
    res.upgrade_suscription(u);
    assert_eq!(estado3, res);
    res.upgrade_suscription(u);
    assert_eq!(estado3, res);
}

#[cfg(test)]
#[test]
fn downgrade_suscription_test() {
    let estado1 = StreamingRust {
        suscripciones: vec![
            Suscripcion::new(
                Usuario {
                    nombre_usuario: "test",
                    medio_pago: MediosDePago::Efectivo
                },
                TipoSuscripcion::Clasic,
                1799.99,
                1,
                Fecha::new(Local::now().day(), Local::now().month(), Local::now().year()),
            )
        ],
        file_path: "downgrade_suscription_test.json",
    };
    let estado2 = StreamingRust {
        suscripciones: vec![
            Suscripcion::new(
                Usuario {
                    nombre_usuario: "test",
                    medio_pago: MediosDePago::Efectivo
                },
                TipoSuscripcion::Basic,
                999.99,
                1,
                Fecha::new(Local::now().day(), Local::now().month(), Local::now().year()),
            )
        ],
        file_path: "downgrade_suscription_test.json",
    };
    let estado3 = StreamingRust {
        suscripciones: vec![
            Suscripcion::new(
                Usuario {
                    nombre_usuario: "test",
                    medio_pago: MediosDePago::Efectivo
                },
                TipoSuscripcion::Cancelado,
                0.0,
                0,
                Fecha::new(Local::now().day(), Local::now().month(), Local::now().year()),
            )
        ],
        file_path: "downgrade_suscription_test.json",
    };
    let mut res = StreamingRust {
        suscripciones: vec![
            Suscripcion::new(
                Usuario {
                    nombre_usuario: "test",
                    medio_pago: MediosDePago::Efectivo
                },
                TipoSuscripcion::Super,
                2899.99,
                1,
                Fecha::new(Local::now().day(), Local::now().month(), Local::now().year()),
            )
        ],
        file_path: "downgrade_suscription_test.json",
    };

    let u = "test";
    
    res.downgrade_suscription(u);
    assert_eq!(estado1, res);
    res.downgrade_suscription(u);
    assert_eq!(estado2, res);
    res.downgrade_suscription(u);
    assert_eq!(estado3, res);
    res.downgrade_suscription(u);
    assert_eq!(estado3, res);
}

#[cfg(test)]
#[test]
fn cancel_suscription_test() {
    let exp = StreamingRust {
        suscripciones: vec![
            Suscripcion::new(
                Usuario {
                    nombre_usuario: "test",
                    medio_pago: MediosDePago::Efectivo
                },
                TipoSuscripcion::Cancelado,
                0.0,
                0,
                Fecha::new(Local::now().day(), Local::now().month(), Local::now().year()),
            )
        ],
        file_path: "cancel_suscription_test.json",
    };
    let mut res = StreamingRust {
        suscripciones: vec![
            Suscripcion::new(
                Usuario {
                    nombre_usuario: "test",
                    medio_pago: MediosDePago::Efectivo
                },
                TipoSuscripcion::Super,
                2899.99,
                1,
                Fecha::new(Local::now().day(), Local::now().month(), Local::now().year()),
            )
        ],
        file_path: "cancel_suscription_test.json",
    };
    let u = "test";
    res.cancel_suscription(u);
    assert_eq!(exp, res);
}

#[cfg(test)]
#[test]
fn top_active_payment_option_test() {
    let fake_sr =  StreamingRust {
        suscripciones: Vec::new(),
        file_path: "top_active_payment_option_test.json",
    };
    let sr = StreamingRust {
        suscripciones: vec![
            Suscripcion::new(
                Usuario {
                    nombre_usuario: "test1",
                    medio_pago: MediosDePago::Efectivo
                },
                TipoSuscripcion::Super,
                2899.99,
                1,
                Fecha::new(Local::now().day(), Local::now().month(), Local::now().year()),
            ),
            Suscripcion::new(
                Usuario {
                    nombre_usuario: "test2",
                    medio_pago: MediosDePago::Efectivo
                },
                TipoSuscripcion::Clasic,
                2899.99,
                1,
                Fecha::new(Local::now().day(), Local::now().month(), Local::now().year()),
            ),
            Suscripcion::new(
                Usuario {
                    nombre_usuario: "test3",
                    medio_pago: MediosDePago::TarjetaDeCredito
                },
                TipoSuscripcion::Clasic,
                2899.99,
                1,
                Fecha::new(Local::now().day(), Local::now().month(), Local::now().year()),
            ),
            Suscripcion::new(
                Usuario {
                    nombre_usuario: "test3",
                    medio_pago: MediosDePago::Cripto
                },
                TipoSuscripcion::Basic,
                2899.99,
                1,
                Fecha::new(Local::now().day(), Local::now().month(), Local::now().year()),
            ),
            Suscripcion::new(
                Usuario {
                    nombre_usuario: "test4",
                    medio_pago: MediosDePago::Cripto
                },
                TipoSuscripcion::Cancelado,
                2899.99,
                1,
                Fecha::new(Local::now().day(), Local::now().month(), Local::now().year()),
            )
        ],
        file_path: "top_active_payment_option_test.json",
    };
    let exp = Some(MediosDePago::Efectivo);
    let res = sr.top_active_payment_option();
    assert!(fake_sr.top_active_payment_option().is_none());
    assert!(res.is_some());
    assert_eq!(exp,res);
}

#[cfg(test)]
#[test]
fn top_active_suscription_option_test() {
    let fake_sr =  StreamingRust {
        suscripciones: Vec::new(),
        file_path: "top_active_suscription_option_test.json",
    };
    let sr = StreamingRust {
        suscripciones: vec![
            Suscripcion::new(
                Usuario {
                    nombre_usuario: "test1",
                    medio_pago: MediosDePago::Efectivo
                },
                TipoSuscripcion::Super,
                2899.99,
                1,
                Fecha::new(Local::now().day(), Local::now().month(), Local::now().year()),
            ),
            Suscripcion::new(
                Usuario {
                    nombre_usuario: "test2",
                    medio_pago: MediosDePago::Efectivo
                },
                TipoSuscripcion::Clasic,
                2899.99,
                1,
                Fecha::new(Local::now().day(), Local::now().month(), Local::now().year()),
            ),
            Suscripcion::new(
                Usuario {
                    nombre_usuario: "test2",
                    medio_pago: MediosDePago::TarjetaDeCredito
                },
                TipoSuscripcion::Clasic,
                2899.99,
                1,
                Fecha::new(Local::now().day(), Local::now().month(), Local::now().year()),
            ),
            Suscripcion::new(
                Usuario {
                    nombre_usuario: "test3",
                    medio_pago: MediosDePago::Cripto
                },
                TipoSuscripcion::Basic,
                2899.99,
                1,
                Fecha::new(Local::now().day(), Local::now().month(), Local::now().year()),
            ),
            Suscripcion::new(
                Usuario {
                    nombre_usuario: "test4",
                    medio_pago: MediosDePago::Cripto
                },
                TipoSuscripcion::Cancelado,
                2899.99,
                1,
                Fecha::new(Local::now().day(), Local::now().month(), Local::now().year()),
            )
        ],
        file_path: "top_active_suscription_option_test.json",
    };
    let exp = Some(TipoSuscripcion::Clasic);
    let res = sr.top_active_suscription_option();
    assert!(fake_sr.top_active_suscription_option().is_none());
    assert!(res.is_some());
    assert_eq!(exp,res);
}

#[cfg(test)]
#[test]
fn top_payment_option_test() {
    let fake_sr =  StreamingRust {
        suscripciones: Vec::new(),
        file_path: "top_payment_option_test.json",
    };
    let sr = StreamingRust {
        suscripciones: vec![
            Suscripcion::new(
                Usuario {
                    nombre_usuario: "test1",
                    medio_pago: MediosDePago::Efectivo
                },
                TipoSuscripcion::Super,
                2899.99,
                1,
                Fecha::new(Local::now().day(), Local::now().month(), Local::now().year()),
            ),
            Suscripcion::new(
                Usuario {
                    nombre_usuario: "test2",
                    medio_pago: MediosDePago::Efectivo
                },
                TipoSuscripcion::Clasic,
                2899.99,
                1,
                Fecha::new(Local::now().day(), Local::now().month(), Local::now().year()),
            ),
            Suscripcion::new(
                Usuario {
                    nombre_usuario: "test1",
                    medio_pago: MediosDePago::TransferenciaBancaria
                },
                TipoSuscripcion::Clasic,
                2899.99,
                1,
                Fecha::new(Local::now().day(), Local::now().month(), Local::now().year()),
            ),
            Suscripcion::new(
                Usuario {
                    nombre_usuario: "test3",
                    medio_pago: MediosDePago::Cripto
                },
                TipoSuscripcion::Basic,
                2899.99,
                1,
                Fecha::new(Local::now().day(), Local::now().month(), Local::now().year()),
            ),
            Suscripcion::new(
                Usuario {
                    nombre_usuario: "test4",
                    medio_pago: MediosDePago::Cripto
                },
                TipoSuscripcion::Cancelado,
                2899.99,
                1,
                Fecha::new(Local::now().day(), Local::now().month(), Local::now().year()),
            ),
            
            Suscripcion::new(
                Usuario {
                    nombre_usuario: "test4",
                    medio_pago: MediosDePago::Cripto
                },
                TipoSuscripcion::Cancelado,
                2899.99,
                1,
                Fecha::new(Local::now().day(), Local::now().month(), Local::now().year()),
            )
        ],
        file_path: "top_payment_option_test.json",
    };
    let exp = Some(MediosDePago::Cripto);
    let res = sr.top_payment_option();
    assert!(fake_sr.top_payment_option().is_none());
    assert!(res.is_some());
    assert_eq!(exp,res);
}

#[cfg(test)]
#[test]
fn top_suscription_option_test() {
    let fake_sr =  StreamingRust {
        suscripciones: Vec::new(),
        file_path: "top_suscription_option_test.json",
    };
    let sr = StreamingRust {
        suscripciones: vec![
            Suscripcion::new(
                Usuario {
                    nombre_usuario: "test1",
                    medio_pago: MediosDePago::Efectivo
                },
                TipoSuscripcion::Super,
                2899.99,
                1,
                Fecha::new(Local::now().day(), Local::now().month(), Local::now().year()),
            ),
            Suscripcion::new(
                Usuario {
                    nombre_usuario: "test2",
                    medio_pago: MediosDePago::Efectivo
                },
                TipoSuscripcion::Clasic,
                2899.99,
                1,
                Fecha::new(Local::now().day(), Local::now().month(), Local::now().year()),
            ),
            Suscripcion::new(
                Usuario {
                    nombre_usuario: "test2",
                    medio_pago: MediosDePago::TarjetaDeCredito
                },
                TipoSuscripcion::Clasic,
                2899.99,
                1,
                Fecha::new(Local::now().day(), Local::now().month(), Local::now().year()),
            ),
            Suscripcion::new(
                Usuario {
                    nombre_usuario: "test3",
                    medio_pago: MediosDePago::Cripto
                },
                TipoSuscripcion::Cancelado,
                2899.99,
                1,
                Fecha::new(Local::now().day(), Local::now().month(), Local::now().year()),
            ),
            Suscripcion::new(
                Usuario {
                    nombre_usuario: "test4",
                    medio_pago: MediosDePago::Cripto
                },
                TipoSuscripcion::Cancelado,
                2899.99,
                1,
                Fecha::new(Local::now().day(), Local::now().month(), Local::now().year()),
            ),
            
            Suscripcion::new(
                Usuario {
                    nombre_usuario: "test4",
                    medio_pago: MediosDePago::Cripto
                },
                TipoSuscripcion::Cancelado,
                2899.99,
                1,
                Fecha::new(Local::now().day(), Local::now().month(), Local::now().year()),
            )
        ],
        file_path: "top_suscription_option_test.json",
    };
    let exp = Some(TipoSuscripcion::Cancelado);
    let res = sr.top_suscription_option();
    assert!(fake_sr.top_suscription_option().is_none());
    assert!(res.is_some());
    assert_eq!(exp,res);
}

#[cfg(test)]
#[test]
fn actualizar_archivo_test() {
    use std::io::Read;

    let sr = StreamingRust::new_path("actualizar_archivo_test1.json");
    let exp = r#"[]"#.to_string();
    sr.actualizar_archivo();
    let mut file = File::open("actualizar_archivo_test1.json").expect("Error al abrir archivo");
    let mut res = String::new();
    file.read_to_string(&mut res).expect("Error al leer archivo");
    assert_eq!(exp, res);
    let mut sr = StreamingRust::new_path("actualizar_archivo_test2.json");
    sr.crear_usuario(
        "test",
        MediosDePago::Efectivo,
        TipoSuscripcion::Basic,
        999.99, 1,
        Fecha::new(Local::now().day(), Local::now().month(), Local::now().year())
    );
    let mut file = File::open("actualizar_archivo_test2.json").expect("Error al abrir archivo");
    let mut res = String::new();
    file.read_to_string(&mut res).expect("Error al leer archivo");
    assert_ne!(exp, res);
}