use std::{collections::HashMap};
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
            self.dia <= 29
        } else if self.mes < 12 && self.mes > 0 {
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
            if self.mes < 12 {
                self.dia -= Self::get_dias_mes(self.mes);
                self.mes += 1;
            } else {
                self.dia -= 31;
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

pub trait EsPrimo {
    fn es_primo(&self) -> bool; 
}

impl EsPrimo for i32 {
    fn es_primo(&self) -> bool {
        if self.abs() > 2 {
            for i in 0..*self {
                if self % i == 0 {
                    return false
                }
            }
            return true
        }
        false
    }
}

pub fn contar_primos(vec: &Vec<i32>) -> usize {
    vec.iter().filter(|x| x.es_primo()).count()
}

pub struct Persona<'a>{
    nombre:&'a str,
    apellido:&'a str,
    direccion:&'a str,
    ciudad:&'a str,
    salario:f64,
    edad:u8,
}

pub fn salarios_altos<'a> (vec: &Vec<Persona>, salario: &f64) -> usize {
    vec.iter().filter(|p| p.salario > *salario).count()
}

pub fn personas_mayores<'a> (vec: &'a Vec<Persona<'a>>, edad: &u8, ciudad: &'a str) -> Vec<&'a Persona<'a>> {
    vec.iter().filter(|p| p.edad > *edad && p.ciudad > ciudad).collect()
}

pub fn misma_ciudad (vec: &Vec<Persona>, ciudad: &str) -> bool {
    vec.iter().all(|p| p.ciudad==ciudad)
}

pub fn existe_residente (vec: &Vec<Persona>, ciudad: &str) -> bool {
    vec.iter().any(|p| p.ciudad == ciudad)
}

pub fn existe_persona (arr: &[Persona], persona: &Persona) -> bool {
    arr.iter().any(|p| p.nombre == persona.nombre && p.apellido == persona.apellido &&
        p.ciudad == persona.ciudad && p.edad == persona.edad && 
        p.direccion == persona.direccion && p.salario == persona.salario)
}

pub fn arreglo_edades<'a, const N: usize> (arr: &'a [Persona<'a>; N]) -> [u8;N] {
    arr.iter().map(|p| p.edad).collect::<Vec<u8>>().try_into().expect("Error al convertir el Vector en Arreglo")
}

pub fn menor_salario<'a> (arr: &'a [Persona<'a>]) -> Option<(&'a Persona<'a>,&'a Persona<'a>)> {
    let min = arr.iter().min_by(|p1, p2| {
        p1.salario.partial_cmp(&p2.salario)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| p2.edad.cmp(&p1.edad))
    });
    
    let max = arr.iter().max_by(|p1, p2|
        p1.salario.partial_cmp(&p2.salario)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| p2.edad.cmp(&p1.edad))
    );
    if min.is_some() {
        return Some((min.unwrap(),max.unwrap()));
    };
    None
}

/// 
/// REVISAR DESDE ESTE PUNTO.
/// 

pub struct StreamingRust<'a> {
    suscripciones: Vec<Suscripcion<'a>>,
}

#[derive(Copy,Clone)]
struct Suscripcion<'a> {
    usuario: &'a str,
    tipo: TipoSuscripcion,
    costo_mensual: f64,
    duracion_meses: u8,
    fecha_inicio: Fecha,
    medio_pago: MediosDePago,
}

#[derive(PartialEq, Eq, Copy, Clone)]
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

#[derive(PartialEq, Eq, Copy, Clone)]
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
    pub fn new(usuario: &'a str, medio_pago: MediosDePago, tipo: TipoSuscripcion,
            costo_mensual: f64, duracion_meses: u8, fecha_inicio: Fecha) -> Suscripcion<'a> {
        Suscripcion {
            usuario,
            medio_pago,
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
        self.medio_pago == *other
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
    pub fn crear_usuario(&mut self, usuario: &'a str, medio_pago: MediosDePago, tipo: TipoSuscripcion, costo_mensual: f64, duracion_meses: u8, fecha_inicio: Fecha) {
        if self.suscripciones.iter().any(|s| s.usuario == usuario) {
            return;
        } 
        let suscripcion = Suscripcion::new(usuario, medio_pago, tipo, costo_mensual, duracion_meses, fecha_inicio);
        self.suscripciones.push(suscripcion);
    }

    pub fn upgrade_suscription(&mut self, usuario: &str) {
        if let Some(mut sub) = self.suscripciones.iter_mut().find(|s| s.usuario == usuario){
            match sub.tipo {
                TipoSuscripcion::Basic => {
                    sub.tipo = TipoSuscripcion::Clasic;
                    sub.costo_mensual = 1799.99;
                },
                TipoSuscripcion::Clasic  => {
                    sub.tipo = TipoSuscripcion::Super;
                    sub.costo_mensual = 2899.99;
                },
                TipoSuscripcion::Cancelado => {
                    sub.tipo = TipoSuscripcion::Basic;
                    sub.costo_mensual = 999.99
                }
                _ => ()
            }
        }
    }

    pub fn downgrade_suscription(&mut self, usuario: &str) {
        if let Some(mut sub) = self.suscripciones.iter_mut().find(|s| s.usuario == usuario){
            match sub.tipo {
                TipoSuscripcion::Clasic => {
                    sub.tipo = TipoSuscripcion::Basic;
                    sub.costo_mensual = 999.99;
                },
                TipoSuscripcion::Super  => {
                    sub.tipo = TipoSuscripcion::Clasic;
                    sub.costo_mensual = 1799.99;
                },
                TipoSuscripcion::Basic => {
                    sub.tipo = TipoSuscripcion::Cancelado;
                    sub.costo_mensual = 0.0;
                    sub.duracion_meses = 0;
                }
                _ => ()
            }
        }
    }

    pub fn cancel_suscription(&mut self, usuario: &str) {
        if let Some(s) = self.suscripciones.iter_mut().find(|s| s.usuario == usuario) {
            s.tipo = TipoSuscripcion::Cancelado;
            s.costo_mensual = 0.0;
            s.duracion_meses = 0;
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
}

struct Ventas<'a> {
    registro_ventas: Vec<UnaVenta<'a>>,
}

impl<'a> Ventas<'a> {
    pub fn agregar_venta(&mut self, fecha: Fecha, productos: Vec<Producto<'a>>, cliente: Cliente<'a>, 
    vendedor: Vendedor<'a>, medio_pago: TipoMedioPago) {
        self.registro_ventas.push(UnaVenta::new(fecha, productos, cliente, vendedor, medio_pago));
    }

    pub fn reporte_ventas_por_categoria(&self, categoria: Categoria) {
        self.registro_ventas
            .iter()
            .for_each(|v|
                if v.productos
                .iter()
                .any(|p| p.categoria == categoria
            ){
                println!("{:#?}",v);
            });
    }
}

#[derive(PartialEq, Eq, Debug)]
enum Categoria {
    ProductoAnimal,
    Lacteos,
    Vegetales,
    Herramientas,
    Botanica,
}

impl Categoria {
    fn get_descuento(&self) -> Option<i32> {
        
        match self {
            Categoria::ProductoAnimal => Some(5),
            Categoria::Lacteos => Some(10),
            Categoria::Vegetales => Some(15),
            _ => None,
        }
    }
}

#[derive(Debug)]
struct Producto<'a> {
    nombre: &'a str,
    categoria: Categoria,
    precio_base: f64,
}

#[derive(Debug)]
enum TipoMedioPago {
    TarjetaDeCredito,
    TarjetaDeDebito,
    TransferenciaBancaria,
    Efectivo,
}

#[derive(Debug)]
struct UnaVenta<'a> {
    productos: Vec<Producto<'a>>,
    fecha: Fecha,
    vendedor: Vendedor<'a>,
    cliente: Cliente<'a>,
    medio_pago: TipoMedioPago,
}

impl<'a> UnaVenta<'a> {
    pub fn new(fecha: Fecha, productos: Vec<Producto<'a>>, cliente: Cliente<'a>, vendedor: Vendedor<'a>, medio_pago:
    TipoMedioPago) -> UnaVenta<'a> {
        UnaVenta {
            productos,
            fecha,
            vendedor,
            cliente,
            medio_pago,
        }
    }

    pub fn calcular_precio_final(&self) -> f64 {
        let productos = self.productos.iter();
        let precio_final = productos
            .map(|p| match p.categoria.get_descuento() {
                None => p.precio_base,
                Some(des) => p.precio_base - (p.precio_base * des as f64) / 100.0,
            }).sum();
        if self.cliente.newsletter {
            precio_final - (precio_final * 2.0) / 100.0
        } else {
            precio_final
        }
    }
}

#[derive(Debug)]
struct DatosPersona<'a>{
    nombre: &'a str,
    apellido: &'a str,
    dni: u32,
}

#[derive(Debug)]
struct Vendedor<'a> {
    datos_persona: DatosPersona<'a>,
    legajo: u32,
    antiguedad: u8,
    salario: f64,
}

#[derive(Debug)]
struct Cliente<'a> {
    datos_persona: DatosPersona<'a>,
    newsletter: bool,
    correo: Option<&'a str>,
}



struct XYZ<'a> {
    usuarios: Vec<Usuario<'a>>,
    transacciones: Vec<Transaccion<'a>>,
    cryptos_operables: Vec<TipoCrypto>,
}

impl<'a> XYZ<'a> {
    pub fn ingresar_dinero(&mut self, dni: u32, monto: f64) {
        if let Some(u) = self.usuarios.iter_mut().find(|u| u.dni == dni) {
            u.dinero_fiat += monto;
        }
    }

    pub fn obtener_cotizacion(&self, crypto: &Crypto) -> Option<f64> {
        match crypto.prefijo {
            "BTC" => Some(1200.0),
            "ETH" => Some(450.0),
            "ETC" => Some(400.0),
            _ => None
        }
    }

    pub fn comprar_crypto(&mut self, usuario: &'a mut Usuario, crypto: &'a Crypto, monto: f64) {
        let cotizacion = self.obtener_cotizacion(crypto).expect("Crypto no encontrada");
        let comprables = (monto / cotizacion).floor();
        let monto = cotizacion * comprables;
        if usuario.dinero_fiat >= monto { 
            usuario.dinero_fiat -= monto;
            let mut binding = 0;
            let mut iter = usuario.balance.iter_mut().find(|(c,_)| c.prefijo == crypto.prefijo);
            let a = iter.get_or_insert((crypto, &mut binding));
            *a.1 += comprables as i64;

            self.transacciones.push(Transaccion {
                fecha: Fecha {
                    dia: Local::now().day(),
                    mes: Local::now().month(),
                    año: Local::now().year(), 
                },
                usuario,
                criptomoneda: Some(crypto),
                tipo: TipoTransaccion::CompraCrypto,
                monto: comprables,
                cotizacion: Some(cotizacion),
            });
        }
    }

    pub fn vender_crypto(&mut self, usuario: &'a mut Usuario, crypto: &'a Crypto, monto: i64) {
        let cotizacion = self.obtener_cotizacion(crypto).expect("Crypto no encontrada");
        let a_recibir = monto as f64 * cotizacion;
        if let Some(balance) = usuario.balance.iter_mut().find(|b| b.0.prefijo == crypto.prefijo ) {
            if *balance.1 >= monto { 
                usuario.dinero_fiat += a_recibir;
                *balance.1 -= monto;

                self.transacciones.push(Transaccion {
                    fecha: Fecha {
                        dia: Local::now().day(),
                        mes: Local::now().month(),
                        año: Local::now().year(), 
                    },
                    usuario,
                    criptomoneda: Some(crypto),
                    tipo: TipoTransaccion::VentaCrypto,
                    monto: monto as f64,
                    cotizacion: Some(cotizacion),
                });
            }
        }
    }

    pub fn retirar_a_blockchain(&mut self, usuario: &'a mut Usuario, crypto: &'a Crypto, monto: i64, blockchain: &'a Blockchain) {
        let cotizacion = self.obtener_cotizacion(crypto).expect("Crypto no encontrada");
        if let Some(balance) = usuario.balance.iter_mut().find(|b| b.0.prefijo == crypto.prefijo) {
            if *balance.1 >= monto && balance.0.blockchains.iter().any(|b| b.prefijo == blockchain.prefijo) {
                *balance.1 -= monto;
                let hash: String = String::from(blockchain.prefijo);
                let hash = hash + &rand::random::<i64>().to_string();
                
                self.transacciones.push(Transaccion{
                    fecha: Fecha {
                        dia: Local::now().day(),
                        mes: Local::now().month(),
                        año: Local::now().year(), 
                    },
                    usuario,
                    criptomoneda: Some(crypto),
                    tipo: TipoTransaccion::RetiroCrypto(CambioBlockchain{
                        blockchain,
                        hash,
                    }),
                    monto: monto as f64,
                    cotizacion: Some(cotizacion)
                })                
            }
        }
    }

    pub fn recibir_de_blockchain(&mut self, usuario: &'a mut Usuario, crypto: &'a Crypto, monto: i64, blockchain: &'a Blockchain) -> Option<String> {
        let cotizacion = self.obtener_cotizacion(crypto).expect("Crypto no encontrada");
        if let Some(balance) = usuario.balance.iter_mut().find(|b| b.0.prefijo == crypto.prefijo) {
            if balance.0.blockchains.iter().any(|b| b.prefijo == blockchain.prefijo) {
                *balance.1 += monto;
                let hash: String = String::from(blockchain.prefijo);
                let hash = hash + &rand::random::<i64>().to_string();
                
                self.transacciones.push(Transaccion{
                    fecha: Fecha {
                        dia: Local::now().day(),
                        mes: Local::now().month(),
                        año: Local::now().year(), 
                    },
                    usuario,
                    criptomoneda: Some(crypto),
                    tipo: TipoTransaccion::RecepcionCrypto(CambioBlockchain{
                        blockchain,
                        hash: hash.clone(),
                    }),
                    monto: monto as f64,
                    cotizacion: Some(cotizacion)
                });
                return Some(hash);              
            }
        }
        None
    }

    pub fn retirar_fiat(&mut self, usuario: &'a mut Usuario, monto: f64, medio: MedioRetiro) {
        if usuario.dinero_fiat >= monto {
            self.transacciones.push(Transaccion {
                fecha: Fecha {
                    dia: Local::now().day(),
                    mes: Local::now().month(),
                    año: Local::now().year(), 
                },
                usuario,
                criptomoneda: None,
                tipo: TipoTransaccion::RetiroFiat(medio),
                monto,
                cotizacion: None
            });
        } 
    }



    pub fn crypto_mas_vendida(&self) -> Option<TipoCrypto> {
        let criptos = TipoCrypto::get_cryptos();
        let iter = self.transacciones.iter().filter(|t| t.tipo == TipoTransaccion::CompraCrypto);

        let mut ts_max: Option<TipoCrypto> = None;
        let mut max = 0;
        for cripto in criptos {
            let cont = iter.clone().filter(|t| t.criptomoneda.is_some() && t.criptomoneda.unwrap().tipo.this_ct(&cripto)).count();
            if cont > max {
                max = cont;
                ts_max = Some(cripto);
            }
        }
        ts_max
    }

    pub fn crypto_mas_comprada(&self) -> Option<TipoCrypto> {
        let criptos = TipoCrypto::get_cryptos();
        let iter = self.transacciones.iter().filter(|t| t.tipo == TipoTransaccion::VentaCrypto);

        let mut ts_max: Option<TipoCrypto> = None;
        let mut max = 0;
        for cripto in criptos {
            let cont = iter.clone().filter(|t| t.criptomoneda.is_some() && t.criptomoneda.unwrap().tipo.this_ct(&cripto)).count();
            if cont > max {
                max = cont;
                ts_max = Some(cripto);
            }
        }
        ts_max
    }

    pub fn crypto_mas_volumen_vendida(&self) -> Option<TipoCrypto> {
        let criptos = TipoCrypto::get_cryptos();
        let iter = self.transacciones.iter().filter(|t| t.tipo == TipoTransaccion::CompraCrypto);

        let mut ts_max: Option<TipoCrypto> = None;
        let mut max = 0;
        for cripto in criptos {
            let cont = iter.clone().filter_map(|t| match t.criptomoneda.is_some() && t.criptomoneda.unwrap().tipo.this_ct(&cripto){
                true => Some(t.monto as i64),
                false => None,
            }).sum();
            if cont > max {
                max = cont;
                ts_max = Some(cripto);
            }
        }
        ts_max
    }

    pub fn crypto_mas_volumen_comprada(&self) -> Option<TipoCrypto> {
        let criptos = TipoCrypto::get_cryptos();
        let iter = self.transacciones.iter().filter(|t| t.tipo == TipoTransaccion::VentaCrypto);

        let mut ts_max: Option<TipoCrypto> = None;
        let mut max = 0;
        for cripto in criptos {
            let cont = iter.clone().filter_map(|t| match t.criptomoneda.is_some() && t.criptomoneda.unwrap().tipo.this_ct(&cripto){
                true => Some(t.monto as i64),
                false => None,
            }).sum();
            if cont > max {
                max = cont;
                ts_max = Some(cripto);
            }
        }
        ts_max
    }
}

struct Transaccion<'a> {
    fecha: Fecha,
    usuario: &'a Usuario<'a>,
    criptomoneda: Option<&'a Crypto<'a>>,
    tipo: TipoTransaccion<'a>,
    monto: f64,
    cotizacion: Option<f64>,
}

#[derive(PartialEq, Eq)]
enum TipoTransaccion<'a> {
    CompraCrypto,
    VentaCrypto,
    RetiroCrypto(CambioBlockchain<'a>),
    RecepcionCrypto(CambioBlockchain<'a>),
    RetiroFiat(MedioRetiro),
}

#[derive(PartialEq, Eq)]
enum MedioRetiro {
    MercadoPago,
    TransferenciaBancaria,
}

#[derive(PartialEq, Eq)]
struct CambioBlockchain <'a> {
    blockchain: &'a Blockchain<'a>,
    hash: String,
}

struct Usuario<'a> {
    nombre: &'a str,
    apellido: &'a str,
    email: &'a str,
    dni: u32,
    validado: bool,
    dinero_fiat: f64,
    balance: HashMap<Crypto<'a>, i64>,
}

impl<'a> Usuario<'a> {
    pub fn new() -> Usuario<'a>{
        todo!("Inicializar hashmaps con las cryptos en 0");
    }
}

#[derive(PartialEq, Eq)]
enum TipoCrypto {
    ETC,
    ETH,
    BTC,
}

impl TipoCrypto {
    pub fn get_cryptos() ->  Vec<TipoCrypto> {
        let vec = vec![
            TipoCrypto::ETC,
            TipoCrypto::ETH,
            TipoCrypto::BTC,
        ];
        vec
    }

    pub fn this_ct(&self, other: &TipoCrypto) -> bool {
        self == other
    }
}

struct Crypto<'a> {
    nombre: &'a str,
    prefijo: &'a str,
    tipo: TipoCrypto,
    blockchains: Vec<Blockchain<'a>>,
}

#[derive(PartialEq, Eq)]
struct Blockchain<'a> {
    nombre: &'a str,
    prefijo: &'a str,
}