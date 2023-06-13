mod data_storage;
mod globals;
mod utils;

use std::collections::HashMap;
use std::env;
use std::sync::Mutex;
use actix_web::{get, post, delete, web, App, HttpServer, HttpResponse, HttpRequest};
use actix_web::http::{header, StatusCode};
use serde::Deserialize;
use serde_json::json;

use crate::data_storage::Group;
use crate::data_storage::Storage;

use crate::globals::RQUE_DEFAULT_PORT;
use crate::globals::RQUE_MSG_DEF_PORT;
use crate::globals::RQUE_MSG_CUS_PORT;
use crate::globals::RQUE_ERROR_ZERO_GROUPS;
use crate::globals::RQUE_ERROR_GROUP_NOT_FOUND;
use crate::globals::RQUE_ERROR_GROUP_EMPTY;
use crate::globals::RQUE_ERROR_ITEM_NOT_FOUND;
use crate::globals::RQUE_ERROR_ITEM_NOT_VALID;
use crate::globals::RQUE_ERROR_SLICE;
use crate::globals::RQUE_INFO;
use crate::globals::RQUE_HELP;

use crate::utils::get_client_ip;
use crate::utils::is_auth;
use crate::utils::json_res;

// Application Data in a Mutex

struct TheAppState { holder: Mutex<Storage> }

// JSON schemas

#[derive(Deserialize)]
struct POST_AddOne
{
	name:String,
	item:Vec<String>,
}

#[derive(Deserialize)]
struct POST_AddMul
{
	name:String,
	list:Vec<Vec<String>>,
	details:bool,
}

#[derive(Deserialize)]
struct Configuration
{
	port:u16,
	password:String,
}

// HTTP Handlers

#[get("/")]
async fn get_status(req: HttpRequest) -> HttpResponse
{
	let sc:u16={ if is_auth(&req) { 200 } else { 401 } };
	json_res(sc, json!({ "status":sc }) )
}

#[get("/help")]
async fn show_help(req: HttpRequest) -> HttpResponse
{
	let valid={
		let iv=is_auth(&req);
		if iv { iv } else { let client_ip=get_client_ip(&req);client_ip.starts_with("127.0.0.1") }
	};
	HttpResponse::Ok()
	.status(StatusCode::from_u16( if valid { 200 } else { 401 } ).unwrap())
	.insert_header(("Content-Type", if valid { "text/html"} else { "text/plain" } ))
	.body( if valid { RQUE_HELP.to_string() } else { String::from("UNAUTHORIZED") } )
}

#[get("/all")]
async fn get_names(req: HttpRequest,app_data: web::Data<TheAppState>) -> HttpResponse
{
	if !is_auth(&req)
	{
		return json_res(401, json!({ "status":401 }));
	};
	let storage=app_data.holder.lock().unwrap();
	if storage.is_empty()
	{
		return json_res(404,json!({ "status":404,"msg":RQUE_ERROR_ZERO_GROUPS }));
	};
	let mut list_of_names: Vec<String>=Vec::new();
	for name in storage.quecol.keys()
	{
		list_of_names.push(name.to_string());
	};

	json_res(200,json!({ "status":200,"names":list_of_names }))
}

#[get("/r/{name}")]
async fn get_group(req: HttpRequest,from_path: web::Path<String>,app_data: web::Data<TheAppState>) -> HttpResponse
{
	if !is_auth(&req)
	{
		return json_res(401, json!({ "status":401 }));
	};
	let storage=app_data.holder.lock().unwrap();
	if storage.is_empty()
	{
		return json_res(403,json!({ "status":403,"msg":RQUE_ERROR_ZERO_GROUPS }));
	};

	let the_name=&from_path.into_inner();
	if !storage.quecol.contains_key(the_name)
	{
		return json_res(404,json!({ "status":404,"msg":RQUE_ERROR_GROUP_NOT_FOUND }));
	};

	let the_group=storage.quecol.get(the_name).unwrap();
	let mut list:Vec<Vec<String>>=Vec::new();
	let mut list_size:u32=0;

	let status_code:u16={
		if the_group.is_empty()
		{
			206
		}
		else
		{
			for item in the_group.data.iter()
			{
				list.push(item.to_vec());
				list_size=list_size+1;
			};
			200
		}
	};

	json_res(status_code,json!({ "status":status_code,"group_size":list_size,"group":list }))
}

#[get("/r/{name}/size")]
async fn get_group_size(req: HttpRequest,from_path: web::Path<String>,app_data: web::Data<TheAppState>) -> HttpResponse
{
	if !is_auth(&req)
	{
		return json_res(401, json!({ "status":401 }));
	};
	let storage=app_data.holder.lock().unwrap();
	if storage.is_empty()
	{
		return json_res(403,json!({ "status":403,"msg":RQUE_ERROR_ZERO_GROUPS }));
	};

	let the_name=&from_path.into_inner();
	if !storage.quecol.contains_key(the_name)
	{
		return json_res(404,json!({ "status":404,"msg":RQUE_ERROR_GROUP_NOT_FOUND }));
	};

	let the_group=storage.quecol.get(the_name).unwrap();
	let the_size=the_group.get_size() as u32;
	let status_code:u16={ if the_size==0 { 206 } else { 200 } };
	json_res(status_code,json!({ "status":status_code,"group_size":the_size }))
}

#[get("/r/{name}/s/{index}")]
async fn get_index(req: HttpRequest,from_path: web::Path<(String,usize)>,app_data: web::Data<TheAppState>) -> HttpResponse
{
	if !is_auth(&req)
	{
		return json_res(401, json!({ "status":401 }));
	};
	let storage=app_data.holder.lock().unwrap();
	if storage.is_empty()
	{
		return json_res(403,json!({ "status":403,"msg":RQUE_ERROR_ZERO_GROUPS }));
	};

	let (the_name,the_index)=from_path.into_inner();
	if !storage.quecol.contains_key(&the_name)
	{
		return json_res(403,json!({ "status":403,"msg":RQUE_ERROR_GROUP_NOT_FOUND }));
	};

	let the_group=storage.quecol.get(&the_name).unwrap();
	if the_group.is_empty()
	{
		return json_res(403,json!({ "status":403,"msg":RQUE_ERROR_GROUP_EMPTY }));
	};

	let the_item=the_group.get(the_index);
	if the_item.len()==0
	{
		json_res(404,json!({"status":404,"msg":RQUE_ERROR_ITEM_NOT_FOUND}))
	}
	else
	{
		json_res(200,json!({"status":200,"item":the_item}))
	}
}

#[get("/r/{name}/s/{index}/{qtty}")]
async fn get_range(req: HttpRequest,from_path: web::Path<(String,usize,usize)>,app_data: web::Data<TheAppState>) -> HttpResponse
{
	if !is_auth(&req)
	{
		return json_res(401, json!({ "status":401 }));
	};
	let mut storage=app_data.holder.lock().unwrap();
	if storage.is_empty()
	{
		return json_res(403,json!({ "status":403,"msg":RQUE_ERROR_ZERO_GROUPS }));
	};

	let (the_name,index,qtty)=from_path.into_inner();
	if !storage.quecol.contains_key(&the_name)
	{
		return json_res(403,json!({ "status":403,"msg":RQUE_ERROR_GROUP_NOT_FOUND }));
	};

	let the_group=storage.quecol.get_mut(&the_name).unwrap();
	if the_group.is_empty()
	{
		return json_res(403,json!({ "status":403,"msg":RQUE_ERROR_GROUP_EMPTY }));
	};

	let the_slice:Vec<Vec<String>>=the_group.get_range(index,qtty,false);
	if the_slice.len()==0
	{
		json_res(400,json!({ "status":400,"msg":RQUE_ERROR_SLICE }))
	}
	else
	{
		json_res(200,json!({ "status":200,"slice":the_slice }))
	}
}

#[post("/add-one")]
async fn post_group_addone(req: HttpRequest,from_post: web::Json<POST_AddOne>,app_data: web::Data<TheAppState>) -> HttpResponse
{
	if !is_auth(&req)
	{
		return json_res(401, json!({ "status":401 }));
	};
	if from_post.item.len()==0
	{
		return json_res(403, json!({"msg":RQUE_ERROR_ITEM_NOT_VALID}));
	};

	let the_name=&from_post.name;
	let the_item=&from_post.item;
	let mut storage=app_data.holder.lock().unwrap();

	let mut msg:&str="";
	let mut newgroup:bool=false;
	let status_code:u16=match storage.quecol.get_mut(the_name)
	{
		Some(fq)=>{
			if fq.add(the_item.to_vec())
			{
				println!("\n- Added a new item to an existing group\n  Name: {}\n  Item: {:?}",the_name,the_item);200
			}
			else
			{
				msg=RQUE_ERROR_ITEM_NOT_VALID;403
			}
		},
		None=>{
			let mut ng:Vec<Vec<String>>=Vec::new();
			ng.push(the_item.to_vec());
			println!("\n- Created a new group\n  Name: {}\n  Content: {:?}",the_name,&ng);
			storage.quecol.insert(the_name.to_string(), Group { data:ng });
			newgroup=true;
			200
		}
	};

	json_res(status_code,if status_code==200 { json!({ "status":200,"newgroup":newgroup }) } else { json!({ "status":status_code,"msg":msg }) })
}

#[post("/add-mul")]
async fn post_group_addmul(req: HttpRequest,from_post: web::Json<POST_AddMul>,app_data: web::Data<TheAppState>) -> HttpResponse
{
	if !is_auth(&req)
	{
		return json_res(401, json!({ "status":401 }));
	};
	if from_post.list.len()==0
	{
		return json_res(403,json!({"msg":"The provided list of items is empty"}));
	};

	let the_name=&from_post.name;
	let mut storage=app_data.holder.lock().unwrap();
	let newgroup:bool={
		if storage.quecol.contains_key(the_name) { false } else { storage.quecol.insert(the_name.to_string(),Group::new());true }
	};

	let mut msg:&str="";
	let mut res_arr:Vec<bool>=Vec::new();
	let status_code:u16={
		let the_group=storage.quecol.get_mut(the_name).unwrap();
		let the_list=&from_post.list;

		let mut added:usize=0;
		for item in the_list.iter()
		{
			if the_group.add(item.to_vec())
			{
				added=added+1;res_arr.push(true);
			}
			else
			{
				res_arr.push(false);
			};
		};

		if added>0
		{
			let ok:bool=added==res_arr.len();
			println!("\n- Added multiple items to a group\n  NewGroup?: {}\n  Name: {}\n  List: {:?}\n  Added/Total: {}/{} {:?}",newgroup,the_name,the_list,added,res_arr.len(),&res_arr);
			if ok { 200 } else { 206 }
		}
		else
		{
			if newgroup
			{
				println!("\n- A new group is empty after attempting to add multiple items\n  Name: {}",the_name);
			};
			msg="The provided items are all invalid";400
		}
	};

	json_res(status_code,
		match status_code
		{
			200=>json!({"status":status_code,"newgroup":newgroup}),
			206=>if from_post.details
			{
				let mut items_succ=0;
				let mut items_fail=0;
				for b in res_arr.iter()
				{
					if *b { items_succ=items_succ+1 } else { items_fail=items_fail+1 };
				};
				json!({"status":status_code,"newgroup":newgroup,"items_succ":items_succ,"items_fail":items_fail})

			} else { json!({"status":status_code,"newgroup":newgroup,"details":res_arr}) },

			_=>json!({"status":status_code,"msg":msg})
		}
	)
}

#[delete("/all")]
async fn delete_all(req: HttpRequest,app_data: web::Data<TheAppState>) -> HttpResponse
{
	if !is_auth(&req)
	{
		return json_res(401, json!({ "status":401 }));
	};
	let mut storage=app_data.holder.lock().unwrap();
	if storage.is_empty()
	{
		json_res(404,json!({ "status":404,"msg":RQUE_ERROR_ZERO_GROUPS }))
	}
	else
	{
		println!("\n- All groups have been deleted");
		storage.quecol.clear();
		json_res(200,json!({ "status":200 }))
	}
}

#[delete("/d/{name}")]
async fn delete_group(req: HttpRequest,from_path: web::Path<String>,app_data: web::Data<TheAppState>) -> HttpResponse
{
	if !is_auth(&req)
	{
		return json_res(401, json!({ "status":401 }));
	};

	let mut storage=app_data.holder.lock().unwrap();

	let mut msg:&str="";
	let mut status_code:u16={ if storage.is_empty() { msg=RQUE_ERROR_ZERO_GROUPS;403 } else { 200 } };

	let the_name=&from_path.into_inner();
	if status_code==200
	{
		if !storage.quecol.contains_key(the_name)
		{
			msg=RQUE_ERROR_GROUP_NOT_FOUND;status_code=404;
		};
	};
	if status_code==200
	{
		let mut the_group=storage.quecol.remove(the_name).unwrap();
		println!("\n- Deleting this group:\n  Name: {}\n  Items: {:?}",the_name,&the_group.data);
		the_group.data.clear();
	};

	json_res(status_code, if status_code==200 { json!({ "status":status_code }) } else { json!({ "status":status_code,"msg":msg }) } )
}

#[delete("/d/{name}/{index}")]
async fn delete_index(req: HttpRequest,from_path: web::Path<(String,usize)>,app_data: web::Data<TheAppState>) -> HttpResponse
{
	if !is_auth(&req)
	{
		return json_res(401, json!({ "status":401 }));
	};
	let mut storage=app_data.holder.lock().unwrap();

	let mut msg:&str="";
	let mut status_code:u16={ if storage.is_empty() { msg=RQUE_ERROR_ZERO_GROUPS;403 } else { 200 } };

	let (the_name,the_index)=from_path.into_inner();

	if status_code==200
	{
		if !storage.quecol.contains_key(&the_name)
		{
			status_code=404;
		};
	};
	if status_code==200
	{
		let the_group=storage.quecol.get_mut(&the_name).unwrap();
		if the_group.is_empty()
		{
			msg=RQUE_ERROR_GROUP_EMPTY;status_code=403;
		}
		else
		{
			let item=the_group.kick(the_index);
			if the_group.is_empty()
			{
				storage.quecol.remove(&the_name).unwrap();
				println!("\n- Deleted empty group\n  Name: {}",&the_name);
			};
			if item.len()==0
			{
				msg=RQUE_ERROR_ITEM_NOT_FOUND;status_code=404;
			}
			else
			{
				println!("\n- Deleted an item from a group\n  Name: {}\n  Index: {}\n  Item: {:?}",&the_name,the_index,&item);
				return json_res(200,json!({"status":200,"item":item}));
			};
		};
	};

	json_res(status_code,json!({ "status":status_code,"msg":msg }))
}

#[delete("/d/{name}/{index}/{qtty}")]
async fn delete_range(req: HttpRequest,from_path: web::Path<(String,usize,usize)>,app_data: web::Data<TheAppState>) -> HttpResponse
{
	if !is_auth(&req)
	{
		return json_res(401, json!({ "status":401 }));
	};
	let mut storage=app_data.holder.lock().unwrap();
	if storage.is_empty()
	{
		return json_res(403,json!({ "status":403,"msg":RQUE_ERROR_ZERO_GROUPS }));
	};
	let (the_name,index,qtty)=from_path.into_inner();
	if !storage.quecol.contains_key(&the_name)
	{
		return json_res(403,json!({ "status":403,"msg":RQUE_ERROR_GROUP_NOT_FOUND }));
	};
	let the_group=storage.quecol.get_mut(&the_name).unwrap();
	if the_group.is_empty()
	{
		return json_res(403,json!({ "status":403,"msg":RQUE_ERROR_GROUP_EMPTY }));
	};
	let the_slice:Vec<Vec<String>>=the_group.get_range(index,qtty,true);
	if the_group.is_empty()
	{
		storage.quecol.remove(&the_name).unwrap();
		println!("\n- Deleted empty group\n  Name: {}",&the_name);
	};
	if the_slice.len()==0
	{
		json_res(400,json!({ "status":400,"msg":RQUE_ERROR_SLICE }))
	}
	else
	{
		println!("\n- Deleted multiple items from a group\n  Name: {}\n  List: {:?}",&the_name,&the_slice);
		json_res(200,json!({ "status":200,"slice":the_slice }))
	}
}

// Application setup

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

	let pdata=web::Data::new(TheAppState{
		//holder: Mutex::new( Storage{ quecol: HashMap::new() } )
		holder: Mutex::new( Storage::new() )
	});

	println!("Need help? Docs are provided by this server at: http://127.0.0.1{}/help",if port==80 { String::new() } else { format!(":{}",port) });

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
