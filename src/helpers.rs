pub mod helpers {

            // saves multiPart Image to disk
            use std::vec;
            use bytes::BufMut;
            use futures::{TryStreamExt, TryFutureExt};
            use warp::{
                multipart::{FormData},
                Rejection, Reply, Buf,
            };
            use anyhow::Result;
            
            pub async fn upload_file(form: FormData) -> Result<impl Reply, Rejection> {

                //get the parts of the formData
                 let parts: Result<Vec<(String, Vec<u8>)>, warp::Rejection> = form
                .and_then(|part| {
                    let name = part.name().to_string();
                    //let content_type  = part.content_type();
                    
                    let bytes = 
                    part
                    .stream()
                    .try_fold(Vec::new(), |mut vec, data| {
                        vec.put_slice(data.chunk());
                        async move { Ok(vec) }
                    });
                    bytes
                    .map_ok(move |vec| (name, vec))
                })
                .try_collect()
                .await
                .map_err(|e| {
                    panic!("multipart error: {:?}", e);
                 });


                let bytes_vec: Option<Vec<u8>> = match parts {
                    Ok(parts) => parts.first().map(|(_, vec)| vec.clone()),
                    Err(_) => None,
                };

               let bytes_vec =  match bytes_vec {
                    Some(vec) => vec,
                    None => {
                     vec![0]    
                    }
                };


                print!( "writting.....");

                let file_name = "new_one.png";
             

                tokio::fs::write(&file_name, the_vec).await.map_err(|e| {
                    eprint!("error writing file: {}", e);
                    warp::reject::reject()
                })?;

            Ok("success")
        }

        pub async fn handle_rejection(err: Rejection) -> std::result::Result<impl Reply, Infallible> {
            let (code, message) = if err.is_not_found() {
            (StatusCode::NOT_FOUND, "Not Found".to_string())
            } else if err.find::<warp::reject::PayloadTooLarge>().is_some() {
            (StatusCode::BAD_REQUEST, "Payload too large".to_string())
            } else {
            println!("unhandled error: {:?}", err);
                (StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error".to_string(),)
            };
            Ok(warp::reply::with_status(message, code))
        }
    
}