use std::{fs::{File, OpenOptions}, io::{Write, Read, Error}, marker::PhantomData};
use serde::{Serialize, Deserialize};

struct FileManager<'a, T>
where
    T: Serialize + Deserialize<'a>
{
    path: &'a str,
    buf: String,
    collapse: PhantomData<T>
}

impl<'a, T> FileManager<'a, T> 
where
    T: Serialize + Deserialize<'a>
{
    pub fn new(path: &'a str) -> FileManager<'a, T> {
        FileManager { path, buf: String::new(), collapse: PhantomData}
    }

    pub fn cargar(&'a mut self) -> Result<Vec<T>, FmError> {
        self.buf.clear();
        let file = File::open(self.path);
        match file {
            Ok(mut file) => {
                match file.read_to_string(&mut self.buf){
                    Ok(_) => {},
                    Err(_) => return Err(FmError::Read),
                }
            },
            Err(_) => return Err(FmError::Open),
        };
        let vec = serde_json::from_str(self.buf.as_str());
        match vec {
            Ok(vec) => Ok(vec),
            Err(_) => Err(FmError::Deserialize),
        }
    }

    pub fn guardar(&'a mut self, dato: &T) -> Result<(), FmError> {
        let read = serde_json::to_string_pretty(dato);
        self.buf = match read {
            Ok(s) => s,
            Err(_) => return Err(FmError::Serialize),
        };
        let file = File::create(self.path);
        let mut file = match file {
            Ok(mut file) => file,
            Err(_) => return Err(FmError::Create),
        };
        let res = file.write_all(self.buf.as_bytes());
        match res {
            Ok(_) => Ok(()),
            Err(_) => Err(FmError::Write),
        }
    }
}

enum FmError {
    Deserialize,
    Serialize,
    Open,
    Create,
    Read,
    Write,
}