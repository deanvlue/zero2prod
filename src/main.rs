use zero2prod::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // Added ?
    // Will bubble up the error in case of error
    // or will await otherwise
    //run("0.0.0.0:0")?.await
    Ok(())
}
