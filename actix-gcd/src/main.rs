use actix_web::{get, post,web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
struct GcdParameters {
    n: u64,
    m: u64,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server = HttpServer::new(|| {
        App::new()
            .service(get_index)
            .service(post_gcd)
    });

    println!("Serving on http://localhost:3000");
    server
        .bind("127.0.0.1:3000")?
        .run()
        .await
    
    
}

#[get("/")]
async fn get_index() -> impl Responder {
    HttpResponse::Ok()
            .content_type("text/html")
            .body(
                r#"
                <!DOCTYPE html>
                <html lang="en">
                    <title> GCD Calculator</title>
                    <form action="/post_gcd" method="post">
                    <input type="text" name="n"/>
                    <input type="text" name="m"/>
                    <button type="submit"> Compute GCD</button>
                    </form>
                "#,
            )
}

#[post("/post_gcd")]
async fn post_gcd(form: web::Form<GcdParameters>) -> impl Responder {
    if form.n == 0 || form.m == 0 {
        return HttpResponse::BadRequest()
                .content_type("text/html")
                .body("Computing the GCD with zero is boring");
    }

        println!("The greatest common divisor of the numbers {} and {} \
        is <b>{}</b>\n", 
        form.n, form.m, gcd(form.n, form.m));

    let response =
        format!(
            r#" <!DOCTYPE html>
                <html lang="en">
        The greatest common divisor of the numbers {} and {} \
                is <b>{}</b>\n"
                "#, 
                form.n, form.m, gcd(form.n, form.m));
    
    HttpResponse::Ok()
        .content_type("text/html")
        .body(response)
}

fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}





































