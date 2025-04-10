# Starberry Web Framework - Example Project Documentation 

## 0. Basic Project Layout 

Your project structure should be 

```
crate
├── src
│   ├── main.rs                 # Start run App 
│   └── lib.rs                  # App and URL defination goes here 
└── templates                   # Templates goes there 
    ├── base.html                       
    ├── form.html
    └── home.html
```

If you didnot install starberry,  install it by 

`cargo install starberry` 

## 1. Main Application 

This file defines the core application structure and routes for the Starberry web application.

### Application Setup

```rust:12:17:src/main.rs
pub static APP: SApp = Lazy::new(|| { 
    App::new()
        .binding(String::from("127.0.0.1:1111"))
        .mode(RunMode::Development)
        .workers(4) 
        .max_body_size(1024 * 1024 * 10) 
        .max_header_size(1024 * 10) 
        .append_middleware::<MyMiddleWare1>() // Appending the middleware to the last in the middleware chain 
        .append_middleware::<MyMiddleWare2>() 
        .append_middleware::<MyMiddleWare3>() 
        .build() 
}); 
```

The application is configured to:
- Listen on `127.0.0.1:1111`
- Run in development mode
- Use 4 worker threads
- Accept request bodies up to 10MB
- Accept headers up to 10KB
- Configure a middleware chain with `MyMiddleWare1`, `MyMiddleWare2`, and `MyMiddleWare3` that process requests in sequence 

### Routes and Handlers

#### Root Route

```rust:19:21:src/main.rs
#[url(APP.lit_url("/"))] 
async fn home_route(mut req: Rc) -> HttpResponse { 
    html_response("<h1>Home</h1>") 
}  
```

A simple home page route that returns an HTML response. 

Note: The function registered into a Url will automatically get the input of `&mut req: Rc`, you do not need to explicitly write in the arguments. 

You can give a name of request variable by changing the function like this: 

`async fn home_route(my_custom_req_name: Rc) -> HttpResponse ` 

You must explicitly say your return type as HttpResponse or Rc 

#### Random Routes

```rust:23:31:src/main.rs
#[url(APP.lit_url("/random/split/something"))]
async fn random_route(mut req: Rc) -> Rc {
    req.response = text_response("A random page"); 
    req 
}  

#[url(APP.lit_url("/directly_return"))]
async fn directly_return() -> HttpResponse {
    text_response("A random page") 
}  
```

Two different routes showing how URLs can be registered.

#### Test URL Group

```rust:33:35:src/main.rs
static TEST_URL: SPattern = Lazy::new(|| {LitUrl("test")}); 
```

Creates a URL group for organizing related routes under `/test`.

#### JSON Response Examples

```rust:42:55:src/main.rs
#[url(APP.reg_from(&[TEST_URL.clone(), LitUrl("json_old")]))]
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
```

Two ways to generate JSON responses:
1. Using `object!` macro with `json_response`. You may first generate a object then pass it to HttpResponse 
2. Using the more concise `akari_json!` macro, it will directly return this JsonResponse to the user 

#### Async Function Examples

```rust:57:72:src/main.rs
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

#[url(TEST_URL.clone(), RegUrl("async_test2"))]  
async fn async_test2() -> HttpResponse {
    // Similar to async_test
    // ...
}
```

Demonstration of async processing with sleep operations.

#### Form Handling

```rust:83:95:src/main.rs
#[url(APP.reg_from(&[TEST_URL.clone(), LitUrl("form_url_coded")]))]  
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
```

Handles form submissions:
- Checks if request method is POST
- Extracts form data using `request.form()`
- Returns the form template for GET requests

#### File Upload Handling

```rust:97:110:src/main.rs
#[url(TEST_URL.clone(), LitUrl("form"))]  
async fn test_file(request: Rc) -> HttpResponse { 
    println!("Request to this dir"); 
    if request.method() == POST { 
        match request.files().await { 
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
```

Handles file uploads:
- Uses `request.files()` to extract uploaded files 
- Displays file information when POST request contains files

#### Template Rendering

```rust:112:122:src/main.rs
#[url(TEST_URL.clone(), LitUrl("temp"))]  
async fn test_template() -> HttpResponse { 
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
```

Demonstrates template rendering with the Akari template engine:
- Uses `akari_render!` macro to render the "home.html" template
- Passes multiple variables to the template

#### Dynamic Route Registration

```rust:4:7:src/main.rs
let furl = APP.clone().reg_from(&[LitUrl("flexible"), LitUrl("url"), LitUrl("may_be_changed")]); 
furl.set_method(Arc::new(flexible_access)); 
```

Shows how to programmatically register a route at runtime. 

### Running the application 

```rust 
APP.clone().run().await;  
``` 

#### Middleware Defination 

# Middleware Functions

This module contains custom middleware functions for request processing in the web application framework.

## Available Middleware

### MyMiddleWare1 

```rust
#[middleware]
pub async fn MyMiddleWare1(){ 
    println!("Middleware: Received request for {}, start processing", req.path()); 
    next(req)  
}  
```

A simple logging middleware that prints when a request is received and begins processing. 

This middleware logs the path of the incoming request before passing it to the next handler.

### MyMiddleWare2 

```rust
#[middleware]
pub async fn MyMiddleWare2(){ 
    let path = req.path().to_owned(); 
    let a = next(req).await; 
    println!("Middleware: Received request for {}, end processing", path); // You cannot access to req here 
    a.boxed_future() 
}   
```

An "around" middleware that executes code both before and after the request is processed. 

It captures the path early because the request object cannot be accessed after passing control 

to the next middleware in the chain. After the next middleware completes, it logs the completion 

of request processing.

### MyMiddleWare3

```rust
#[middleware]
pub async fn MyMiddleWare3(){ 
    if req.path() == "/directly_return" { 
        req.response = text_response("Directly return"); 
        req.boxed_future() 
    } else {
        next(req) 
    } 
}  
```  

A conditional middleware that demonstrates path-based routing control. 

- If the request path equals "/directly_return", it immediately returns a text response without
    passing the request to subsequent middleware
- For all other paths, it forwards the request to the next middleware in the chain

## Usage Notes

- Middleware executes in the order they are applied to the application. Refer to the top to see how the middleware chain is built 

- Request objects cannot be accessed after being passed to `next(req)` as demonstrated in MyMiddleWare2

- Middleware can short-circuit the request processing by returning a response instead of calling `next(req)` 

## 2. Templates

### Base Template (`base.html`)

A reusable layout template with block sections that can be overridden by child templates.

```html
<!DOCTYPE html>
<html>
<head>
    <title>-[ title ]-</title>
    -[ block head ]-
    <!-- Default head content -->
    -[ endblock ]-
</head>
<body>
    <header>
        -[ block header ]-
        <h1>Default Site Header</h1>
        -[ endblock ]-
    </header>
    
    <main>
        -[ block content ]-
        <p>Default content - override this</p>
        -[ endblock ]-
    </main>
    
    <footer>
        -[ block footer ]-
        <p>&copy; 2025 Template Engine</p>
        -[ endblock ]-
    </footer>
</body>
</html>
```

Key features:
- Defines the overall HTML structure
- Uses `-[ block name ]-` syntax to define sections that can be overridden
- Variable substitution with `-[ variable_name ]-` syntax

### Home Template (`home.html`)

Extends the base template and customizes specific sections.

```html
-[ template "base.html" ]-

-[ block head ]-
<link rel="stylesheet" href="style.css">
<meta name="description" content="My awesome page">
-[ endblock ]-

-[ block header ]-
<h1>-[ page_title ]-</h1>
<nav>
    <ul>
        <li><a href="/">Home</a></li>
        <li><a href="/about">About</a></li>
        <li><a href="/contact">Contact</a></li>
    </ul>
</nav>
-[ endblock ]-

-[ block content ]-
<div class="container">
    <h2>Welcome to our website</h2>
    
    -[ if show_message ]-
        <div class="message">-[ message ]-</div>
    -[ endif ]-
    
    <ul class="items">
        -[ for item items ]-
            <li class="item">-[ item ]-</li>
        -[ endfor ]-
    </ul>
</div>
-[ endblock ]-
```

Key features:
- Inherits from base.html with `-[ template "base.html" ]-`
- Overrides blocks: head, header, content
- Demonstrates conditionals with `-[ if condition ]-`
- Shows loops with `-[ for variable collection ]-`
- Uses variables passed from the `akari_render!` macro

### Form Template (`form.html`)

A standalone template with form examples.

```html
<!DOCTYPE html> 
<html lang="en"> 
    <head> 
        <meta charset="UTF-8"> 
        <meta name="viewport" content="width=device-width, initial-scale=1.0"> 
        <title>Form</title> 
    </head>
    <body> 
        <h1>Form</h1> 
        <form action="" method="post"> 
            <label for="name">Name:</label> 
            <input type="text" id="name" name="name"><br><br> 
            <label for="email">Email:</label> 
            <input type="email" id="email" name="email"><br><br> 
            <input type="submit" value="Submit"> 
        </form>

        <h1>Form</h1> 
        <form action="" method="post" enctype="multipart/form-data"> 
            <label for="name">Name:</label> 
            <input type="text" id="name" name="name"><br><br> 
            <label for="email">Email:</label> 
            <input type="email" id="email" name="email"><br><br> 
            <label for="file">Upload file:</label> 
            <input type="file" id="file" name="file" multiple><br><br> 
            <input type="submit" value="Submit"> 
        </form>
    </body> 
</html>
```

Contains two form examples:
1. A basic form for regular data submission
2. A form with file upload capability using `enctype="multipart/form-data"`

## How to Use This Example

1. **Start the server**
   ```bash
   starberry run
   ```
   The server will listen on http://127.0.0.1:1111

2. **Access available routes**
   - Home page: http://127.0.0.1:1111/
   - JSON examples: 
     - http://127.0.0.1:1111/test/json
     - http://127.0.0.1:1111/test/json_old
   - Template example: http://127.0.0.1:1111/test/temp
   - Form examples:
     - http://127.0.0.1:1111/test/form (file upload)
     - http://127.0.0.1:1111/test/form_url_coded (regular form)

3. **Template usage**
   - To create a new template, add a file to the `templates` directory
   - Extend base.html for consistent layout
   - Use the `akari_render!` macro to render templates with variables

4. **Adding new routes**
   - Use the `#[lit_url(APP, "/path")]` attribute for standalone routes
   - Create URL groups with `APP.reg_from(&[LitUrl("group")])`
   - Add child routes with `#[url(PARENT_URL.clone(), LitUrl("child"))]`
 
