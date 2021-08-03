use sqlite::Error as sqErr;

pub struct Database {
    file: String
}

#[derive(Debug)]
pub enum PastaErr {
    DbErr(sqErr),
    UsrErr(String)
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
        //Trying better methods.
        //Not working yet.
        let mut db = connection.prepare(
   "
            if not exists (select * from pastas where name=?)
            begin
            insert into pastas (name, value) values (?, ?)
            end
            "
        )?;
        db.bind(1, name)?;
        db.bind(2, name)?;
        db.bind(3, content)?;
        Ok(())
        

        //Inserts the params into the database.
        //Check for duplicates by getting the name from the database first.
        // match Self::get(&self, name) {
        //     //If get succeeds, then there is a duplicate.
        //     //Return a user error
        //     Ok(_) => Err(PastaErr::UsrErr(String::from("duplicate pasta"))),

        //     //If the get fails, then there is most likely no duplicate.
        //     //Add the pasta into the database and return Ok.
        //     Err(_) => {
        //         let mut db = connection.prepare("insert into pastas (name, value) values (?, ?);")?;
        //         db.bind(1, name)?;
        //         db.bind(2, content)?;
        //         db.next()?;
        //         Ok(())
        //     }
        // }
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