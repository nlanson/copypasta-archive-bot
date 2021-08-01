use sqlite::Error as sqErr;
use sqlite::State;

pub struct Database {
    file: String
}

#[derive(Debug)]
pub enum PastaErr {
    DbErr(sqErr),
}

impl From<sqErr> for PastaErr {
    fn from(s: sqErr) -> Self {
        PastaErr::DbErr(s)
    }
}

impl Database {
    pub fn new(file: String) -> Self {
        Self {
            file: file
        }
    }
    
    pub fn add(&self, name: &str, content: &str) -> Result<(), PastaErr> {
        let connection = sqlite::open(&self.file)?;
        let mut db = connection.prepare("insert into pastas (name, value) values (?, ?);")?;
        db.bind(1, name)?;
        db.bind(2, content)?;
        db.next()?;
        Ok(())
    }

    //Not yet working
    //Returns error: "colum index out of range"
    pub fn get(&self, name: &str) -> Result<(), PastaErr> {
        let connection = sqlite::open(&self.file)?;
        let mut statement = connection.prepare("select * from pastas where name='?'")?;
        statement.bind(1, name)?;

        while let State::Row = statement.next()? {
            println!("result");
        }

        Ok(())
    }
}