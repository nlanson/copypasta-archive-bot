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
    
    //Adds a new pasta with key name and value content.
    pub fn add(&self, name: &str, content: &str) -> Result<(), PastaErr> {
        let connection = sqlite::open(&self.file)?;

        //Inserts the params into the database.
        //Need to check for duplicates.
        let mut db = connection.prepare("insert into pastas (name, value) values (?, ?);")?;
        db.bind(1, name)?;
        db.bind(2, content)?;
        db.next()?;
        Ok(())
    }

    //Returns the pasta with key name.
    pub fn get(&self, name: &str) -> Result<String, PastaErr> {
        let connection = sqlite::open(&self.file)?;
        let mut db = connection.prepare("select * from pastas where name=?")?;
        db.bind(1, name)?;

        db.next()?;
        Ok(db.read::<String>(1)?)
    }
}