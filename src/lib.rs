use starberry::prelude::*;  

pub static APP: SApp = Lazy::new(|| { 
    App::new() 
        .binding(String::from("127.0.0.1:1111"))
        .mode(RunMode::Development)
        .workers(4) 
        .max_body_size(1024 * 1024 * 10) 
        .max_header_size(1024 * 10) 
        .append_middleware::<MyMiddleWare1>() 
        .append_middleware::<MyMiddleWare2>() 
        .append_middleware::<MyMiddleWare3>() 
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
        text_response("Directly return").boxed_future()
    } else {
        next(req) 
    }
} 

#[lit_url(APP, "/")] 
async fn home_route(_: Rc) -> HttpResponse { 
    html_response("<h1>Home</h1>") 
} 

#[lit_url(APP, "/random/split/something")]
async fn random_route(_: Rc) -> HttpResponse {
    text_response("A random page") 
}  

#[lit_url(APP, "/directly_return")]
async fn directly_return(_: Rc) -> HttpResponse {
    text_response("A random page") 
}  

#[lit_url(APP, "/print_sequence")]
async fn print_sequence(_: Rc) -> HttpResponse {
    println!("Processing request"); 
    text_response("See console") 
}  

#[lit_url(APP, "random")]
async fn anything_random(_: Rc) -> HttpResponse {
    text_response("A random page") 
}  

static TEST_URL: SUrl = Lazy::new(|| {
    APP.reg_from(&[LitUrl("test")]) 
}); 

#[url(TEST_URL.clone(), LitUrl("hello"))]
async fn hello(_: Rc) -> HttpResponse { 
    text_response("Hello")  
} 

#[url(TEST_URL.clone(), LitUrl("json_old"))]
async fn json_test(_: Rc) -> HttpResponse { 
    let a = 2; 
    let body = object!({number: a, string: "Hello", array: [1, 2, 3]}); 
    json_response(body)
} 

#[url(TEST_URL.clone(), LitUrl("json"))]
async fn json_new_test(_: Rc) -> HttpResponse { 
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

#[url(TEST_URL.clone(), LitUrl("async_test"))] 
async fn async_test(_: Rc) -> HttpResponse {
    sleep(Duration::from_secs(1));
    println!("1");
    sleep(Duration::from_secs(1)); 
    println!("2");
    sleep(Duration::from_secs(1));
    println!("3");
    text_response("Async Test Page") 
} 

#[url(TEST_URL.clone(), RegUrl("async_test2"))]  
async fn async_test2(_: Rc) -> HttpResponse {
    sleep(Duration::from_secs(1));
    println!("1");
    sleep(Duration::from_secs(1));
    println!("2");
    sleep(Duration::from_secs(1));
    println!("3");
    text_response("Async Test Page") 
} 

#[url(TEST_URL.clone(), RegUrl("[0-9]+"))]  
// #[set_header_size(max_size: 2048, max_line_size: 1024, max_lines: 200)] 
async fn testa(_: Rc) -> HttpResponse { 
    text_response("Number page") 
} 

#[url(TEST_URL.clone(), LitUrl("form_url_coded"))]  
async fn test_form(context: Rc) -> HttpResponse { 
    println!("Request to this dir"); 
    if context.method() == POST { 
        match context.form() { 
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

#[url(TEST_URL.clone(), LitUrl("form"))]  
async fn test_file(context: Rc) -> HttpResponse { 
    println!("Request to this dir"); 
    if context.method() == POST { 
        return text_response(format!("{:?}", context.files_or_default())); 
    } 
    plain_template_response("form.html") 
} 

#[url(TEST_URL.clone(), LitUrl("cookie"))]  
async fn test_cookie(mut context: Rc) -> HttpResponse { 
    if context.method() == POST { 
        match context.form() { 
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

#[url(TEST_URL.clone(), LitUrl("temp"))]  
async fn test_template(_: Rc) -> HttpResponse { 
    akari_render!(
        "home.html", 
        title="My Website - Home", 
        page_title="Welcome to My Website", 
        show_message=true, 
        message="Hello, world!", 
        items=[1, 2, 3, 4, 5]
    ) 
} 

#[url(TEST_URL.clone(), AnyUrl())]  
async fn any(mut context: Rc) -> HttpResponse { 
    text_response(context.get_path(1)) 
} 

pub async fn flexible_access(_: Rc) -> HttpResponse { 
    text_response("Flexible") 
} 
 

