pub struct Opt {
    pub name: String,
    pub notation: Option<String>,
    pub takesValue: bool,
    pub required: bool,
}

impl Opt {
    pub fn new(name: &str) -> Box<Self>{
        Box::new(Self{
            name: name.to_string(),
            notation: None,
            takesValue: false,
            required:false
        })
    }

    pub fn notation(mut self: Box<Self>, n: &str) -> Box<Self> {

        self.notation = Some(n.to_string());

        self

    }   

    pub fn takesValue(mut self: Box<Self>, take: bool) -> Box<Self> {
        
        self.takesValue = take;

        self
    }

    pub fn required(mut self: Box<Self>, isReq: bool) -> Box<Self> {

        self.required = isReq;

        self

    }
}
