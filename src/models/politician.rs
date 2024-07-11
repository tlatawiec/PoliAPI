use std::fmt;

use rusqlite::{
  Result, 
  ToSql, 
  types::{
    ToSqlOutput, 
    ValueRef, 
    FromSql, 
    FromSqlError,
  },
};

pub struct Politician {
  pub name: String,		    // name of politician
  pub state: String,		    // state of politician
  pub position: Position,	    // posititon of politician (house / senate)
  pub party: Party,		    // party of politician (republican / democrat)
}

// enum representing the political postion of the politician
pub enum Position {
  House,
  Senate,
}

// display function for position enum
impl fmt::Display for Position {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Position::House => write!(f, "House"),
      Position::Senate => write!(f, "Senate"),
    }
  }
}

impl ToSql for Position {
  fn to_sql(&self) -> Result<ToSqlOutput> {
    match *self {
      Position::House => Ok(ToSqlOutput::from("House")),
      Position::Senate => Ok(ToSqlOutput::from("Senate")),
    }
  }
}

impl FromSql for Position {
  fn column_result(value: ValueRef) -> std::result::Result<Self, FromSqlError> {
    match value.as_str()? {
      "House" => Ok(Position::House),
      "Senate" => Ok(Position::Senate),
      _ => Err(FromSqlError::Other(Box::new(
        rusqlite::Error::InvalidColumnType(0, "Expected House or Senate".to_string(), value.data_type()),
      ))),
    }
  }
}
// enum representing the political party of the politician
pub enum Party {
  Republican,
  Democrat,
  Independent,
} 

// display function for party enum
impl fmt::Display for Party {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Party::Republican => write!(f, "Republican"),
      Party::Democrat => write!(f, "Democrat"),
      Party::Independent => write!(f, "Independent"),
    }
  }
}

impl ToSql for Party {
    fn to_sql(&self) -> Result<ToSqlOutput> {
        match *self {
            Party::Republican => Ok(ToSqlOutput::from("Republican")),
            Party::Democrat => Ok(ToSqlOutput::from("Democrat")),
            Party::Independent => Ok(ToSqlOutput::from("Independent")),
        }
    }
}


impl FromSql for Party {
  fn column_result(value: ValueRef) -> std::result::Result<Self, FromSqlError> {
    match value.as_str()? {
      "Republican" => Ok(Party::Republican),
      "Democrat" => Ok(Party::Democrat),
      "Independent" => Ok(Party::Independent),
      _ => Err(FromSqlError::Other(Box::new(
        rusqlite::Error::InvalidColumnType(0, "Expected Republican, Democrat, or Independent".to_string(), value.data_type()),
      ))),
    }
  }
}

impl Politician {
  // politician contructor
  pub fn new(name: String, state: String, position: Position, party: Party) -> Politician {
    Politician { 
      name,
      state, 
      position, 
      party 
    }
  }
  // print function for politician
  pub fn print(&self) {
    println!("Name: {} [\n\tState: {} Position: {} Party: {}", self.name, self.state, self.position, self.party);
  }
} 
    
