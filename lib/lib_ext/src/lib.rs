extern crate rocket;

use rocket::{get, routes};

#[no_mangle]
extern "C" fn sub(a: i32, b: i32) -> i32 {
    a - b
}

#[no_mangle]
extern "C" fn start_server() {
    std::thread::spawn(move || {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        rt.block_on(async {
            rocket::build()
                .mount("/", routes![index])
                .launch()
                .await
                .unwrap();
        })
    });
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}
