use postgres::{self, Client, NoTls};

struct Connection {
    client: Client,
}

/* SQL - IN POSTGRESQL

CREATE DATABASE db
    WITH
    OWNER = postgres
    ENCODING = 'UTF8'
    CONNECTION LIMIT = -1
    IS_TEMPLATE = False;

CREATE TABLE public.persons
(
    id serial,
    name character(300),
    PRIMARY KEY (id)
);

ALTER TABLE IF EXISTS public.persons
    OWNER to postgres;

*/

#[derive(Debug)]
struct Person {
    id: i32,
    name: String
}

impl Connection {
    fn new(db_url: &str) -> Result<Self, postgres::Error> {
        let client = Client::connect(db_url, NoTls)?;
        Ok(Self {client})
    }

    #[allow(dead_code)]
    fn create(&mut self, tbl_name: &str, name: &str) -> Result<String, postgres::Error> {
        let rows = self.client.execute(
            format!("INSERT INTO {} (name) VALUES ($1)", tbl_name).as_str(),
            &[&name],
        )?;
        let result = match rows {
            1 => String::from(format!("Registry \"{}\", created!", name)),
            _ => String::from("No records have been created!")
        };
        Ok(result)
    }

    #[allow(dead_code)]
    fn read(&mut self, tbl_name: &str) -> Result<Vec<Person>, postgres::Error> {
        let rows = self.client.query(
            &format!("SELECT * FROM {} ORDER BY id", tbl_name), &[]
        )?;
        let mut result: Vec<Person> = Vec::new();
        for row in rows {
            let name: String = row.get(1);
            let person = Person {
                id: row.get(0),
                name: name.trim().to_string(),
            };
            result.push(person);
        }
        Ok(result)
    }

    #[allow(dead_code)]
    fn update(&mut self, tbl_name: &str, id: i32, name: &str) -> Result<String, postgres::Error> {
        let rows = self.client.execute(
            format!("UPDATE {} SET name = $1 WHERE id = $2", tbl_name).as_str(),
            &[&name, &id],
        )?;
        let result = match rows {
            1 => String::from(format!("Registry id: {}, updated!!", id)),
            _ => String::from("No records have been updated!")
        };
        Ok(result)
    }

    #[allow(dead_code)]
    fn delete(&mut self, tbl_name: &str, id: i32) -> Result<String, postgres::Error> {
        let rows = self.client.execute(
            format!("DELETE FROM {} WHERE id = $1", tbl_name).as_str(),
            &[&id],
        )?;
        let result = match rows {
            1 => String::from(format!("Registry id: {}, deleted!", id)),
            _ => String::from("No records were deleted!")
        };
        Ok(result)
    }

}

fn main() -> Result<(), postgres::Error> {
    let user = "postgres";
    let password = "postgres";
    let host = "localhost";
    let port = 5432;
    let dbname = "db";
    let tbl_name = "persons";

    let db_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        user,
        password,
        host,
        port,
        dbname
    );
    let mut conn = Connection::new(db_url.as_str())?;

    // Create data into a table
    // println!("{:?}", conn.create(tbl_name, "William C. Canin")?);
    // Update data into a table
    // println!("{:?}", conn.update(tbl_name, 1, "Will C. Canin")?);
    // Delete data into a table
    // println!("{:?}", conn.delete(tbl_name, 1)?);

    // Read data into a table
    let persons = conn.read(tbl_name)?;
    println!("id | name");
    println!("----------");
    for person in persons {
        println!("{}  | {}", person.id, person.name);
    }

    Ok(())
}
