mod data_storage;
mod globals;
mod routes;
mod utils;

use std::env;
use std::sync::Mutex;
use actix_web::{web,App,HttpServer};

use crate::data_storage::Storage;
use crate::globals::RQUE_DEFAULT_PORT;
use crate::globals::RQUE_INFO;
use crate::globals::RQUE_MSG_CUS_PORT;
use crate::globals::RQUE_MSG_DEF_PORT;
use crate::routes::get_status;
use crate::routes::show_help;
use crate::routes::get_names;
use crate::routes::get_group;
use crate::routes::get_group_size;
use crate::routes::get_index;
use crate::routes::get_range;
use crate::routes::post_group_addone;
use crate::routes::post_group_addmul;
use crate::routes::delete_all;
use crate::routes::delete_group;
use crate::routes::delete_index;
use crate::routes::delete_range;

pub struct TheAppState
{
	pub holder: Mutex<Storage>
}

fn parse_port(raw_arg: String) -> (u16,bool)
{
	match raw_arg.parse::<u16>()
	{
		Ok(num) => (num,true),
		Err(_) => (RQUE_DEFAULT_PORT,false),
	}
}

#[actix_web::main]
async fn main() -> std::io::Result<()>
{
	println!("\n[ rQUE ]\n\n{}",RQUE_INFO);

	let port:u16={
		println!("\n- From config: Obtaining the port");
		let (from_arg,arg_ok):(u16,bool)={
			let mut args: Vec<String>=env::args().collect();
			if args.len()>1
			{
				let from_arg_raw=args.remove(1);parse_port(from_arg_raw)
			}
			else { (RQUE_DEFAULT_PORT,false) }
		};
		if arg_ok
		{
			println!("  {}: {}",RQUE_MSG_CUS_PORT,from_arg);from_arg
		}
		else
		{
			let (msg,from_env):(String,u16)=match env::var("RQUE_CUSTOMPORT")
			{
				Err(_)=>( String::from(RQUE_MSG_DEF_PORT),from_arg ),
				Ok(from_env_raw)=>{
					let (the_port,env_ok):(u16,bool)=parse_port(from_env_raw);
					if env_ok
					{ ( format!("{}: {}",RQUE_MSG_CUS_PORT,the_port) , the_port ) }
					else
					{ ( String::from(RQUE_MSG_DEF_PORT) , the_port ) }
				}
			};
			println!("  {}",msg);from_env
		}
	};

	println!("\n- {}",
		match env::var("RQUE_SECRETKEY")
		{
			Ok(_)=>"Secret key env var detected!",
			Err(_)=>"WARNING: There is no secret key",
		}
	);

	let pdata=web::Data::new( TheAppState{ holder: Mutex::new( Storage::new() ) } );

	println!("\nNeed any help? This is the documentation: http://127.0.0.1{}/help",if port==80 { String::new() } else { format!(":{}",port) });

	HttpServer::new(move ||
		App::new()
			.app_data(pdata.clone())
			.service(show_help)
			.service(get_status)
			.service(get_names)
			.service(get_group)
			.service(get_group_size)
			.service(get_index)
			.service(get_range)
			.service(post_group_addone)
			.service(post_group_addmul)
			.service(delete_all)
			.service(delete_group)
			.service(delete_index)
			.service(delete_range)
		)
		.bind(("127.0.0.1",port))?
		.run()
		.await
}
