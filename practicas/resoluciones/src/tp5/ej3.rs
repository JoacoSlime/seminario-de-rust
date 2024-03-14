use std::{collections::VecDeque, fs::File, io::Write};
use serde::{Serialize, Deserialize};

use crate::tp4::Fecha;

#[derive(Debug, PartialEq, Eq, Clone)]
struct Veterinaria{
    nombre: String,
    direccion: String,
    id: u32,
    cola_atencion: VecDeque<Mascota>,
    registro_atencion: Vec<Atencion>,
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
enum Animal {
    Perro,
    Gato,
    Caballo,
    Otros,
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
struct Dueño {
    nombre: String,
    direccion: String,
    telefono: String,
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
struct Mascota {
    nombre: String,
    edad: u32,
    animal: Animal,
    dueño: Dueño,
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
struct Atencion{
    mascota: Mascota,
    diagnostico: String,
    tratamiento: String,
    proxima_visita: Option<Fecha>,
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

    pub fn eliminar_mascota(&mut self, mascota: &Mascota) {
        for i in 0..self.cola_atencion.len() {
            if self.cola_atencion[i] == *mascota {
                self.cola_atencion.remove(i);
                break
            }
        }
    }

    pub fn registrar_atencion(&mut self, atencion: Atencion) {
        self.registro_atencion.push(atencion);
        self.actualizar_archivo();
    } 

    pub fn buscar_atencion(&self, nombre_mascota: String, nombre_dueño: String, telefono: String) -> Option<&Atencion> {
        for i in 0..self.registro_atencion.len() {
            if self.registro_atencion[i].mascota.nombre == nombre_mascota && self.registro_atencion[i].mascota.dueño.nombre == nombre_dueño && self.registro_atencion[i].mascota.dueño.telefono == telefono {
                self.actualizar_archivo();
                return Some(&self.registro_atencion[i]);
            }
        }
        None
    }

    pub fn modificar_diagnostico(&mut self, atencion: &Atencion, nuevo_diagnostico: String) {
        for a in &mut self.registro_atencion {
            if a == atencion {
                a.diagnostico = nuevo_diagnostico;
                return;
            }
        }
    }

    pub fn modificar_fecha(&mut self, atencion: &Atencion, nueva_fecha: Fecha) {
        for i in 0..self.registro_atencion.len() {
            if self.registro_atencion[i] == *atencion {
                self.registro_atencion[i].proxima_visita = Some(nueva_fecha);
                self.actualizar_archivo();
                return;
            }
        }
    }

    pub fn eliminar_fecha(&mut self, atencion: &Atencion) {
        for i in 0..self.registro_atencion.len() {
            if self.registro_atencion[i] == *atencion{
                self.registro_atencion[i].proxima_visita = None;
                self.actualizar_archivo();
                return;
            }
        }
    }

    pub fn eliminar_atencion(&mut self, atencion: &Atencion){
        for i in 0..self.registro_atencion.len() {
            if self.registro_atencion[i] == *atencion {
                self.registro_atencion.remove(i);
                self.actualizar_archivo();
                return;
            }
        }
    }

    fn actualizar_archivo(&self){
        let json = serde_json::to_string_pretty(&self.registro_atencion)
            .expect("Error al serializar");
        let mut file = File::create("registro.json")
            .expect("Error al crear archivo.");
        file.write_all(json.as_bytes());
    }
}

#[cfg(test)]
#[test]
fn new_test() {
    let exp = Veterinaria{
        nombre: String::from("test"),
        direccion: String::from("test"),
        id: u32::default(),
        cola_atencion: VecDeque::new(),
        registro_atencion: Vec::new(),
    };
    let res = Veterinaria::new(
        String::from("test"),
        String::from("test"),
        u32::default(),
    );

    assert_eq!(exp, res);
}

#[cfg(test)]
#[test]
fn agregar_mascota_test() {
    let m = Mascota {
        nombre: String::default(),
        edad: u32::default(),
        animal: Animal::Otros,
        dueño: Dueño { nombre: String::default(), direccion: String::default(), telefono: String::default() },
    };
    let exp = Veterinaria{
        nombre: String::from("test"),
        direccion: String::from("test"),
        id: u32::default(),
        cola_atencion: VecDeque::from([m.clone()]),
        registro_atencion: Vec::new(),
    };
    let mut res = Veterinaria::new(
        String::from("test"),
        String::from("test"),
        u32::default(),
    );
    res.agregar_mascota(m);
    assert_eq!(exp, res);
}

#[cfg(test)]
#[test]
fn agregar_mascota_alta_prioridad_test() {
    let m1 = Mascota {
        nombre: String::from("test1"),
        edad: 10,
        animal: Animal::Otros,
        dueño: Dueño { nombre: String::default(), direccion: String::default(), telefono: String::default() },
    };
    let m2 = Mascota {
        nombre: String::from("test2"),
        edad: 11,
        animal: Animal::Otros,
        dueño: Dueño { nombre: String::default(), direccion: String::default(), telefono: String::default() },
    };
    let m3 = Mascota {
        nombre: String::from("test3"),
        edad: 12,
        animal: Animal::Otros,
        dueño: Dueño { nombre: String::default(), direccion: String::default(), telefono: String::default() },
    };
    let exp = Veterinaria{
        nombre: String::from("test"),
        direccion: String::from("test"),
        id: u32::default(),
        cola_atencion: VecDeque::from([m1.clone(), m2.clone(), m3.clone()]),
        registro_atencion: Vec::new(),
    };
    let mut res = Veterinaria{
        nombre: String::from("test"),
        direccion: String::from("test"),
        id: u32::default(),
        cola_atencion: VecDeque::from([m2.clone(), m3.clone()]),
        registro_atencion: Vec::new(),
    };
    res.agregar_mascota_alta_prioridad(m1);
    assert_eq!(exp, res);
}

#[cfg(test)]
#[test]
fn atender_mascota_test() {
    let m = Mascota {
        nombre: String::default(),
        edad: u32::default(),
        animal: Animal::Otros,
        dueño: Dueño { nombre: String::default(), direccion: String::default(), telefono: String::default() },
    };
    let exp = Veterinaria{
        nombre: String::from("test"),
        direccion: String::from("test"),
        id: u32::default(),
        cola_atencion: VecDeque::new(),
        registro_atencion: Vec::new(),
    };
    let mut res = Veterinaria{
        nombre: String::from("test"),
        direccion: String::from("test"),
        id: u32::default(),
        cola_atencion: VecDeque::from([m]),
        registro_atencion: Vec::new(),
    };
    assert!(res.atender_mascota().is_some());
    assert!(res.atender_mascota().is_none());
    assert_eq!(exp,res);
}

#[cfg(test)]
#[test]
fn eliminar_mascota_test() {
    let m = Mascota {
        nombre: String::default(),
        edad: u32::default(),
        animal: Animal::Otros,
        dueño: Dueño { nombre: String::default(), direccion: String::default(), telefono: String::default() },
    };
    let exp = Veterinaria{
        nombre: String::from("test"),
        direccion: String::from("test"),
        id: u32::default(),
        cola_atencion: VecDeque::new(),
        registro_atencion: Vec::new(),
    };
    let mut res = Veterinaria{
        nombre: String::from("test"),
        direccion: String::from("test"),
        id: u32::default(),
        cola_atencion: VecDeque::from([m.clone()]),
        registro_atencion: Vec::new(),
    };
    res.eliminar_mascota(&m);
    assert_eq!(exp,res);
}

#[cfg(test)]
#[test]
fn registrar_atencion_test() {
    let a = Atencion {
        mascota: Mascota {
            nombre: String::default(),
            edad: u32::default(),
            animal: Animal::Otros,
            dueño: Dueño { nombre: String::default(), direccion: String::default(), telefono: String::default() },
        },
        diagnostico: String::default(),
        tratamiento: String::default(),
        proxima_visita: None,
    };
    let exp = Veterinaria{
        nombre: String::from("test"),
        direccion: String::from("test"),
        id: u32::default(),
        cola_atencion: VecDeque::new(),
        registro_atencion: Vec::from([a.clone()]),
    };
    let mut res = Veterinaria{
        nombre: String::from("test"),
        direccion: String::from("test"),
        id: u32::default(),
        cola_atencion: VecDeque::new(),
        registro_atencion: Vec::new(),
    };
    res.registrar_atencion(a);
    assert_eq!(exp, res);
}

#[cfg(test)]
#[test]
fn buscar_atencion_test() {
    let v = Veterinaria{
        nombre: String::from("test"),
        direccion: String::from("test"),
        id: u32::default(),
        cola_atencion: VecDeque::new(),
        registro_atencion: Vec::from([Atencion {
            mascota: Mascota {
                nombre: String::default(),
                edad: u32::default(),
                animal: Animal::Otros,
                dueño: Dueño { nombre: String::default(), direccion: String::default(), telefono: String::default() },
            },
            diagnostico: String::default(),
            tratamiento: String::default(),
            proxima_visita: None,
        }]),
    };
    let res = v.buscar_atencion(String::from("TEST"), String::from("TEST"), String::from("TEST"));
    assert!(res.is_none());
    let res = v.buscar_atencion(String::default(), String::default(), String::default());
    assert!(res.is_some());
}

#[cfg(test)]
#[test]
fn modificar_diagnostico_test() {
    let exp = Veterinaria{
        nombre: String::from("test"),
        direccion: String::from("test"),
        id: u32::default(),
        cola_atencion: VecDeque::new(),
        registro_atencion: Vec::from([Atencion {
            mascota: Mascota {
                nombre: String::default(),
                edad: u32::default(),
                animal: Animal::Otros,
                dueño: Dueño { nombre: String::default(), direccion: String::default(), telefono: String::default() },
            },
            diagnostico: String::from("test"),
            tratamiento: String::default(),
            proxima_visita: None,
        }]),
    };
    let a = Atencion {
        mascota: Mascota {
            nombre: String::default(),
            edad: u32::default(),
            animal: Animal::Otros,
            dueño: Dueño { nombre: String::default(), direccion: String::default(), telefono: String::default() },
        },
        diagnostico: String::default(),
        tratamiento: String::default(),
        proxima_visita: None,
    };
    let mut res = Veterinaria{
        nombre: String::from("test"),
        direccion: String::from("test"),
        id: u32::default(),
        cola_atencion: VecDeque::new(),
        registro_atencion: Vec::from([a.clone()]),
    };
    res.modificar_diagnostico(&a, String::from("test"));
    assert_eq!(exp,res)
}

#[cfg(test)]
#[test]
fn modificar_fecha_test() {
    let f = Fecha::new(12, 8, 2023);
    let exp = Veterinaria{
        nombre: String::from("test"),
        direccion: String::from("test"),
        id: u32::default(),
        cola_atencion: VecDeque::new(),
        registro_atencion: Vec::from([Atencion {
            mascota: Mascota {
                nombre: String::default(),
                edad: u32::default(),
                animal: Animal::Otros,
                dueño: Dueño { nombre: String::default(), direccion: String::default(), telefono: String::default() },
            },
            diagnostico: String::default(),
            tratamiento: String::default(),
            proxima_visita: Some(f.clone()),
        }]),
    };
    let a = Atencion {
        mascota: Mascota {
            nombre: String::default(),
            edad: u32::default(),
            animal: Animal::Otros,
            dueño: Dueño { nombre: String::default(), direccion: String::default(), telefono: String::default() },
        },
        diagnostico: String::default(),
        tratamiento: String::default(),
        proxima_visita: None,
    };
    let mut res = Veterinaria{
        nombre: String::from("test"),
        direccion: String::from("test"),
        id: u32::default(),
        cola_atencion: VecDeque::new(),
        registro_atencion: Vec::from([a.clone()]),
    };
    res.modificar_fecha(&a, f);
    assert_eq!(exp,res)
}

#[cfg(test)]
#[test]
fn eliminar_fecha_test() {
    let f = Fecha::new(12, 8, 2023);
    let exp = Veterinaria{
        nombre: String::from("test"),
        direccion: String::from("test"),
        id: u32::default(),
        cola_atencion: VecDeque::new(),
        registro_atencion: Vec::from([Atencion {
            mascota: Mascota {
                nombre: String::default(),
                edad: u32::default(),
                animal: Animal::Otros,
                dueño: Dueño { nombre: String::default(), direccion: String::default(), telefono: String::default() },
            },
            diagnostico: String::default(),
            tratamiento: String::default(),
            proxima_visita: None,
        }]),
    };
    let a = Atencion {
        mascota: Mascota {
            nombre: String::default(),
            edad: u32::default(),
            animal: Animal::Otros,
            dueño: Dueño { nombre: String::default(), direccion: String::default(), telefono: String::default() },
        },
        diagnostico: String::default(),
        tratamiento: String::default(),
        proxima_visita: Some(f),
    };
    let mut res = Veterinaria{
        nombre: String::from("test"),
        direccion: String::from("test"),
        id: u32::default(),
        cola_atencion: VecDeque::new(),
        registro_atencion: Vec::from([a.clone()]),
    };
    res.eliminar_fecha(&a);
    assert_eq!(exp,res)
}

#[cfg(test)]
#[test]
fn eliminar_atencion_test() {
    let exp = Veterinaria{
        nombre: String::from("test"),
        direccion: String::from("test"),
        id: u32::default(),
        cola_atencion: VecDeque::new(),
        registro_atencion: Vec::new(),
    };
    let a = Atencion {
        mascota: Mascota {
            nombre: String::default(),
            edad: u32::default(),
            animal: Animal::Otros,
            dueño: Dueño { nombre: String::default(), direccion: String::default(), telefono: String::default() },
        },
        diagnostico: String::default(),
        tratamiento: String::default(),
        proxima_visita: None,
    };
    let mut res = Veterinaria{
        nombre: String::from("test"),
        direccion: String::from("test"),
        id: u32::default(),
        cola_atencion: VecDeque::new(),
        registro_atencion: Vec::from([a.clone()]),
    };
    res.eliminar_atencion(&a);
    assert_eq!(exp,res)
}

#[cfg(test)]
#[test]
fn actualizar_archivo_test() {
    use std::io::Read;

    let vet = Veterinaria::new(
        "test1".to_string(),
        "test2".to_string(),
        0,
    );
    vet.actualizar_archivo();
    let exp = r#"[]"#.to_string();
    let mut file = File::open("registro.json").expect("Error al abrir archivo");
    let mut res = String::new();
    file.read_to_string(&mut res).expect("Error al leer archivo");
    assert_eq!(exp, res);
}