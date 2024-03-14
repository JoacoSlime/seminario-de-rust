use std::collections::{HashMap, VecDeque};

#[allow(unused)]
pub struct Persona {
    nombre: String,
    edad: i32,
    direccion: Option<String>
}

impl Persona {
    pub fn new(nombre: String, edad: i32, direccion: Option<String>) -> Persona {
        Persona { 
            nombre,
            edad,
            direccion
        }
    }
    pub fn imprimir(&self) {
        if let Some(dir) = &self.direccion {
            println!("Soy {}, tengo {} años y vivo en {}.",self.nombre, self.obtener_edad(), dir);
        } else {
            println!("Soy {} y tengo {} años.",self.nombre, self.obtener_edad());
        }
    }
    pub fn obtener_edad(&self) -> i32 {
        self.edad
    }
    pub fn actualizar_direccion(&mut self, nueva_direccion: String) {
        self.direccion = Some(nueva_direccion);
    }
}

pub fn ejercicio1(){
    let mut p = Persona::new("Augusto".to_string(), 25, Some("45 1418".to_string()));
    p.imprimir();
    p.actualizar_direccion("75 1546".to_string());
    p.imprimir();
}

#[derive(Debug)]
struct Rectangulo {
    longitud: f64,
    ancho: f64
}
impl Rectangulo {
    pub fn new(longitud: f64, ancho: f64) -> Rectangulo{
        Rectangulo {
            longitud,
            ancho
        }
    }
    pub fn calcular_area(&self) -> f64 {
        self.longitud * self.ancho
    }
    pub fn calcular_perimetro(&self) -> f64 {
        self.longitud * 2.0 + self.ancho * 2.0
    }
    pub fn es_cuadrado(&self) -> bool {
        self.longitud == self.ancho
    }
}

pub fn ejercicio2() {
    let r = Rectangulo::new(2.0,4.5);
    println!("{:#?}", r);
    println!("Perimetro: {}", r.calcular_perimetro());
    println!("Area: {}", r.calcular_area());
    println!("Es cuadrado: {}", r.es_cuadrado());
    let r = Rectangulo::new(5.0,5.0);
    println!("{:#?}", r);
    println!("Es cuadrado: {}", r.es_cuadrado());
}

#[derive(Debug)]
pub struct Fecha {
    dia: u32,
    mes: u32,
    año: u32,
}

impl Fecha {
    pub fn new(dia: u32,mes: u32,año: u32) -> Fecha {
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

pub fn ejercicio3() {
    let mut f1 = Fecha::new(12,08,2002);
    let mut f2 = Fecha::new(30,2,2024);
    println!("Fecha 1: {} de {} del {}", f1.dia, f1.mes, f1.año);
    println!("Es la fecha válida?: {}", f1.es_fecha_valida());
    println!("Es un año biciesto?: {}", f1.es_biciesto());
    f1.sumar_dias(30);
    println!("Sumando 30 días es: {} de {} de {}", f1.dia, f1.mes, f1.año);
    f1.restar_dias(60);
    println!("Restando 60 días es: {} de {} de {}", f1.dia, f1.mes, f1.año);
    println!("Fecha 2: {} de {} del {}", f2.dia, f2.mes, f2.año);
    println!("Es la fecha válida?: {}", f2.es_fecha_valida());
    f2.restar_dias(1);
    println!("Restando un número, es la fecha válida?: {}", f2.es_fecha_valida());
    println!("Es un año biciesto?: {}", f2.es_biciesto());
    println!("Es la Fecha 2 mayor a la 1?: {}", f2.es_mayor(&f1));
}

pub struct Triangulo {
    lado1: f64,
    lado2: f64,
    lado3: f64,
}

#[derive(Debug)]
pub enum TipoTriangulo {
    Equilatero,
    Isoceles,
    Escaleno,
}

impl Triangulo {
    pub fn new(lado1: f64, lado2: f64, lado3: f64) -> Triangulo {
        Triangulo{
            lado1,
            lado2,
            lado3,
        }
    }

    pub fn determinar_tipo (&self) -> TipoTriangulo {
        if self.lado1 == self.lado2 && self.lado2 == self.lado3 {
            TipoTriangulo::Equilatero
        } else if self.lado1 == self.lado2 || self.lado2 == self.lado3 || self.lado3 == self.lado1 {
            TipoTriangulo::Isoceles
        } else {
            TipoTriangulo::Escaleno
        }
    }

    pub fn calcular_area(&self) -> f64 {
        let perimetro = self.calcular_perimetro();
        (perimetro*(perimetro-self.lado1)*(perimetro-self.lado2)*(perimetro-self.lado3)).sqrt()
    }

    pub fn calcular_perimetro(&self) -> f64 {
        self.lado1 + self.lado2 + self.lado3
    }
}

pub fn ejercicio4() {
    let t1 = Triangulo::new(1.0,2.0,3.0);
    let t2 = Triangulo::new(2.0,2.0,3.0);
    let t3 = Triangulo::new(3.0,3.0,3.0);
    println!("El triangulo 1 es de tipo {:?}, tiene perimetro {} y area {}", t1.determinar_tipo(), t1.calcular_perimetro(), t1.calcular_area());
    println!("El triangulo 1 es de tipo {:?}, tiene perimetro {} y area {}", t2.determinar_tipo(), t2.calcular_perimetro(), t2.calcular_area());
    println!("El triangulo 1 es de tipo {:?}, tiene perimetro {} y area {}", t3.determinar_tipo(), t3.calcular_perimetro(), t3.calcular_area());
    println!();
}

#[derive(Debug)]
struct Producto {
    nombre: String,
    precio_bruto: f64,
    numero_id: i32,
}

impl Producto {
    pub fn new(nombre: String, precio_bruto: f64, numero_id: i32) -> Producto {
        Producto{
            nombre,
            precio_bruto,
            numero_id,
        }
    }
    pub fn calcular_impuestos(&self, impuestos: Option<i32>) -> f64{
        if let Some(imp) = impuestos {
            self.precio_bruto * (imp as f64 / 100.0)
        } else {
            0.0
        }
    }

    pub fn aplicar_descuento(&self, descuentos: Option<i32>) -> f64 {
        if let Some(des) = descuentos {
            (self.precio_bruto * des as f64) / 100.0
        } else {
            0.0
        }
    }

    pub fn calcular_precio_total(&self, porcentaje_impuestos: Option<i32>, porcentaje_descuentos: Option<i32>) -> f64 {
        self.precio_bruto + self.calcular_impuestos(porcentaje_impuestos)
                          - self.aplicar_descuento(porcentaje_descuentos)
    }
}

pub fn ejercicio5() {
    let p = Producto::new("Kinder".to_string(), 1700.00, 32);
    println!("{:?}", p);
    println!("Con impuestos 11% y descuento 5%, los impuestos son {} y el descuento es de {}", p.calcular_impuestos(Some(11)), p.aplicar_descuento(Some(5)));
    println!("Aplicados impuestos y descuentos, el precio final es: {}",p.calcular_precio_total(Some(11), Some(5)));
    println!();
}

#[derive(Debug)]
struct Examen {
    materia: String,
    nota: f64,
}

#[derive(Debug)]
struct Estudiante {
    nombre: String,
    numero_id: i32,
    calificacion: Option<Vec<Examen>>,
}

impl Examen {
    pub fn new(materia: String, nota: f64) -> Examen {
        Examen {
            materia,
            nota,
        }
    }
}

impl Estudiante {
    pub fn new(nombre: String, numero_id: i32, calificacion: Option<Vec<Examen>>) -> Estudiante {
        Estudiante{
            nombre,
            numero_id,
            calificacion,
        }
    }

    pub fn obtener_promedio(&self) -> f64{
        let mut promedio:f64 = 0.0;
        if let Some(v) = &self.calificacion {
            for e in v {
                promedio += e.nota;
            }
            promedio /= v.len() as f64;
        }
        promedio
    }

    pub fn obtener_calificacion_mas_alta(&self) -> f64 {
        let mut max: f64 = f64::NEG_INFINITY;
        if let Some(v) = &self.calificacion {
            for e in v {
                max = max.max(e.nota);
            }
        }
        max
    }
    
    pub fn obtener_calificacion_mas_baja(&self) -> f64 {
        let mut min: f64 = f64::INFINITY;
        if let Some(v) = &self.calificacion {
            for e in v {
                min = min.min(e.nota);
            }
        }
        min
    }
}

pub fn ejercicio6() {
    let examenes = vec![
        Examen::new("Mate 1".to_string(), 8.66),
        Examen::new("Mate 2".to_string(), 9.66),
        Examen::new("CADP".to_string(), 4.0),
        Examen::new("Taller".to_string(), 8.33),
        Examen::new("Inglés".to_string(), 10.0)
    ];
    let e = Estudiante::new("Juan Alberto".to_string(), 19254, Some(examenes));
    println!("{:#?}", e);
    println!("Promedio: {}", e.obtener_promedio());
    println!("Nota más alta: {}, nota más baja: {}", e.obtener_calificacion_mas_alta(), e.obtener_calificacion_mas_baja());
    let e = Estudiante::new("Tomás Pepe".to_string(), 21354, None);
    println!("{:#?}", e);
    println!("Promedio: {}", e.obtener_promedio());
    println!("Nota más alta: {}, nota más baja: {}", e.obtener_calificacion_mas_alta(), e.obtener_calificacion_mas_baja());
    println!();
}

#[derive(Debug)]
struct ConcesionarioAuto {
    nombre: String,
    direccion: String,
    capacidad_max: i32,
    autos: Vec<Auto>,
}

#[derive(Debug)]
struct Auto {
    marca: String,
    modelo: String,
    año: i32,
    precio_bruto: f64,
    color: Color,
}

#[derive(Debug)]
enum Color {
    Rojo,
    Verde,
    Azul,
    Amarillo,
    Blanco,
    Negro,
}

impl Color {
    fn color_match(&self) -> i32 {
        match self {
            &Self::Rojo => 0,
            &Self::Verde => 1,
            &Self::Azul => 2,
            &Self::Amarillo => 3,
            &Self::Blanco => 4,
            &Self::Negro => 5,
        }
    }

    pub fn equals (&self, other: &Color) -> bool{
        self.color_match() == other.color_match()
    }
}

impl ConcesionarioAuto {
    pub fn new(nombre: String, direccion: String, capacidad_max: i32) -> ConcesionarioAuto {
        ConcesionarioAuto {
            nombre,
            direccion,
            capacidad_max,
            autos: Vec::with_capacity(capacidad_max as usize),
        }
    }

    pub fn agregar_auto(&mut self, auto: Auto) -> bool{
        if self.autos.len() < self.capacidad_max as usize {
            self.autos.push(auto);
            return true
        }
        false
    }

    pub fn eliminar_auto(&mut self, auto: &Auto){
        for i in 0..self.autos.len()-1 {
            if auto.equals(&self.autos[i]) {
                self.autos.remove(i);
            }
        }
    }

    pub fn buscar_auto(&mut self, auto: &Auto) -> Option<&Auto>{
        let mut ret: Option<&Auto> = None;
        for i in 0..self.autos.len() {
            if auto.equals(&self.autos[i]) {
                ret = Some(&self.autos[i]);
            }
        }
        ret
    }
}

impl Auto {
    pub fn new(marca:String, modelo:String, año:i32, precio_bruto:f64, color:Color) -> Auto {
        Auto{
            marca,
            modelo,
            año,
            precio_bruto,
            color,
        }
    }

    pub fn equals(&self, other: &Auto) -> bool {
        other.marca == self.marca && other.año == self.año && other.modelo == self.modelo &&
        other.color.equals(&self.color) && other.precio_bruto == self.precio_bruto
    }

    pub fn calcular_precio(&self) -> f64 {
        let mut recargo = match self.color {
            Color::Rojo => self.precio_bruto * 0.25,
            Color::Azul => self.precio_bruto * 0.25,
            Color::Amarillo => self.precio_bruto * 0.25,
            _ => self.precio_bruto * -0.10,
        };
        recargo = if self.marca == "BMW" {
            recargo - self.precio_bruto * 0.15
        } else {
            recargo
        };
        recargo = if self.año < 2000 {
            recargo - self.precio_bruto * 0.05
        } else {
            recargo
        };
        self.precio_bruto + recargo
    }
}

pub fn ejercicio7 () {
    let a1 = Auto::new("BMW".to_string(), "Carcacha".to_string(),
        2003, 75_000.0, Color::Verde);
    let a2 = Auto::new("Chevrolet".to_string(), "Socotroco".to_string(),
        2014, 175_000.0, Color::Azul);
    let a3 = Auto::new("Renault".to_string(), "15 2.0".to_string(),
        2020, 275_000.0, Color::Negro);
    let a4 = Auto::new("Chevi".to_string(), "Argento".to_string(),
        1992, 40_000.0, Color::Verde);
    let mut concecionario = ConcesionarioAuto::new("Lamina Motors".to_string(),
        "Olavarria 1254".to_string(), 3);
    concecionario.agregar_auto(a1);
    concecionario.agregar_auto(a2);
    concecionario.agregar_auto(a3);
    concecionario.agregar_auto(a4);
    println!("{:#?}", concecionario);


    let a1 = Auto::new("BMW".to_string(), "Carcacha".to_string(),
        2003, 75_000.0, Color::Verde);
    let a2 = Auto::new("Chevrolet".to_string(), "Socotroco".to_string(),
        2014, 175_000.0, Color::Azul);
    let a3 = Auto::new("Renault".to_string(), "15 2.0".to_string(),
        2020, 275_000.0, Color::Negro);
    let a4 = Auto::new("Chevi".to_string(), "Argento".to_string(),
        1992, 40_000.0, Color::Blanco);
    match concecionario.buscar_auto(&a4) {
        Some(_) => {println!("Existe el auto")},
        None => {println!("No existe el auto")},
    }
    println!("Precios:");
    match concecionario.buscar_auto(&a1) {
        Some(auto) => {println!("{:?} tiene precio total: {}", auto, auto.calcular_precio())},
        None => {println!("No existe el auto")},
    }
    match concecionario.buscar_auto(&a2) {
        Some(auto) => {println!("{:?} tiene precio total: {}", auto, auto.calcular_precio())},
        None => {println!("No existe el auto")},
    }
    match concecionario.buscar_auto(&a3) { // TREMENDO BUG ACÁ.
        Some(auto) => {println!("{:?} tiene precio total: {}", auto, auto.calcular_precio())},
        None => {println!("No existe el auto")},
    }
    concecionario.eliminar_auto(&a2);
    concecionario.agregar_auto(a4);
    println!("{:#?}", concecionario);
    println!();
}

#[derive(Debug)]
enum Genero{
    Rock,
    Pop,
    Rap,
    Jazz,
    Otros,
}

#[derive(Debug)]
struct Cancion {
    titulo: String,
    artista: String,
    genero: Genero,
}


#[derive(Debug)]
struct Playlist {
    canciones: VecDeque<Cancion>,
    nombre: String,
}


impl Genero {
    fn genre_match(&self) -> u32 {
        match self {
            Self::Rock => 0,
            Self::Pop => 1,
            Self::Rap => 2,
            Self::Jazz => 3,
            Self::Otros => 4,
        }
    }

    pub fn equals(&self, other: &Genero) -> bool {
        self.genre_match() == other.genre_match()
    }
}

impl Cancion {
    pub fn equals(&self, other: &Cancion) -> bool {
        self.titulo == other.titulo && self.artista == other.artista
        && self.genero.equals(&other.genero)
    }
}

impl Playlist {
    pub fn agregar_cancion(&mut self, cancion: Cancion) {
        self.canciones.push_back(cancion)
    }

    fn get_pos_cancion(&mut self, cancion: &Cancion) -> Option<usize> {
        (0..self.canciones.len()).find(|&i| self.canciones[i].equals(cancion))
    }

    pub fn eliminar_cancion(&mut self, cancion: &Cancion) {
        if let Some(pos) = self.get_pos_cancion(cancion){
            self.canciones.remove(pos);
        }
    }

    pub fn mover_cancion(&mut self, cancion: &Cancion, pos: usize) {
        if let Some(i) = self.get_pos_cancion(cancion) {
            let cancion = self.canciones.remove(i).unwrap();
            self.canciones.insert(pos, cancion);
        }
    }

    pub fn buscar_cancion(&self, nombre: String) -> Option<&Cancion> {
        for i in 0..self.canciones.len() {
            if self.canciones[i].titulo == nombre{
                return Some(&self.canciones[i]);
            }
        }
        None
    }

    pub fn get_from_genero(&self, genero: &Genero) -> Vec<&Cancion> {
        let mut vec = Vec::new();
        for i in 0..self.canciones.len() {
            if self.canciones[i].genero.equals(genero){
                vec.push(&self.canciones[i]);
            }
        }
        vec
    }

    pub fn get_from_artista(&self, nombre: String) -> Vec<&Cancion> {
        let mut vec = Vec::new();
        for i in 0..self.canciones.len() {
            if self.canciones[i].artista == nombre {
                vec.push(&self.canciones[i])
            }
        }
        vec
    }

    pub fn set_titulo(&mut self, titulo: String) {
        self.nombre = titulo;
    }

    pub fn limpiar_lista(&mut self) {
        self.canciones.clear()
    }
}

pub fn ejercicio8() {
    let canciones = VecDeque::from(
        [
            Cancion{
                titulo: "Oyasumi".to_string(),
                artista: "Mr. Shiggy".to_string(),
                genero: Genero::Pop
            },
            Cancion{
                titulo: "Lagtrain".to_string(),
                artista: "Inaba Cumori".to_string(),
                genero: Genero::Otros
            },
            Cancion{
                titulo: "No sopor".to_string(),
                artista: "Joaquín Sabina".to_string(),
                genero: Genero::Rap
            },
            Cancion{
                titulo: "Beneath The Mask".to_string(),
                artista: "Lyn".to_string(),
                genero: Genero::Jazz
            },
            Cancion{
                titulo: "Somos".to_string(),
                artista: "Mata Tiempo".to_string(),
                genero: Genero::Rock
            },
        ]
    );
    let mut playlist = Playlist {
        canciones,
        nombre: "Liked".to_string(),
    };
    println!("{:#?}", playlist);
    let cancion = Cancion{
        titulo: "nevermore".to_string(),
        artista: "sasalasa".to_string(),
        genero: Genero::Otros,
    };
    playlist.agregar_cancion(cancion);
    let cancion = Cancion{
        titulo: "Lagtrain".to_string(),
        artista: "Inaba Cumori".to_string(),
        genero: Genero::Otros
    };
    playlist.eliminar_cancion(&cancion);
    let cancion = Cancion{
        titulo: "nevermore".to_string(),
        artista: "sasalasa".to_string(),
        genero: Genero::Otros,
    };
    playlist.mover_cancion(&cancion, 0);
    println!("{:#?}", playlist);
    if let Some(cancion) = playlist.buscar_cancion("No sopor".to_string()){
        println!("Se encontró {:?}", cancion);
    }
    let canciones = playlist.get_from_genero(&Genero::Pop);
    if !canciones.is_empty() {
        println!("Se encontraron las siguientes canciones pop:");
        for c in canciones {
            println!("{:?}", c)
        }
    }
    let canciones = playlist.get_from_artista("Lyn".to_string());
    if !canciones.is_empty() {
        println!("Se encontraron las siguientes canciones:");
        for c in canciones {
            println!("{:?}", c)
        }
    }
    playlist.set_titulo("Unnamed".to_string());
    playlist.limpiar_lista();
    println!("{:#?}", playlist);
}

#[derive(Debug)]
struct Veterinaria{
    nombre: String,
    direccion: String,
    id: u32,
    cola_atencion: VecDeque<Mascota>,
    registro_atencion: Vec<Atencion>,
}

#[derive(Debug)]
enum Animal {
    Perro,
    Gato,
    Caballo,
    Otros,
}

#[derive(Debug)]
struct Dueño {
    nombre: String,
    direccion: String,
    telefono: String,
}

#[derive(Debug)]
struct Mascota {
    nombre: String,
    edad: u32,
    animal: Animal,
    dueño: Dueño,
}

#[derive(Debug)]
struct Atencion{
    mascota: Mascota,
    diagnostico: String,
    tratamiento: String,
    proxima_visita: Option<Fecha>,
}

impl Animal {
    fn match_number(&self) -> u32{
        match *self {
            Self::Perro => 0,
            Self::Gato => 1,
            Self::Caballo => 2,
            Self::Otros => 3,
        }
    }

    pub fn equals(&self, other: &Self) -> bool {
        self.match_number() == other.match_number()
    }
}

impl Dueño {
    pub fn equals(&self, other: &Self) -> bool{
        self.nombre == other.nombre && self.telefono == other.telefono && self.direccion == other.direccion
    }
}

impl Mascota {
    pub fn equals(&self, other: &Self) -> bool{
        self.nombre == other.nombre && self.edad == other.edad && self.dueño.equals(&other.dueño) && self.animal.equals(&other.animal)
    }
}

impl Atencion {
    pub fn equals(&self, other: &Self) -> bool{
        let misma_fecha = if self.proxima_visita.is_none() == other.proxima_visita.is_none() {
            true
        } else if (self.proxima_visita.is_none() && other.proxima_visita.is_some()) || (self.proxima_visita.is_some() && other.proxima_visita.is_none()) {
            false
        } else {
            self.proxima_visita.as_ref().unwrap().año == other.proxima_visita.as_ref().unwrap().año && self.proxima_visita.as_ref().unwrap().mes == other.proxima_visita.as_ref().unwrap().mes && self.proxima_visita.as_ref().unwrap().dia == other.proxima_visita.as_ref().unwrap().dia
        };
        misma_fecha && self.diagnostico == other.diagnostico && self.mascota.equals(&other.mascota) && self.tratamiento == other.tratamiento
    }
}

impl Veterinaria {
    pub fn new(nombre: String, direccion: String, id: u32) -> Veterinaria {
        Veterinaria{
            nombre,
            direccion,
            id,
            cola_atencion: VecDeque::new(),
            registro_atencion: Vec::new(),
        }
    }
    pub fn agregar_mascota(&mut self, mascota: Mascota) {
        self.cola_atencion.push_back(mascota);
    }

    pub fn agregar_mascota_alta_prioridad(&mut self, mascota: Mascota) {
        self.cola_atencion.push_front(mascota);
    }

    pub fn atender_mascota(&mut self) -> Option<Mascota> {
        self.cola_atencion.pop_front()
    }

    pub fn eliminar_mascota(&mut self, mascota: Mascota) {
        for i in 0..self.cola_atencion.len() {
            if self.cola_atencion[i].equals(&mascota) {
                self.cola_atencion.remove(i);
                break
            }
        }
    }

    pub fn registrar_atencion(&mut self, atencion: Atencion) {
        self.registro_atencion.push(atencion)
    } 

    pub fn buscar_atencion(&mut self, nombre_mascota: String, nombre_dueño: String, telefono: String) -> Option<&Atencion> {
        for i in 0..self.registro_atencion.len() {
            if self.registro_atencion[i].mascota.nombre == nombre_mascota && self.registro_atencion[i].mascota.dueño.nombre == nombre_dueño && self.registro_atencion[i].mascota.dueño.telefono == telefono {
                return Some(&self.registro_atencion[i]);
            }
        }
        None
    }

    pub fn modificar_diagnostico(&mut self, atencion: &Atencion, nuevo_diagnostico: String) {
        for a in &mut self.registro_atencion {
            if a.equals(atencion){
                a.diagnostico = nuevo_diagnostico;
                return;
            }
        }
    }

    pub fn modificar_fecha(&mut self, atencion: &Atencion, nueva_fecha: Fecha) {
        for i in 0..self.registro_atencion.len() {
            if self.registro_atencion[i].equals(atencion){
                self.registro_atencion[i].proxima_visita = Some(nueva_fecha);
                return;
            }
        }
    }

    pub fn eliminar_fecha(&mut self, atencion: &Atencion) {
        for i in 0..self.registro_atencion.len() {
            if self.registro_atencion[i].equals(atencion){
                self.registro_atencion[i].proxima_visita = None;
                return;
            }
        }
    }

    pub fn eliminar_atencion(&mut self, atencion: &Atencion){
        for i in 0..self.registro_atencion.len() {
            if self.registro_atencion[i].equals(atencion) {
                self.registro_atencion.remove(i);
            }
        }
    }
}

pub fn ejercicio9() {
    let mut vet = Veterinaria::new("Patitas".to_string(), "72 1022".to_string(), 0);
    println!("{:?}", vet);
    let d = Dueño {
        nombre: "Joaco".to_string(),
        direccion: "Calle Falsa 123".to_string(),
        telefono: "+542211234567".to_string()
    };
    let m1 = Mascota{
        nombre: "Shiro".to_string(),
        edad: 4,
        animal: Animal::Gato,
        dueño: d,
    };
    vet.agregar_mascota(m1);
    let d = Dueño {
        nombre: "Joaco".to_string(),
        direccion: "Calle Falsa 123".to_string(),
        telefono: "+542211234567".to_string()
    };
    let m2 = Mascota{
        nombre: "Morena".to_string(),
        edad: 3,
        animal: Animal::Perro,
        dueño: d,
    };
    let d = Dueño {
        nombre: "Joaco".to_string(),
        direccion: "Calle Falsa 123".to_string(),
        telefono: "+542211234567".to_string()
    };
    let m3 = Mascota{
        nombre: "Adolfa".to_string(),
        edad: 3,
        animal: Animal::Gato,
        dueño: d,
    };
    vet.agregar_mascota_alta_prioridad(m2);
    vet.agregar_mascota(m3);
    println!("{:?}", vet);
    let d = Dueño {
        nombre: "Joaco".to_string(),
        direccion: "Calle Falsa 123".to_string(),
        telefono: "+542211234567".to_string()
    };
    let m3 = Mascota{
        nombre: "Adolfa".to_string(),
        edad: 3,
        animal: Animal::Gato,
        dueño: d,
    };
    let m = vet.atender_mascota();
    vet.eliminar_mascota(m3);
    println!("{:?}", &vet);
    
    let a = Atencion{
        mascota: m.unwrap(),
        diagnostico: "No soy médico".to_string(),
        tratamiento: "YO que sé".to_string(),
        proxima_visita: None,
    };
    vet.registrar_atencion(a);
    let a = vet.buscar_atencion("Morena".to_string(), "Joaco".to_string(), "+542211234567".to_string()).unwrap();
    println!("{:?}", a);
    println!();
    let a = &Atencion {
        mascota: Mascota{
            nombre: "Morena".to_string(),
            edad: 3,
            animal: Animal::Perro,
            dueño: Dueño {
                nombre: "Joaco".to_string(),
                direccion: "Calle Falsa 123".to_string(),
                telefono: "+542211234567".to_string()
            },
        },
        diagnostico: "No soy médico".to_string(),
        tratamiento: "YO que sé".to_string(),
        proxima_visita: None,
    };

    vet.modificar_diagnostico(a, "Quiste en la pierna trasera derecha".to_string());
    vet.modificar_fecha(a, Fecha { dia: 9, mes: 6, año: 2023 });
    println!("{:?}", a);

    vet.eliminar_fecha(a);
    vet.eliminar_atencion(a);
    println!("{:?}", a);
    
}

struct Biblioteca{
    nombre: String,
    direccion: String,
    libros_a_disposicion: Vec<LibroADisposicion>,
    prestamos_efectuados: Vec<Prestamo>,
}

struct Libro{
    titulo: String,
    autor: String,
    numero_pag: u32,
    genero: GeneroLibro,
}

struct LibroADisposicion{
    libro: Libro,
    cant: u32,
}

enum GeneroLibro{
    Novela,
    Infantil,
    Tecnico,
    Otro
}

struct Prestamo{
    libro: Libro,
    cliente: Cliente,
    vencimiento: Fecha,
    devolucion: Option<Fecha>,
    estado: Estado,
}

enum Estado {
    Devuelto,
    EnPrestamo,
}

struct Cliente {
    nombre: String,
    telefono: String,
    email: String,
}

impl GeneroLibro{
    fn match_number(&self) -> u32 {
        match self {
            Self::Infantil => 0,
            Self::Novela => 1,
            Self::Tecnico => 2,
            Self::Otro => 3,
        }
    }

    pub fn equals(&self, other: &Self) -> bool {
        self.match_number() == other.match_number()
    }
}

impl Libro {
    pub fn equals(&self, other: &Self) -> bool {
        self.autor == other.autor && self.titulo == other.titulo && self.numero_pag == other.numero_pag && self.genero.equals(&other.genero)
    }
}

impl Cliente {
    pub fn equals(&self, other: &Self) -> bool {
        self.nombre == other.nombre && self.telefono == other.telefono && self.email == other.email
    }
}

impl Biblioteca {    
    pub fn obtener_copias(&self, libro: &Libro) -> u32{
        for l in &self.libros_a_disposicion {
            if l.libro.equals(libro) {
                return l.cant
            }
        }
        0
    }

    pub fn decrementar_copias(&mut self, libro: &Libro) {
        for i in 0..self.libros_a_disposicion.len() {
            if self.libros_a_disposicion[i].libro.equals(libro) {
                self.libros_a_disposicion[i].cant -= 1;
            }
        }
    }

    pub fn incrementar_copias(&mut self, libro: &Libro) {
        for i in 0..self.libros_a_disposicion.len() {
            if self.libros_a_disposicion[i].libro.equals(libro) {
                self.libros_a_disposicion[i].cant += 1;
            }
        }
    }

    pub fn contar_prestamos(&self, cliente: Cliente) -> u32{
        let mut cont = 0;
        for p in &self.prestamos_efectuados{
            if p.cliente.equals(&cliente) {
                cont += 1;
            }
        }
        cont
    }

    pub fn realizar_prestamo(&mut self, libro: Libro, cliente: Cliente) -> bool{
        let prestamos = self.contar_prestamos(cliente);
        let disponibilidad = self.obtener_copias(&libro);
        if prestamos <= 5 && disponibilidad > 0 {
            self.decrementar_copias(&libro);
            return true;
        }
        false
    }

    pub fn prestamos_a_vencer(&self, fecha: Fecha) -> Vec<&Prestamo> {
        let mut vec = Vec::new();
        for p in &self.prestamos_efectuados {
            if p.vencimiento.es_mayor(&fecha){
                vec.push(p);
            }
        }
        vec
    }
    pub fn prestamos_vencidos(&self, fecha: Fecha) -> Vec<&Prestamo> {
        let mut vec = Vec::new();
        for p in &self.prestamos_efectuados {
            if fecha.es_mayor(&p.vencimiento){
                vec.push(p);
            }
        }
        vec
    }

    pub fn buscar_prestamo(&self, libro: &Libro, cliente: &Cliente) -> bool {
        for p in &self.prestamos_efectuados {
            if p.cliente.equals(cliente) && p.libro.equals(libro){
                return true;
            }
        }
        false
    }

    pub fn devolver_libro(&mut self, libro: &Libro, cliente: &Cliente, fecha: Fecha) {
        for i in 0..self.prestamos_efectuados.len() {
            if self.prestamos_efectuados[i].cliente.equals(cliente) && self.prestamos_efectuados[i].libro.equals(libro){
                self.prestamos_efectuados[i].estado = Estado::Devuelto;
                self.prestamos_efectuados[i].devolucion = Some(fecha);
                return;
            }
        }
    }
}