use std::collections::HashMap;
use std::env;
use std::sync::Mutex;
use actix_web::{get, post, delete, web, App, HttpServer, HttpResponse};
use actix_web::http::StatusCode;
use serde::Deserialize;
use serde_json::json;

static RQUE_DEFAULT_PORT:u16=8080;

// Group struct

struct Group
{
	data: Vec<Vec<String>>,
}

impl Group
{
	fn new() -> Group
	{
		Group{ data: Vec::new() }
	}

	fn get_size(&self) -> usize
	{
		self.data.len()
	}

	fn is_empty(&self) -> bool
	{
		let size=self.get_size();
		if size==0 { true } else { false }
	}

	fn index_exists(&self,index:usize) -> bool
	{
		let size=self.get_size();
		if index>size || size==0 || size==index { false } else { true }
	}

	fn has_head(&self,head: &String) -> bool
	{
		if self.is_empty()
		{
			return false;
		};
		let mut has_it=false;
		for elem in &self.data
		{
			let elem_head=e.first().unwrap();
			if elem_head==head
			{
				has_it=true;
				break;
			};
		};
		has_it
	}

	fn add(&mut self,value: Vec<String>) -> bool
	{
		let val_head=value.first().unwrap()
		if self.has_head(val_head)
		{
			return false;
		};
		self.data.push(value);
		true
	}

	fn get(&self,index: usize) -> Vec<String>
	{
		if self.index_exists(index) { self.data[index].clone() } else { Vec::new() }
	}

	fn kick(&mut self,index: usize) -> Vec<String>
	{
		if self.index_exists(index) { self.data.remove(index) } else { Vec::new() }
	}

	// NOTE: Comparison is done by checking the index 0 of the element, AKA: the head
	/*
	fn if_exists(&self, &element) -> bool
	{
		if self.is_empty()
		{
			return false;
		};
	}*/
}

// Main Data struct

struct Storage
{
	quecol: HashMap<String,Group>,
}

impl Storage
{
	fn get_size(&self) -> usize
	{
		self.quecol.len()
	}

	fn is_empty(&self) -> bool
	{
		let size:u16=self.get_size() as u16;
		if size==0 { true } else { false }
	}
}

// Application Data in a Mutex

struct TheAppState
{
	counter: Mutex<Storage>
}

// JSON requests

#[derive(Deserialize)]
struct POST_BringElem
{
	name:String,
	elem:Vec<String>,
}

#[derive(Deserialize)]
struct POST_BringIndex
{
	name:String,
	index:usize,
}

// HTTP Handlers

#[get("/")]
async fn get_status() -> HttpResponse
{
	HttpResponse::Ok()
	.status(StatusCode::from_u16(200).unwrap())
	.json( json!({}) )
}

#[get("/que")]
async fn get_names(app_data: web::Data<TheAppState>) -> HttpResponse
{
	let mut names: Vec<String>=Vec::new();
	let status_code:u16={
		let counter=app_data.counter.lock().unwrap();
		if counter.is_empty()
		{
			404
		}
		else
		{
			for key in counter.quecol.keys()
			{
				names.push(key.to_string());
			};
			200
		}
	};

	HttpResponse::Ok()
	.status(StatusCode::from_u16(status_code).unwrap())
	.json(
		if status_code==200
		{
			json!({ "result":names })
		}
		else
		{
			json!({})
		}
	)
}

#[get("/que/{name}")]
async fn get_queue(name: web::Path<String>,app_data: web::Data<TheAppState>) -> HttpResponse
{
	let mut result: Vec<Vec<String>>=Vec::new();
	let status_code:u16={
		let counter=app_data.counter.lock().unwrap();
		if counter.is_empty()
		{
			404
		}
		else
		{
			match counter.quecol.get(&name.into_inner())
			{
				Some(queue_found)=>
				{
					for elem in &queue_found.data
					{
						result.push(elem.to_vec());
					};
					200
				},
				None=>404,
			}
		}
	};

	HttpResponse::Ok()
	.status(StatusCode::from_u16(status_code).unwrap())
	.json( if status_code==200 { json!({ "result":result }) } else { json!({}) } )
}

#[get("/que/{name}/{index}")]
async fn get_index(from_path: web::Path<(String,usize)>,app_data: web::Data<TheAppState>) -> HttpResponse
{
	let mut element:Vec<String>=Vec::new();
	let (name,index)=from_path.into_inner();
	let counter=app_data.counter.lock().unwrap();
	let status_code:u16=match counter.quecol.get(&name)
	{
		Some(queue_found) => {
			if queue_found.index_exists(index)
			{
				for e in &queue_found.get(index)
				{
					element.push(e.to_string());
				};
				200
			}
			else
			{
				404
			}
		},
		None=>404,
	};
	HttpResponse::Ok()
	.status(StatusCode::from_u16(status_code).unwrap())
	.json( if status_code==200 { json!({ "element":element }) } else { json!({}) } )
}

#[post("/add/sin")]
async fn post_queue_add(from_post: web::Json<POST_BringElem>,app_data: web::Data<TheAppState>) -> HttpResponse
{
	let status_code:u16={ if from_post.elem.len()==0 {403} else {200} };
	if status_code==200
	{
		let new_name=from_post.name.clone();
		let new_elem=from_post.elem.clone();
		let mut counter=app_data.counter.lock().unwrap();
		match counter.quecol.get_mut(&new_name)
		{
			Some(fq) => {
				if fq.add(new_elem)
				{
					println!("\n- Added to existing queue\n  Name: {}\n  New: {:?}",&new_name,&new_elem);
				}
				else
				{
					status_code==403;
				};
			},
			None => {
				let mut vec_master:Vec<Vec<String>>=Vec::new();
				vec_master.push(new_elem);
				println!("\n- Created a new queue\n  Name: {}\n  Content: {:?}",&new_name,&vec_master);
				counter.quecol.insert(new_name, Group { data:vec_master });
			},
		};
	};
	HttpResponse::Ok()
	.status(StatusCode::from_u16(status_code).unwrap())
	.json(json!({ "status":status_code }))
}

#[delete("/all")]
async fn delete_all(app_data: web::Data<TheAppState>) -> HttpResponse
{
	let mut counter=app_data.counter.lock().unwrap();
	let status_code:u16={ if counter.is_empty() { 400 } else { counter.quecol.clear();200 } };
	HttpResponse::Ok()
	.status(StatusCode::from_u16(status_code).unwrap())
	.json(json!({ "status":status_code }))
}

#[delete("/sin/{name}")]
async fn delete_queue(from_path: web::Path<String>,app_data: web::Data<TheAppState>) -> HttpResponse
{
	let mut counter=app_data.counter.lock().unwrap();
	let mut status_code:u16={ if counter.is_empty() { 404 } else { 200 } };
	let name=from_path.into_inner();
	if status_code==200
	{
		if !counter.quecol.contains_key(&name)
		{
			status_code=404;
		};
	};
	if status_code==200
	{
		let contents=counter.quecol.remove(&name).unwrap();
		println!("\n- Deleting this queue:\n  Name: {}\n  Contents: {:?}",name,contents.data);
	};
	HttpResponse::Ok()
	.status(StatusCode::from_u16(status_code).unwrap())
	.json(json!({ "status":status_code }))
}

#[delete("/sin/{name}/{index}")]
async fn delete_index(from_path: web::Path<(String,usize)>,app_data: web::Data<TheAppState>) -> HttpResponse
{
	let (name,index)=from_path.into_inner();
	let mut counter=app_data.counter.lock().unwrap();
	let mut status_code:u16={ if counter.is_empty() { 404 } else { 200 } };
	if status_code==200
	{
		if !counter.quecol.contains_key(&name)
		{
			status_code=404;
		};
	};
	if status_code==200
	{
		let queue=counter.quecol.get_mut(&name).unwrap();
		let dumped=queue.kick(index);
		if dumped.len()>0
		{
			println!("\n- Kicked out from a queue\n  Name: {}\n  Index: {}\nElement: {:?}",name,index,dumped);
		}
		else
		{
			status_code=404;
		};
	};
	HttpResponse::Ok()
	.status(StatusCode::from_u16(status_code).unwrap())
	.json(json!({ "status":status_code }))
}

// Application setup

fn get_port() -> u16
{
	println!("\n- Obtaining Port from args");
	let mut args: Vec<String> = env::args().collect();
	if args.len()==1
	{
		println!("  NOTE: Using the default port");
		RQUE_DEFAULT_PORT
	}
	else
	{
		let port_raw:String=args.remove(1);
		match port_raw.parse::<u16>()
		{
			Ok(num) => num,
			Err(_) => {
				println!("  WARN: The arg for the port provided is NaN. Using default port instead");
				RQUE_DEFAULT_PORT
			},
		}
	}
}

#[actix_web::main]
async fn main() -> std::io::Result<()>
{
	println!("[ rQUE ]");
	let port=get_port();
	println!("\nChosen port: {}\n",port);
	let persistent=web::Data::new(TheAppState{
		counter: Mutex::new( Storage{quecol: HashMap::new()} )
	});
	HttpServer::new(move ||
		App::new()
			.app_data(persistent.clone())
			.service(get_status)
			.service(get_names)
			.service(get_queue)
			.service(get_index)
			.service(post_queue_add)
			.service(delete_all)
			.service(delete_queue)
			.service(delete_index)
		)
		.bind(("127.0.0.1",port))?
		.run()
		.await
}
