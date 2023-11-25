#[macro_use]
extern crate rocket;

use rocket::data::{Data, ToByteUnit};
use rocket::tokio::fs::File;
use std::path::Path;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[post("/trace", data = "<data>")]
async fn trace(data: Data<'_>) -> std::io::Result<()> {
    let path = Path::new("trace.r3");
    let mut file = File::create(path).await?;

    // Stream the data to the file
    data.open(2.mebibytes()).stream_to(&mut file).await?;

    Ok(())
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, trace])
}
