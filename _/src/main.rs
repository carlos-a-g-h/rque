use std::collections::HashMap;
use std::env;
use std::sync::Mutex;
use actix_web::{get, post, delete, web, App, HttpServer, HttpResponse};
use actix_web::http::StatusCode;
use serde::Deserialize;
use serde_json::json;

static RQUE_DEFAULT_PORT:u16=8080;
static RQUE_HTML_HELP:&str="
<!DOCTYPE html>
<html lang=\"en\">
	<meta charset=\"UTF-8\">
	<meta name=\"viewport\" content=\"width=device-width,initial-scale=1\">
	<head>
		<title>rQUE quick help</title>
	</head>
	<body>
		<h1>rQUE</h1>
		<h2>How data is stored</h2>
		<p>In rQUE, data is stored in a large hashmap, where each key is a group name and each value is the content of the group</p>
		<h3><code>{<br>'group 1' : [<br>['thing1'] , ['thing2','bonus']<br>] ,<br>'group2' : [<br>['thing1']<br>] ,<br>'another group' : [<br> ['headname1','data'] , ['headname2','data'], ['headname3','data','more data'] <br>] <br>}</code></h3>
		<p>Groups are lists and each group name is unique<br>Each element inside a group is a list with the first index being the head<br>2 or more elements in the same group cannot have the same head, and this is checked automatically by rQUE before adding new elements to a group</p>
		<h2>API usage</h2>
		<p>GET /help<br>Desc.: This help</p>
		<p>GET /<br>Desc.: Always gives 200<br>Res.: (200): <code>{}</code></p>
		<p>GET /all<br>Desc.: Recovers a list of existing group names<br>Res. (JSON, 200): <code>{'result':['name1','name2',...,'nameN']}</code><br>Res. (JSON, 4xx): <code>{}</code></p>
		<p>GET /sel/{name}<br>Desc.: Recovers all the contents of the specified group<br>Res. (JSON, 200): <code>{ 'group' : [ ['thing1',...,'qwe'] , ['thing2',...,'rty'] , ... , ['thingN',...,'uio'] ] }</code><br>Res. (JSON, 4xx): <code>{}</code></p>
		<p>GET /sel/{name}/{index}<br>Desc.: Recovers a selected element from a group<br>Res. (JSON, 200): <code>{'element':['thing','content',...,'qwe'] }</code><br>Res. (JSON, 4xx): <code>{}</code></p>
		<p>GET /sel/{name}/{index}/{qtty}<br>Desc.: Recovers a slice of a group, using a starting index and a quantity (range selection). Returns 200 if it recovered at least one element<br>Res. (JSON, 200): <code>{ 'slice' : ['thing1',...,'tail'] , ['thing2'] , ['head','data','more'] }</code><br>Res. (JSON, 4xx): <code>{}</code></p>
		<p>POST /add/sin<br>JSON <code>{'name':'some group','element':['head','content',...,'tail']}</code><br>Desc.: Adds a new element to the bottom of an existing group (yes, it's like a queue). Returns 200 if successful<br>NOTE: If the group does not exist, it will  be created automatically<br>NOTE: If the new element to add matches the head of an existing element, the new element will not be added<br>Res. (JSON, any): <code>{}</code></p>
		<p>DELETE /all<br>Desc.: Deletes all groups. Returns 200 if successful<br>WARNING: This is dangerous<br>Res. (JSON, any): <code>{}</code></p>
		<p>DELETE /sel/{name}<br>Desc.: Delete a specific group and all of its content. Returns 200 if successful<br>Res. (JSON, any): <code>{}</code></p>
		<p>DELETE /sel/{name}/{index}<br>Desc.: Select by index a specific element in a specific group and delete that element. Returns 200 if successful</p>
	</body>
</html>
";

// Group struct

struct Group
{
	data: Vec<Vec<String>>,
}

impl Group
{
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
			let elem_head=elem.first().unwrap();
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
		let val_head=value.first().unwrap();
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

	fn get_range(&self,index: usize, qtty: usize) -> Vec<Vec<String>>
	{
		if !self.index_exists(index)
		{
			return Vec::new()
		};
		let size=self.get_size();
		let qtty_real:usize={ if qtty==0 { size } else { qtty } };
		let mut result:Vec<Vec<String>>=Vec::new();
		let mut pos=index;
		let mut added:usize=0;
		loop
		{
			let elem=self.get(pos);
			result.push(elem.to_vec());
			pos=pos+1;
			added=added+1;
			if pos==size || added==qtty_real
			{
				break;
			};
		};
		result
	}

	fn kick(&mut self,index: usize) -> Vec<String>
	{
		if self.index_exists(index) { self.data.remove(index) } else { Vec::new() }
	}
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
struct POST_BringOne
{
	name:String,
	elem:Vec<String>,
}

/*
#[derive(Deserialize)]
struct POST_BringMul
{
	name:String,
	elem:Vec<Vec<String>>,
}
*/

// HTTP Handlers

#[get("/")]
async fn get_status() -> HttpResponse
{
	HttpResponse::Ok()
	.status(StatusCode::from_u16(200).unwrap())
	.json( json!({}) )
}

#[get("/help")]
async fn page_help() -> HttpResponse
{
	HttpResponse::Ok()
	.status(StatusCode::from_u16(200).unwrap())
	.insert_header(("Content-Type","text/html"))
	.body( RQUE_HTML_HELP.to_string() )
}

#[get("/all")]
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

#[get("/sel/{name}")]
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
	.json( if status_code==200 { json!({ "group":result }) } else { json!({}) } )
}

#[get("/sel/{name}/{index}")]
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

#[get("/sel/{name}/{index}/{qtty}")]
async fn get_index_range(from_path: web::Path<(String,usize,usize)>,app_data: web::Data<TheAppState>) -> HttpResponse
{
	let (name,index,qtty)=from_path.into_inner();
	let counter=app_data.counter.lock().unwrap();
	let mut the_slice:Vec<Vec<String>>=Vec::new();
	let mut status_code:u16={ if counter.quecol.is_empty() { 200 } else { 404 } };
	if status_code==200
	{
		if !counter.quecol.contains_key(&name)
		{
			status_code=404;
		};
	};
	if status_code==200
	{
		let ul_group=counter.quecol.get(&name).unwrap();
		let res:Vec<Vec<String>>=ul_group.get_range(index,qtty);
		if res.len()>0
		{
			for e in res.iter()
			{
				the_slice.push(e.to_vec());
			};
		};
	};
	if status_code==200
	{
		if the_slice.len()==0
		{
			status_code=403;
		};
	};
	HttpResponse::Ok()
	.status(StatusCode::from_u16(status_code).unwrap())
	.json( if status_code==200 { json!({ "slice":the_slice }) } else { json!({}) } )
}

#[post("/add/sin")]
async fn post_queue_add(from_post: web::Json<POST_BringOne>,app_data: web::Data<TheAppState>) -> HttpResponse
{
	let mut status_code:u16={ if from_post.elem.len()==0 {403} else {200} };
	if status_code==200
	{
		let new_name=from_post.name.clone();
		let new_elem=from_post.elem.clone();
		let mut counter=app_data.counter.lock().unwrap();
		match counter.quecol.get_mut(&new_name)
		{
			Some(fq) => {
				if fq.add(new_elem.clone())
				{
					println!("\n- Added to existing queue\n  Name: {}\n  New: {:?}",&new_name,&new_elem);
				}
				else
				{
					status_code=403;
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
	.json(json!({}))
}

#[delete("/sel/{name}")]
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
	.json(json!({}))
}

#[delete("/sel/{name}/{index}")]
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
	.json(json!({}))
}

// Application setup

fn get_port() -> u16
{
	println!("\n- Getting the port");
	let mut args: Vec<String> = env::args().collect();
	if args.len()==1
	{
		println!("  Using the default port");
		RQUE_DEFAULT_PORT
	}
	else
	{
		let port_raw:String=args.remove(1);
		match port_raw.parse::<u16>()
		{
			Ok(num) => {
				println!("  Choosing the given port");
				num
			},
			Err(_) => {
				println!("  Using default port instead: Got NaN from the args");
				RQUE_DEFAULT_PORT
			},
		}
	}
}

#[actix_web::main]
async fn main() -> std::io::Result<()>
{
	println!("\n[ rQUE ]");
	let port=get_port();
	println!("\nChosen port: {}\n",port);
	let persistent=web::Data::new(TheAppState{
		counter: Mutex::new( Storage{quecol: HashMap::new()} )
	});
	HttpServer::new(move ||
		App::new()
			.app_data(persistent.clone())
			.service(page_help)
			.service(get_status)
			.service(get_names)
			.service(get_queue)
			.service(get_index)
			.service(get_index_range)
			.service(post_queue_add)
			.service(delete_all)
			.service(delete_queue)
			.service(delete_index)
		)
		.bind(("127.0.0.1",port))?
		.run()
		.await
}
