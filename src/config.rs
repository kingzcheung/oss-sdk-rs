
#[derive(Debug,Clone)]
pub struct Config {
    pub endpoint: String,
    pub access_key_id: String,
    pub access_key_secret: String,
    region: Option<String>,
}

impl Config {
    pub fn region(&self) -> Option<&String> {
        self.region.as_ref()
    }
}

#[derive(Default)]
pub struct ConfigBuilder {
    endpoint: String,
    access_key_id: String,
    access_key_secret: String,
    region: Option<String>,
}

impl ConfigBuilder {
    pub fn builder()->Self {
        ConfigBuilder {
            endpoint: String::new(),
            access_key_id: String::new(),
            access_key_secret: String::new(),
            ..Default::default()
        }
    }

    pub fn build(self) ->Config {
        Config {
            endpoint: self.endpoint,
            access_key_id: self.access_key_id,
            access_key_secret: self.access_key_secret,
            region: self.region,
        }
    }
    
    pub fn set_endpoint<S:AsRef<str>>(mut self, endpoint: S) ->Self{
        self.endpoint = endpoint.as_ref().to_string();
        self
    }
    
    pub fn set_access_key_id<S:AsRef<str>>(mut self, access_key_id: S) ->Self {
        self.access_key_id = access_key_id.as_ref().to_string();
        self
    }
    
    pub fn set_access_key_secret<S:AsRef<str>>(mut self, access_key_secret: S) ->Self{
        self.access_key_secret = access_key_secret.as_ref().to_string();
        self
    }
    
    pub fn set_region<S:AsRef<str>>(mut self, region: S) ->Self {
        self.region = Some(region.as_ref().to_string());
        self
    }
    
    
}