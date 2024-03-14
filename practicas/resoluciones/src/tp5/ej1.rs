use std::{fmt::Display, fs::OpenOptions, io::Write};
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize, Debug)]
struct ConcesionarioAuto {
    nombre: String,
    direccion: String,
    capacidad_max: i32,
    autos: Vec<Auto>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Auto {
    marca: String,
    modelo: String,
    año: i32,
    precio_bruto: f64,
    color: Color,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
enum Color {
    Rojo,
    Verde,
    Azul,
    Amarillo,
    Blanco,
    Negro,
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

    pub fn agregar_auto(&mut self, auto: Auto) -> Result<(), ErrorConsecionaria>{
        if self.autos.len() < self.capacidad_max as usize {
            self.autos.push(auto);
            let file = OpenOptions::new().read(true).write(true).create(true).open("autos.json");
            match file {
                Ok(mut f) => {
                    let Ok(s) = serde_json::to_string(&self.autos) else {return Err(ErrorConsecionaria::Archivo)};
                    let Ok(_) = f.write_all(s.as_bytes()) else {return Err(ErrorConsecionaria::Archivo)};
                    return Ok(())
                },
                Err(_) => return Err(ErrorConsecionaria::Archivo),
            }
        }
        Err(ErrorConsecionaria::Agregar)
    }

    pub fn eliminar_auto(&mut self, auto: &Auto) -> Result<(), ErrorConsecionaria>{
        for i in 0..self.autos.len() {
            if auto == &self.autos[i] {
                self.autos.remove(i);
                let file = OpenOptions::new().read(true).write(true).open("autos.json");
                match file {
                    Ok(mut f) => {
                        let Ok(s) = serde_json::to_string(&self.autos) else {return Err(ErrorConsecionaria::Archivo)};
                        let Ok(_) = f.write_all(s.as_bytes()) else {return Err(ErrorConsecionaria::Archivo)};
                        return Ok(())
                    },
                    Err(_) => return Err(ErrorConsecionaria::Archivo),
                }
            };
        }
        Err(ErrorConsecionaria::Eliminar)
    }

    pub fn buscar_auto(&mut self, auto: &Auto) -> Option<&Auto>{
        let mut ret: Option<&Auto> = None;
        for i in 0..self.autos.len() {
            if auto == &self.autos[i] {
                ret = Some(&self.autos[i]);
            }
        }
        ret
    }
}

#[derive(Debug, PartialEq, Eq)]
enum ErrorConsecionaria {
    Agregar,
    Eliminar,
    Archivo,
}
impl Display for ErrorConsecionaria {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorConsecionaria::Agregar => write!(f, "Error al agregar auto, máximo alcanzado"),
            ErrorConsecionaria::Eliminar => write!(f, "Error al eliminar auto, no fue encontrado"),
            ErrorConsecionaria::Archivo => write!(f, "Error al escribir el archivo"),
        }
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


#[cfg(test)]
#[test]
fn agregar_auto_test() {
    let mut concesionario = ConcesionarioAuto{
        nombre: String::default(),
        direccion: String::default(),
        capacidad_max: 1,
        autos: Vec::with_capacity(1),
    };

    let auto = Auto {
        marca: String::default(),
        modelo: String::default(),
        año: i32::default(),
        precio_bruto: f64::default(),
        color: Color::Amarillo,
    };
    let result = concesionario.agregar_auto(auto);
    let expected = Ok(());
    assert_eq!(result,expected);

    let auto = Auto {
        marca: String::default(),
        modelo: String::default(),
        año: i32::default(),
        precio_bruto: f64::default(),
        color: Color::Amarillo,
    };
    let result = concesionario.agregar_auto(auto);
    let expected = Err(ErrorConsecionaria::Agregar);
    assert_eq!(result, expected);
}


#[cfg(test)]
#[test]
fn eliminar_auto_test(){
    let mut concesionario = ConcesionarioAuto{
        nombre: String::default(),
        direccion: String::default(),
        capacidad_max: 1,
        autos: Vec::with_capacity(1),
    };
    let auto = Auto {
        marca: String::default(),
        modelo: String::default(),
        año: i32::default(),
        precio_bruto: f64::default(),
        color: Color::Amarillo,
    };
    let _ =concesionario.agregar_auto(auto);


    let auto = Auto {
        marca: String::default(),
        modelo: String::default(),
        año: i32::default(),
        precio_bruto: f64::default(),
        color: Color::Verde,
    };
    let result = concesionario.eliminar_auto(&auto);
    let expected = Err(ErrorConsecionaria::Eliminar);
    assert_eq![result,expected];

    let auto = Auto {
        marca: String::default(),
        modelo: String::default(),
        año: i32::default(),
        precio_bruto: f64::default(),
        color: Color::Amarillo,
    };
    let result = concesionario.eliminar_auto(&auto);
    let expected = Ok(());
    assert_eq![result, expected];
}