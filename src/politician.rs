use std::fmt;

pub struct Politician {
  name: String,			    // name of politician
  state: String,		    // state of politician
  position: Position,		    // posititon of politician (house / senate)
  party: Party,			    // party of politician (republican / democrat)
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
    
