use std::{fs::File, io::Read}; 

use starberry::prelude::*;  

pub static APP: SApp = Lazy::new( || { 
    App::new()
        .binding(String::from("127.0.0.1:1111"))
        .mode(RunMode::Build)
        .max_body_size(1024 * 1024 * 10) 
        .max_header_size(1024 * 10) 
        .append_middleware::<MyMiddleWare2>() // Appending the middleware to the last in the middleware chain 
        .append_middleware::<MyMiddleWare1>() 
        .append_middleware::<MyMiddleWare3>() 
        .append_middleware::<MyMiddleWare4>() 
        .append_middleware::<MyMiddleWare5>() 
        .build() 
});  

#[middleware]
pub async fn MyMiddleWare1(){ 
    println!("Middleware: Received request for {}, start processing", req.path()); 
    next(req)  
}  

#[middleware]
pub async fn MyMiddleWare2(){ 
    let path = req.path().to_owned(); 
    let a = next(req).await; 
    println!("Middleware: Received request for {}, end processing", path); // You cannot access to req here 
    a.boxed_future() 
}  

#[middleware]
pub async fn MyMiddleWare3(){ 
    if req.path() == "/directly_return" { 
        req.response = text_response("Directly return"); 
        req.boxed_future() 
    } else {
        next(req) 
    } 
} 

#[middleware] 
pub async fn MyMiddleWare4(){ 
    println!("Middleware: Received request for {}, start processing", req.path()); 
    req.set_local("some_value", 5); 
    req.set_param(true); 
    next(req) 
} 

#[middleware] 
pub async fn MyMiddleWare5(){ 
    req = next(req).await; 
    let value = req.take_local::<i32>("some_value").unwrap_or(0); 
    let param = req.take_param::<bool>().unwrap_or(false); 
    println!("Local: {}, Params: {}", value, param); 
    req.boxed_future() 
}

#[url(APP.lit_url("/"))] 
async fn home_route(mut req: Rc) -> HttpResponse { 
    html_response("<h1>Home</h1>") 
} 

#[url(APP.lit_url("/random/split/something"))]
async fn random_route(mut req: Rc) -> Rc {
    req.response = text_response("A random page"); 
    req 
}  

#[url(APP.lit_url("/directly_return"))]
async fn directly_return() -> HttpResponse {
    text_response("A random page") 
}  

#[url(APP.lit_url("/print_sequence"))]
async fn print_sequence() -> HttpResponse {
    println!("Processing request"); 
    text_response("See console") 
}  

#[url(APP.lit_url("random"))]
async fn anything_random() -> HttpResponse {
    text_response("A random page") 
}  

static TEST_URL: SPattern = Lazy::new(|| {LitUrl("test")});

#[url(reg![&APP, TEST_URL.clone(), LitUrl("hello")])]
async fn hello() -> HttpResponse { 
    text_response("Hello")  
} 

#[url(reg![&APP, TEST_URL, LitUrl("json_old")])]
async fn json_test() -> HttpResponse { 
    let a = 2; 
    let body = object!({number: a, string: "Hello", array: [1, 2, 3]}); 
    json_response(body)
} 

#[url(APP.reg_from(&[TEST_URL.clone(), LitUrl("json")]))]
async fn json_new_test() -> HttpResponse { 
    akari_json!({
        number: 3, 
        string: "Hello", 
        array: [1, 2, 3], 
        object: { 
            a: 1, 
            b: 2, 
            c: 3 
        } 
    }) 
} 

#[url(APP.reg_from(&[TEST_URL.clone(), LitUrl("async_test")]))] 
async fn async_test() -> HttpResponse {
    sleep(Duration::from_secs(1));
    println!("1");
    sleep(Duration::from_secs(1)); 
    println!("2");
    sleep(Duration::from_secs(1));
    println!("3");
    text_response("Async Test Page") 
} 

#[url(APP.reg_from(&[TEST_URL.clone(), RegUrl("async_test2")]))]  
async fn async_test2() -> HttpResponse {
    sleep(Duration::from_secs(1));
    println!("1");
    sleep(Duration::from_secs(1));
    println!("2");
    sleep(Duration::from_secs(1));
    println!("3");
    text_response("Async Test Page") 
} 

#[url(reg![&APP, TEST_URL, RegUrl("[0-9]+")])]  
async fn testa() -> HttpResponse { 
    text_response("Number page") 
} 

#[url(reg![&APP, TEST_URL, LitUrl("form_url_coded")])]  
async fn test_form() -> HttpResponse { 
    println!("Request to this dir"); 
    if req.method() == POST { 
        match req.form().await { 
            Some(form) => { 
                return text_response(format!("Form data: {:?}", form)); 
            } 
            None => { 
                return text_response("Error parsing form"); 
            }  
        } 
    } 
    plain_template_response("form.html") 
} 

#[url(APP.reg_from(&[TEST_URL.clone(), LitUrl("form")]))]  
async fn test_file(context: Rc) -> HttpResponse { 
    println!("Request to this dir"); 
    if context.method() == POST { 
        return text_response(format!("{:?}", context.files_or_default().await)); 
    } 
    plain_template_response("form.html") 
} 

#[url(APP.reg_from(&[TEST_URL.clone(), LitUrl("cookie")]))]  
async fn test_cookie(context: Rc) -> HttpResponse { 
    if context.method() == POST { 
        match context.form().await { 
            Some(form) => { 
                let default_string = String::new(); 
                let name = form.get("name").unwrap_or(&default_string); 
                let value = form.get("value").unwrap_or(&default_string); 
                return text_response(format!("Cookie set data, {}: {}", name, value))
                    .add_cookie(Cookie::new(name, value)); 
            } 
            None => { 
                return text_response("Error parsing form"); 
            }  
        } 
    } 
    let cookies = context.get_cookies(); 
    // Convert cookies into a string in the same variable name 
    let mut scookie = String::new(); 
    for (name, value) in cookies.iter() { 
        scookie.push_str(&format!("{}: {}\n", name, value)); 
    } 
    let scookie = object!(scookie); 
    akari_render!(
        "cookie.html", 
        current_cookie = scookie
    ) 
} 

#[url(APP.reg_from(&[TEST_URL.clone(), LitUrl("temp")]))]  
async fn test_template() -> HttpResponse { 
    akari_render!(
        "home.html", 
        title="My Website - Home", 
        page_title="Welcome to My Website", 
        show_message=true, 
        message="Hello, world!", 
        items=[1, 2, 3, 4, 5]
    ) 
} 

// #[url(APP.reg_from(&[TEST_URL.clone(), LitUrl("large_file")]))]
// async fn large_file(context: Rc) -> HttpResponse {
//     let mut file = File::open("").unwrap(); 
//     let mut buffer = Vec::new();
//     file.read_to_end(&mut buffer).unwrap(); // This reads the file contents into `buffer`.
//     normal_response(StatusCode::OK, buffer) // Pass `buffer` instead of the usize result.
// } 

#[url(APP.reg_from(&[TEST_URL.clone(), LitUrl("get")]), allowed_methods=[GET])]  
async fn get_only() -> HttpResponse { 
    text_response("Get only")  
} 

#[url(APP.reg_from(&[TEST_URL.clone(), LitUrl("post")]), allowed_methods=[POST])]  
async fn post_only() -> HttpResponse { 
    text_response("Post only")  
} 

#[url(APP.reg_from(&[TEST_URL.clone(), AnyUrl()]))]  
async fn any(context: Rc) -> HttpResponse { 
    text_response(context.get_path(1)) 
} 

pub async fn flexible_access(mut rc: Rc) -> Rc { 
    rc.response = text_response("Flexible"); 
    rc 
} 
 

