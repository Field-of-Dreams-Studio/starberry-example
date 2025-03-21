use starberry::preload::*;  

pub static APP: SApp = Lazy::new(|| { 
    App::new()
        .binding(String::from("127.0.0.1:1111"))
        .mode(RunMode::Development)
        .workers(4) 
        .max_body_size(1024 * 1024 * 10) 
        .max_header_size(1024 * 10) 
        .build() 
}); 

#[lit_url(APP, "/")] 
async fn home_route(_: HttpRequest) -> HttpResponse { 
    html_response("<h1>Home</h1>") 
} 

#[lit_url(APP, "/random/split/something")]
async fn random_route(_: HttpRequest) -> HttpResponse {
    text_response("A random page") 
}  

#[lit_url(APP, "random")]
async fn anything_random(_: HttpRequest) -> HttpResponse {
    text_response("A random page") 
}  

static TEST_URL: SUrl = Lazy::new(|| {
    APP.reg_from(&[LitUrl("test")]) 
}); 

#[url(TEST_URL.clone(), LitUrl("hello"))]
async fn hello(_: HttpRequest) -> HttpResponse { 
    text_response("Hello")  
} 

#[url(TEST_URL.clone(), LitUrl("json_old"))]
async fn json_test(_: HttpRequest) -> HttpResponse { 
    let a = 2; 
    let body = object!({number: a, string: "Hello", array: [1, 2, 3]}); 
    json_response(body)
} 

#[url(TEST_URL.clone(), LitUrl("json"))]
async fn json_new_test(_: HttpRequest) -> HttpResponse { 
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
async fn async_test(_: HttpRequest) -> HttpResponse {
    sleep(Duration::from_secs(1));
    println!("1");
    sleep(Duration::from_secs(1)); 
    println!("2");
    sleep(Duration::from_secs(1));
    println!("3");
    text_response("Async Test Page") 
} 

#[url(TEST_URL.clone(), RegUrl("async_test2"))]  
async fn async_test2(_: HttpRequest) -> HttpResponse {
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
async fn testa(_: HttpRequest) -> HttpResponse { 
    text_response("Number page") 
} 

#[url(TEST_URL.clone(), LitUrl("form_url_coded"))]  
async fn test_form(request: HttpRequest) -> HttpResponse { 
    println!("Request to this dir"); 
    if *request.method() == POST { 
        match request.form() { 
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
async fn test_file(request: HttpRequest) -> HttpResponse { 
    println!("Request to this dir"); 
    if *request.method() == POST { 
        match request.files() { 
            Some(form) => { 
                return text_response(format!("{:#?}", form.get("file").unwrap().get_files().unwrap())); 
            } 
            None => { 
                return text_response("Error parsing form"); 
            }  
        }  
    } 
    plain_template_response("form.html") 
} 

#[url(TEST_URL.clone(), LitUrl("temp"))]  
async fn test_template(_: HttpRequest) -> HttpResponse { 
    let items = object!([1, 2, 3, 4, 5]); 
    akari_render!(
        "home.html", 
        title="My Website - Home", 
        page_title="Welcome to My Website", 
        show_message=true, 
        message="Hello, world!", 
        items=items
    ) 
} 

pub async fn flexible_access(_: HttpRequest) -> HttpResponse { 
    text_response("Flexible") 
} 
 
 