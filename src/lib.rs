use std::{collections::HashMap, ops::Index, fmt};

pub enum Value {
    Simple( String),
    Children( HashMap<String,Value>),
    None,
}
impl Value{
    fn get_string(&self) ->String{
        match self {
            Value::None => {
                String::from(" None")
            },
            Value::Simple(x) => {
                let mut a = String::new();
                a.push_str(" \"");
                a.push_str(x);
                a.push_str("\"");
                a
            },
            Value::Children(x)=>{
                let mut a = String::new();
                a.push_str("\n{");
                for y in x{
                    a.push_str(" \"");
                    a.push_str(y.0);
                    a.push_str("\" ");
                    a.push_str(&y.1.get_string());
                    a.push_str("\n");
                   // let zz = write!(f," \"{}\" {{{}}}",y.0 ,y.1.get_string()) ;
                }
                a.push_str("}\n");
                a
            },
        }
    }
}
impl Index<&str> for Value {
    type Output = Value;
    fn index(&self, index: &str) -> &Value {
        match self {
            Value::Children(x) =>{
                let a = x.get(index);
                match a {
                    Some(b) =>{
                        return b;
                    },
                    None =>{
                        return &Value::None;
                    },
                }
            
            },
            _ =>{
                return &Value::None;
            },

        }
    }
}
impl Index<&str> for KeyValue {
    type Output = Value;
    fn index(&self, index: &str) -> &Value {
        let a = self.kv.get(index);
        match a {
            Some(x)=>x,
            _ =>{
                return &Value::None;
            },

        }
    }
}
impl fmt::Display for Value{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::None => write!(f," None"),
            Value::Simple(x) => {
                let mut a = String::new();
                a.push_str(" \"");
                a.push_str(x);
                a.push_str("\"");
                write!(f,"{}",a)

            },
            Value::Children(x)=>{
                let mut a = String::new();
                for y in x{
                    a.push_str(" \"");
                    a.push_str(y.0);
                    a.push_str("\" ");
                    a.push_str(" \n");
                    a.push_str(&y.1.get_string());
                    //a.push_str("");
                   // let zz = write!(f," \"{}\" {{{}}}",y.0 ,y.1.get_string()) ;
                }
                write!(f,"{}",a)
                

            },
        }
        
    }
    
}
impl fmt::Display for KeyValue{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut a = String::new();
        for (k,v) in &self.kv
        { 
            a.push_str("\"");
            a.push_str(&k);
            a.push_str("\" ");
            a.push_str(" \n");
            a.push_str(&v.get_string());
            a.push_str("\r\n");         
        }
        write!(f,"{}",a)

        
    }
    
}
pub struct KeyValue {
    kv:HashMap<String,Value>
}

pub struct  Reader {
    index:usize,
    chars:Vec<char>,
    
 }
 impl Reader{
    pub fn new(a:String)->Reader{
        let c:Vec<char> = a.chars().collect::<Vec<_>>();
        Reader{
            index:0,
            chars:c,
        }

    }
    pub fn end_of_stream (&self)->bool{
        if self.index >= self.chars.len() {
            true
        }
        else {            
            false
        }
    }
    pub fn peek(&mut self)->char{
        if self.end_of_stream(){
            panic!("end of stream")
        }
        else {
            
            
            let a = self.chars[self.index];
            a
        }
    }
    pub fn read(&mut self)->char{
        if self.end_of_stream(){
            panic!("end of stream")
        }
        else {
            let a = self.chars[self.index];
            self.index = self.index + 1;
            a
        }
    }
    pub fn eat_white_space(&mut self){
        while ! self.end_of_stream(){
                let c = self.peek();
                //println!("{}",c);
                if  !c.is_whitespace() 
                {
                    break;
                }

                self.read();
            }

    }
    pub fn eat_cpp_comment(&mut self)->bool{
        if ! self.end_of_stream()
        {
            let next = self.peek();
            if next == '/' 
            {
                while ! self.end_of_stream(){
                    if self.read()=='\n'{
                        break;
                    }
                }
                return true;
            }

            
        }
        return false;
    }
    pub fn eat_white_and_comment(&mut self) {
        loop {
            self.eat_white_space();       
            if self.end_of_stream(){
               return  
            }                   
            if ! self.eat_cpp_comment(){
                break;
            }
        }
    }
    pub fn eat_statement(&mut self){
        if ! self.end_of_stream()
        {
            let next = self.peek();
            if next == '[' 
            {
                while ! self.end_of_stream(){
                    if self.read()==']'{
                        break;
                    }
                }
                self.eat_white_and_comment();
            }

            
        }

    }
    pub fn read_kv(&mut self)->KeyValue{
        let mut h :HashMap<String, Value> = HashMap::new();
        loop {
            self.eat_white_and_comment();
            if self.end_of_stream() {
                break;
            }
            if self.peek()=='}'{
                break;
            }
            let name = self.get_name();
            let value = self.get_value();
            h.insert(name, value);
        }
        return KeyValue{
            kv:h
        };
    }
    pub fn get_value(&mut self)->Value{
        self.eat_white_and_comment();
        self.eat_statement();
        let a = self.peek();
        if a == '"' {
            return Value::Simple(self.get_name());
        }
        else{
            return  Value::Children(self.get_kv());
        }
    }
    pub fn get_kv(&mut self)->HashMap<String, Value>{
        self.read(); //eat {
        let mut h :HashMap<String, Value> = HashMap::new();
        loop {
            self.eat_white_and_comment();
            if self.end_of_stream() {
                break;
            }
            if self.peek()=='}'{

                self.read();
                break;
            }
            let name = self.get_name();
            let value = self.get_value();
            h.insert(name, value);
        }
        return h;
    }
    pub fn get_name(&mut self)->String{
        self.eat_statement();
        let next:char = self.peek();
        let mut sb = String::new();
        if  next == '"'  {
            self.read();
    
            
            while ! self.end_of_stream() {
                        if self.peek() == '\\' 
                        {
                            self.read();
    
                            let escaped_char = &self.read();
                            match escaped_char {
                                'r' => sb.push('\r'),
                                't' => sb.push('\t'),
                                'n' => sb.push('\n'),
                                 _  => sb.push(*escaped_char),
                            }       
                            continue;
                        }
    
                        if  self.peek() == '"' {
                            self.read();  
                            break;
                        }
                        sb.push(self.read()) ;       
            }        
  
            
        }
        else {
            panic!("not get \" when read name");
        }
        //println!("{}",sb);
        return sb;
    }
 }