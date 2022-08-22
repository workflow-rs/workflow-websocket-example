use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(not(target_arch = "wasm32"))] {
        
        use thiserror::Error;

        #[derive(Error, Debug)]
        enum Error {
            #[error("Error: {0:#?}")]
            IoError(#[from] std::io::Error),
        }
        
        #[async_std::main]
        async fn main() -> Result<(), Error> {
        
            tide::log::start();
            let tide_secret :&[u8] = &(0..64).map(|_| { rand::random::<u8>() }).collect::<Vec<u8>>();
            let mut app = tide::new();
            app.with(tide::log::LogMiddleware::new());
            app.with(tide::sessions::SessionMiddleware::new(
                tide::sessions::MemoryStore::new(),
                tide_secret
            ));
        
            app.at("/").serve_file("../wasm/index.html")?;
            app.at("/workflow").serve_dir("../wasm/workflow")?;
            app.listen("0.0.0.0:9191").await?;
        
            Ok(())
        }
    } else {
        fn main() -> std::result::Result<(),String> {
            panic!("wasm32 target is not supported");
        }
    }
}
