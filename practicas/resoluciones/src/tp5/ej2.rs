use std::{collections::VecDeque, fs::File, io::Write};
use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
enum Genero{
    Rock,
    Pop,
    Rap,
    Jazz,
    Otros,
}

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
struct Cancion {
    titulo: String,
    artista: String,
    genero: Genero,
}


#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
struct Playlist {
    canciones: VecDeque<Cancion>,
    nombre: String,
}

impl Cancion {
    pub fn new(titulo: String, artista: String, genero: Genero) -> Cancion {
        Cancion { titulo, artista, genero}
    }
}

impl Playlist {
    pub fn new(nombre: String) -> Playlist {
        Playlist { canciones: VecDeque::new(), nombre }
    }

    pub fn agregar_cancion(&mut self, cancion: Cancion) {
        self.canciones.push_back(cancion);
        self.actualizar_archivo();
    }

    fn get_pos_cancion(&mut self, cancion: &Cancion) -> Option<usize> {
        (0..self.canciones.len()).find(|&i| self.canciones[i] == *cancion)
    }

    pub fn eliminar_cancion(&mut self, cancion: &Cancion) {
        if let Some(pos) = self.get_pos_cancion(cancion){
            self.canciones.remove(pos);
            self.actualizar_archivo();
        }
    }

    pub fn mover_cancion(&mut self, cancion: &Cancion, pos: usize) {
        if let Some(i) = self.get_pos_cancion(cancion) {
            let cancion = self.canciones.remove(i).unwrap();
            self.canciones.insert(pos, cancion);
            self.actualizar_archivo();
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
            if self.canciones[i].genero == *genero{
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
        self.canciones.clear();
        self.actualizar_archivo();
    }

    pub fn actualizar_archivo(&self) {
        let json: String = serde_json::to_string_pretty(&self.canciones).expect("Error al serializar playlist");
        let mut file = File::create("playlist.json").expect("Error al crear/leer archivo");
        file.write_all(json.as_bytes()).expect("Error al escribir");
    }
}


#[cfg(test)]
#[test]
fn new_cancion_test() {
    let can1 = Cancion {
        titulo: String::from("Test1"),
        artista: String::from("Test2"),
        genero: Genero::Otros,
    };
    let can2 = Cancion::new(String::from("Test1"), String::from("Test2"), Genero::Otros);
    assert_eq!(can1,can2);
}

#[cfg(test)]
#[test]
fn new_playlist_test() {
    let pla1 = Playlist {
        canciones: VecDeque::new(),
        nombre: String::from("test"),
    };
    let pla2 = Playlist::new(String::from("test"));
    assert_eq!(pla1, pla2);
}

#[cfg(test)]
#[test]
fn agregar_cancion_test() {
    let can = Cancion::new(String::from("Test1"), String::from("Test2"), Genero::Otros);
    let pla1 = Playlist {
        canciones: VecDeque::from(vec![can.clone()]),
        nombre: String::from("test"),
    };
    let mut pla2 = Playlist::new(String::from("test"));
    pla2.agregar_cancion(can);
    assert_eq!(pla1, pla2);
}

#[cfg(test)]
#[test]
fn get_pos_cancion_test() {
    let mut pla = Playlist::new(String::from("test"));
    pla.agregar_cancion(
        Cancion::new(String::from("Test1"), String::from("Test1"), Genero::Otros)
    );
    pla.agregar_cancion(
        Cancion::new(String::from("Test2"), String::from("Test2"), Genero::Otros)
    );
    pla.agregar_cancion(
        Cancion::new(String::from("Test3"), String::from("Test3"), Genero::Otros)
    );
    assert!(pla.get_pos_cancion(&Cancion::new(String::from("Test1"), String::from("Test1"), Genero::Otros)).is_some());
    assert!(pla.get_pos_cancion(&Cancion::new(String::from("Test1"), String::from("Test2"), Genero::Otros)).is_none());
    assert_eq!(pla.get_pos_cancion(&Cancion::new(String::from("Test1"), String::from("Test1"), Genero::Otros)).unwrap(), 0);
}

#[cfg(test)]
#[test]
fn eliminar_cancion_test() {
    let pla1 = Playlist {
        canciones: VecDeque::from(vec![
            Cancion::new(String::from("Test1"), String::from("Test1"), Genero::Otros),
            Cancion::new(String::from("Test2"), String::from("Test2"), Genero::Otros),
            Cancion::new(String::from("Test3"), String::from("Test3"), Genero::Otros)
        ]),
        nombre: String::from("test"),
    };
    let mut pla2 = pla1.clone();
    let can = Cancion::new(String::from("Test0"), String::from("Test0"), Genero::Otros);
    pla2.eliminar_cancion(&can);
    assert_eq!(pla1,pla2);
    let can = Cancion::new(String::from("Test1"), String::from("Test1"), Genero::Otros);
    pla2.eliminar_cancion(&can);
    assert_ne!(pla1,pla2);
}

#[cfg(test)]
#[test]
fn mover_cancion_test() {
    let pla1 = Playlist {
        canciones: VecDeque::from(vec![
            Cancion::new(String::from("Test1"), String::from("Test1"), Genero::Otros),
            Cancion::new(String::from("Test2"), String::from("Test2"), Genero::Otros),
            Cancion::new(String::from("Test3"), String::from("Test3"), Genero::Otros)
        ]),
        nombre: String::from("test"),
    };
    let mut pla2 = pla1.clone();
    let can = Cancion::new(String::from("Test0"), String::from("Test0"), Genero::Otros);
    pla2.mover_cancion(&can, 2);
    assert_eq!(pla1,pla2);
    let can = Cancion::new(String::from("Test1"), String::from("Test1"), Genero::Otros);
    pla2.mover_cancion(&can, 2);
    assert_ne!(pla1,pla2);
}


#[cfg(test)]
#[test]
fn buscar_cancion_test () {
    let pla = Playlist {
        canciones: VecDeque::from(vec![
            Cancion::new(String::from("Test1"), String::from("Test"), Genero::Otros),
            Cancion::new(String::from("Test2"), String::from("Test"), Genero::Otros),
            Cancion::new(String::from("Test3"), String::from("Test"), Genero::Otros)
        ]),
        nombre: String::from("test"),
    };
    assert!(pla.buscar_cancion(String::from("Test0")).is_none());
    assert!(pla.buscar_cancion(String::from("Test1")).is_some());
}

#[cfg(test)]
#[test]
fn get_from_genero_test () {
    let pla = Playlist {
        canciones: VecDeque::from(vec![
            Cancion::new(String::from("Test1"), String::from("Test"), Genero::Otros),
            Cancion::new(String::from("Test2"), String::from("Test"), Genero::Otros),
            Cancion::new(String::from("Test3"), String::from("Test"), Genero::Jazz)
        ]),
        nombre: String::from("test"),
    };
    let can = &Cancion::new(String::from("Test3"), String::from("Test"), Genero::Jazz);
    let res = pla.get_from_genero(&Genero::Jazz);
    let exp = vec![can];
    assert_eq!(res,exp);
}

#[cfg(test)]
#[test]
fn get_from_artista_test () {
    let pla = Playlist {
        canciones: VecDeque::from(vec![
            Cancion::new(String::from("Test1"), String::from("Test1"), Genero::Otros),
            Cancion::new(String::from("Test2"), String::from("Test1"), Genero::Otros),
            Cancion::new(String::from("Test1"), String::from("Test2"), Genero::Otros),
            Cancion::new(String::from("Test1"), String::from("Test3"), Genero::Otros),
            Cancion::new(String::from("Test2"), String::from("Test3"), Genero::Otros),
        ]),
        nombre: String::from("test"),
    };
    let can = &Cancion::new(String::from("Test1"), String::from("Test2"), Genero::Otros);
    let res = pla.get_from_artista(String::from("Test2"));
    let exp = vec![can];
    assert_eq!(res,exp);
}

#[cfg(test)]
#[test]
fn set_titulo_test () {
    let mut pla = Playlist::new(String::from("test"));
    pla.set_titulo(String::from("test2"));
    assert_eq!(pla.nombre, String::from("test2"));
}

#[cfg(test)]
#[test]
fn limpiar_lista_test () {
    let mut pla = Playlist {
        canciones: VecDeque::from(vec![
            Cancion::new(String::from("Test1"), String::from("Test"), Genero::Otros),
            Cancion::new(String::from("Test2"), String::from("Test"), Genero::Otros),
            Cancion::new(String::from("Test3"), String::from("Test"), Genero::Jazz)
        ]),
        nombre: String::from("test"),
    };
    pla.limpiar_lista();
    let exp = Playlist::new(String::from("test"));
    assert_eq!(pla, exp);
}

#[cfg(test)]
#[test]
fn actualizar_archivo_test () {
    use std::io::Read;

    let pla1 = Playlist {
        canciones: VecDeque::from(vec![
            Cancion::new(String::from("Test1"), String::from("Test1"), Genero::Rap),
            Cancion::new(String::from("Test2"), String::from("Test2"), Genero::Rock),
            Cancion::new(String::from("Test3"), String::from("Test3"), Genero::Jazz)
        ]),
        nombre: String::from("test"),
    };
    pla1.actualizar_archivo();
    let expected = r#"[
  {
    "titulo": "Test1",
    "artista": "Test1",
    "genero": "Rap"
  },
  {
    "titulo": "Test2",
    "artista": "Test2",
    "genero": "Rock"
  },
  {
    "titulo": "Test3",
    "artista": "Test3",
    "genero": "Jazz"
  }
]"#.to_string();
    let mut file = File::open("playlist.json").expect("Error al abrir archivo");
    let mut result = String::new();
    file.read_to_string(&mut result).expect("Error al leer archivo");
    assert_eq!(result, expected);
}