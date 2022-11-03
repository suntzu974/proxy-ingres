use crate::ecomrun::{ResponseStock,ResponseTarif,QueryStock,QueryTarif,QueryTransfert,ResponseAppro};
use actix_web::{post,error,web, Error, HttpResponse};
use crate::utils::utils::{Config};

use futures::{StreamExt};
const MAX_SIZE: usize = 524_288; // max payload size is 512k

#[post("/stockingres")]
pub async fn query(mut payload: web::Payload) ->  Result<HttpResponse, Error>  {

    let config_proxy = Config::from_env().unwrap();


    // payload is a stream of Bytes objects
    let mut body = web::BytesMut::new();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;
        // limit max size of in-memory payload
        if (body.len() + chunk.len()) > MAX_SIZE {
         return Err(error::ErrorBadRequest("overflow"));
        }
        body.extend_from_slice(&chunk); 
    }
    // body is loaded, now we can deserialize serde-json
    let obj = serde_json::from_slice::<QueryStock>(&body)?;
    let client = reqwest::Client::new();
    let res =  client.post(format!("{}://{}:{}/{}/{}","http",config_proxy.ip_tomcat,config_proxy.ip_tomcat_port,config_proxy.ip_tomcat_app,"stockingres"))
    .json(&obj)
    .send()
    .await.unwrap();
    let stocks:ResponseStock = res.json::<ResponseStock>().await.unwrap();
    Ok(HttpResponse::Ok().json(stocks)) // <- send response
}
#[post("/tarif")]
pub async fn query_tarif(mut payload: web::Payload) ->  Result<HttpResponse, Error>  {
    let config_proxy = Config::from_env().unwrap();

    // payload is a stream of Bytes objects
    let mut body = web::BytesMut::new();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;
        // limit max size of in-memory payload
        if (body.len() + chunk.len()) > MAX_SIZE {
         return Err(error::ErrorBadRequest("overflow"));
        }
        body.extend_from_slice(&chunk); 
    }
    // body is loaded, now we can deserialize serde-json
    let obj = serde_json::from_slice::<QueryTarif>(&body)?;
    let client = reqwest::Client::new();
    let res = client.post(format!("{}://{}:{}/{}/{}","http",config_proxy.ip_tomcat,config_proxy.ip_tomcat_port,config_proxy.ip_tomcat_app,"tarif"))
    .json(&obj)
    .send()
    .await.unwrap();

    let tarifs:ResponseTarif = res.json::<ResponseTarif>().await.unwrap();

    Ok(HttpResponse::Ok().json(tarifs)) // <- send response
}
#[post("/stocks")]
pub async fn query_by_gencod(mut payload: web::Payload) ->  Result<HttpResponse, Error>  {

    let config_proxy = Config::from_env().unwrap();

    // payload is a stream of Bytes objects
    let mut body = web::BytesMut::new();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;
        // limit max size of in-memory payload
        if (body.len() + chunk.len()) > MAX_SIZE {
         return Err(error::ErrorBadRequest("overflow"));
        }
        body.extend_from_slice(&chunk); 
    }
    // body is loaded, now we can deserialize serde-json
    let obj = serde_json::from_slice::<QueryStock>(&body)?;
    let client = reqwest::Client::new();
    let res =  client.post(format!("{}://{}:{}/{}/{}","http",config_proxy.ip_tomcat,config_proxy.ip_tomcat_port,config_proxy.ip_tomcat_app,"stocks"))
    .json(&obj)
    .send()
    .await.unwrap();

    let stocks:ResponseStock = res.json::<ResponseStock>().await.unwrap();
    Ok(HttpResponse::Ok().json(stocks)) // <- send response
}

#[post("/transfert")]
pub async fn build_appro(mut payload: web::Payload) ->  Result<HttpResponse, Error>  {
    let config_proxy = Config::from_env().unwrap();


    // payload is a stream of Bytes objects
    let mut body = web::BytesMut::new();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;
        // limit max size of in-memory payload
        if (body.len() + chunk.len()) > MAX_SIZE {
         return Err(error::ErrorBadRequest("overflow"));
        }
        body.extend_from_slice(&chunk); 
    }
    // body is loaded, now we can deserialize serde-json
    let obj = serde_json::from_slice::<QueryTransfert>(&body)?;
    let client = reqwest::Client::new();
    let res = client.post(format!("{}://{}:{}/{}/{}","http",config_proxy.ip_tomcat,config_proxy.ip_tomcat_port,config_proxy.ip_tomcat_app,"transfert"))
    .json(&obj)
    .send()
    .await.unwrap();
    let appro:ResponseAppro = res.json::<ResponseAppro>().await.unwrap();
    Ok(HttpResponse::Ok().json(appro)) // <- send response
}

#[post("/transfert-dev")]
pub async fn build_appro_dev(mut payload: web::Payload) ->  Result<HttpResponse, Error>  {
    let config_proxy = Config::from_env().unwrap();


    // payload is a stream of Bytes objects
    let mut body = web::BytesMut::new();
    while let Some(chunk) = payload.next().await {
        let chunk = chunk?;
        // limit max size of in-memory payload
        if (body.len() + chunk.len()) > MAX_SIZE {
         return Err(error::ErrorBadRequest("overflow"));
        }
        body.extend_from_slice(&chunk); 
    }
    // body is loaded, now we can deserialize serde-json
    let obj = serde_json::from_slice::<QueryTransfert>(&body)?;
    let client = reqwest::Client::new();
    let res = client.post(format!("{}://{}:{}/{}/{}","http",config_proxy.ip_tomcat,config_proxy.ip_tomcat_port,config_proxy.ip_tomcat_app,"transfert"))
    .json(&obj)
    .send()
    .await.unwrap();

    let appro:ResponseAppro = res.json::<ResponseAppro>().await.unwrap();
    Ok(HttpResponse::Ok().json(appro)) // <- send response
}


pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(query);
    cfg.service(query_tarif);
    cfg.service(query_by_gencod);
    cfg.service(build_appro);
    cfg.service(build_appro_dev);
}