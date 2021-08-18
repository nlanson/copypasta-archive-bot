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

    pub fn reset(&self) -> Result<(), PastaErr> {
        let connection = sqlite::open(&self.file)?;

        let statement: String = format!("delete from pastas where name != 'test'");
        connection.execute(statement)?;
        Ok(())
    }
}

mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn add_pasta_success() {
        //Set test data
        let key: String = String::from("testdata");
        let value: String = String::from("This is test data.");

        //Create DB Struct
        let db = Database::new(String::from("./pastas.db"));

        //Invoke the add() method and check if the method returns Ok()
        match db.add(&key, &value) {
            //If Ok()
            Ok(_) => {
                //Get the saved test data from the DB to check if saved.
                match db.get(&key) {
                    //If value is returned
                    Ok(pasta) => {
                        //Reset the database
                        match db.reset() {_=>{}}
                        //Check if the pasta returned is "Hello World!"
                        assert_eq!(pasta, value);
                    },

                    //On get() Err.
                    _ => {
                        //Reset and panic
                        match db.reset() {_=>{}}
                        panic!("Failed to get test data pasta");
                    }
                }
            },
            
            //If Err()
            _ => {
                //Reset and panic
                match db.reset() {_=>{}}
                panic!("Failed to save pasta.");
            }
        }
    }

    #[test]
    fn get_pasta_success() {
        //Use the default test pasta to test get functionality.
        let key: String = String::from("test");

        //Create a DB Struct
        let db = Database::new(String::from("./pastas.db"));

        //Invoke the get() method and check if the method returns Ok()
        match db.get(&key) {
            //If Ok()
            Ok(pasta) => {
                //Check if the pasta returned is "Hello World!"
                assert_eq!(pasta, "Hello World!");
            },
            
            //If Err()
            _ => {
                //Panic and fail the test
                panic!("Failed to get pasta");
            }
        }

    }
}