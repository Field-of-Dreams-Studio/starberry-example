use starberry::preload::*; 
use example_documentation::*; 

#[tokio::main]  
async fn main() { 

    let furl = APP.clone().reg_from(&[LitUrl("flexible"), LitUrl("url"), LitUrl("may_be_changed")]); 
    furl.set_method(Arc::new(flexible_access)); 

    APP.clone().run().await; 
} 

