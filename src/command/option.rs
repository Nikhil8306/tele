pub struct Opt {
    pub name: String,
    pub notation: Option<String>,
    pub takesValue: bool,
    pub required: bool,
}

impl Opt {
    pub fn new(name: String) -> Self{
        Self{
            name,
            notation: None,
            takesValue: false,
            required:false
        }
    }

    pub fn notation(&mut self, n: String) -> &mut Self {

        self.notation = Some(n);

        self

    }   

    pub fn takesValue(&mut self, take: bool) -> &mut Self {
        
        self.takesValue = take;

        self
    }

    pub fn required(&mut self, isReq: bool) -> &mut Self {

        self.required = isReq;

        self

    }
}
