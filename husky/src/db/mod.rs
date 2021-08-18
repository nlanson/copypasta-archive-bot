use sqlite::Error as sqErr;

pub struct Database {
    file: String
}

#[derive(Debug)]
pub enum PastaErr {
    DbErr(sqErr)
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
    pub fn add(&self, name: &str, value: &str) -> Result<(), PastaErr> {
        let connection = sqlite::open(&self.file)?;

        let statement: String = format!("insert into pastas (name, value) values('{}', '{}');", name, value);
        connection.execute(statement)?;
        Ok(())
    }

    //Returns the pasta with key name.
    pub fn get(&self, name: &str) -> Result<String, PastaErr> {
        let connection = sqlite::open(&self.file)?;
        let mut db = connection.prepare("select * from pastas where name=?")?;
        db.bind(1, name)?;

        //Return the first item from the query.
        //If no matches then it will return a PastaErr::DbErr()
        db.next()?;
        Ok(db.read::<String>(1)?)
    }
}

mod tests {
    use super::*;
    
    #[test]
    fn add_pasta() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn get_pasta() {
        let key: String = String::from("test");
    }
}