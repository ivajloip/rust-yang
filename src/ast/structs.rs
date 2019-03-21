#[derive(Debug,PartialEq)]
pub struct Module {
    pub contact: String,
    pub description: String,
    pub name: String,
    pub namespace: String,
    pub organization: String,
    pub prefix: String,
    pub reference: String,
    pub yang_version: String,
}

impl Module {
    pub fn new() -> Self {
        Module{
            contact: String::new(),
            description: String::new(),
            name: String::new(),
            namespace: String::new(),
            organization: String::new(),
            prefix: String::new(),
            reference: String::new(),
            yang_version: String::new(),
        }
    }

    pub fn contact(&self)-> &str {
        &self.contact
    }

    pub fn description(&self)-> &str {
        &self.description
    }

    pub fn name(&self)-> &str {
        &self.name
    }

    pub fn namespace(&self)-> &str {
        &self.namespace
    }

    pub fn organization(&self)-> &str {
        &self.organization
    }

    pub fn prefix(&self)-> &str {
        &self.prefix
    }

    pub fn reference(&self)-> &str {
        &self.reference
    }

    pub fn yang_version(&self)-> &str {
        &self.yang_version
    }
}

#[derive(Debug,PartialEq)]
pub enum Node {
    Contact(String),
    Description(String),
    Namespace(String),
    Organization(String),
    Prefix(String),
    Reference(String),
    Unknown(String),
    YangVersion(String),
}
