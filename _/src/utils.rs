use std::env;

use actix_web::{HttpRequest,HttpResponse};
use actix_web::http::{header, StatusCode};

pub fn get_client_ip(req: &HttpRequest) -> String
{
	match req.peer_addr()
	{
		Some(val)=>format!("{}",val),
		None=>"Unknown".to_string(),
	}
}

pub fn is_auth(req: &HttpRequest) -> bool
{
	let key:String=match env::var("RQUE_SECRETKEY") 
	{
		Ok(env_var)=>env_var,
		Err(_)=>String::new()
	};
	let result:bool={
		if key.as_str()==""
		{
			return true;
		};
		let the_headers=req.headers();
		if !the_headers.contains_key(header::AUTHORIZATION)
		{
			return false;
		};
		let the_value=the_headers.get(header::AUTHORIZATION).unwrap();
		match the_value.to_str()
		{
			Err(_)=>false,
			Ok(the_value_str)=>{
				if the_value_str.starts_with("Bearer ")
				{
					if key==&the_value_str[7..] { true } else { false }
				}
				else { false }
			}
		}
	};
	if !result
	{
		let msg:String=match req.peer_addr()
		{
			Some(val)=>format!("\n- {} Attempted to access this server",val),
			None=>String::from("\n- Someone attempted to access this server, watch out"),
		};
		println!("{}",msg);
	};
	result
}

pub fn json_res(sc: u16,payload: serde_json::Value) -> HttpResponse
{
	HttpResponse::Ok()
	.status(StatusCode::from_u16(sc).unwrap())
	.json( payload )
}
