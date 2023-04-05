use std::collections::HashMap;
use std::env;
use std::sync::Mutex;
use actix_web::{get, post, delete, web, App, HttpServer, HttpResponse};
use actix_web::http::StatusCode;
use serde::Deserialize;
use serde_json::json;


static RQUE_DEFAULT_PORT:u16=8080;

static RQUE_MSG_DEF_PORT:&str="Using the default port";
static RQUE_MSG_CUS_PORT:&str="Using a custom port";

static RQUE_ERROR_ZERO_GROUPS:&str="There are no groups yet";
static RQUE_ERROR_GROUP_NOT_FOUND:&str="The specified group does not exist";
static RQUE_ERROR_GROUP_EMPTY:&str="The specified group is empty";
static RQUE_ERROR_ITEM_NOT_FOUND:&str="The item that correspond the specified index does not exist";
static RQUE_ERROR_ITEM_NOT_VALID:&str="The provided item is not valid";
static RQUE_ERROR_SLICE:&str="Try lowering the starting index";

static RQUE_INFO:&str="Written by Carlos Alberto González Hernández - 2023-04-02";
static RQUE_HELP:&str="
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
		<h3>Schema</h3>
		<p>The data is stored in a large hashmap, where each key is the name of a group and each value is the content of the group</p>
		<p>
			<strong>
				<code>
<pre>
{
	'group 1':
	[
		['thing1'] ,
		['thing2','bonus']
	],

	'group2':
	[
		['thing1']
	],

	'another group':
	[
		['headname1','data'],
		['headname2','data'],
		['headname3','data','more data']
	]
}

</pre>
				</code>
			</strong>
		</p>
		<h3>Rules</h3>
		<p>→ Groups are lists and each group name is unique<br>→ Each item inside a group is a list with the first index being the head of the item<br>→ Items cannot have a length of zero, they must at least have the head<br>→ 2 or more items in the same group cannot have the same head, and this is checked automatically by the program before adding new items to a group<br>→ If a group does not exist when adding items, the group is created automatically before adding the new items<br>→ A group can only exist as empty if all of its items have been removed manually</p>

		<h2>API reference</h2>
		<h3>Endpoints</h3>
		<p>GET requests read existing data only, POST requests add data, and DELETE requests remove/steal data<br>All data modifications with POST and DELETE requests are printed in the console output</p>
		<p>GET /help<br>Desc.: This help</p>
		<p>GET /<br>Desc.: It always returns HTTP 200<br>Res. (200): <code>{}</code></p>
		<p>GET /all<br>Desc.: Recovers a list of existing group names<br>Res. (JSON, 200): <code>{ 'status':200 , 'result':['name1','name2',...,'nameN']}</code><br>Res. (JSON, 4xx): <code>{ 'status':4xx , 'msg':'error description' }</code></p>
		<p>GET /sel/{name}<br>Desc.: Recovers all the items of the specified group. Returns HTTP 206 (Partial response) if the group is empty<br>Res. (JSON, 200): <code>{ 'status':200 , 'group' : [ ['thing1',...,'qwe'] , ['thing2',...,'rty'] , ... , ['thingN',...,'uio'] ] }</code><br>Res. (JSON, 206): <code>{ 'status':206 , 'group':[] }</code><br>Res. (JSON, 4xx): <code>{ 'status':4xx , 'msg':'error description' }</code></p>
		<p>GET /sel/{name}/{index}<br>Desc.: Recovers a selected item from a group by its index<br>Res. (JSON, 200): <code>{ 'status':200 ,'item':['thing','content',...,'qwe'] }</code><br>Res. (JSON, 4xx): <code>{ 'status':4xx , 'msg':'error description' }</code></p>
		<p>GET /sel/{name}/{index}/{qtty}<br>Desc.: Recovers a slice of a group by selecting in range<br>Res. (JSON, 200): <code>{ 'status':200 , 'slice' : ['thing1',...,'tail'] , ['thing2'] , ['head','data','more'] }</code><br>Res. (JSON, 4xx): <code>{ 'status':4xx , 'msg':'error description' }</code></p>
		<p>POST /add/sin<br>JSON <code>{ 'name':'some group' , 'item': ['head','content',...,'tail']}</code><br>Desc.: Adds a new item to the bottom of an existing group (yes, it's like a queue)<br>Res. (JSON, 200): <code>{ 'status' : 200 , 'newgroup' : bool }</code><br>Res. (JSON, 4xx): <code>{ 'status' : 4xx , 'msg' : 'error description' }</code></p>
		<p>POST /add/mul<br>JSON <code>{ 'name':'some group' , 'list': ['head','content'] , ... , ['other','tail'] , ['thing'] }</code><br>Desc.: Adds multiple new items to a group. Returns 206 if partially successful<br>Res. (JSON, 200): <code>{ 'status' : 200 , 'newgroup' : bool }</code><br>Res. (JSON, 206): <code>{ 'status' : 206 , 'newgroup' : bool , details: [...] }</code><br>Res. (JSON, 4xx): <code>{ 'status' : 4xx , 'msg' : 'error description' }</code></p>
		<p>DELETE /all<br>Desc.: Deletes all groups. Use with caution<br>Res. (JSON, 200): <code>{ 'status': 200 }</code><br>Res. (JSON, 4xx): <code>{ 'status': 4xx , 'error description' }</code></p>
		<p>DELETE /sel/{name}<br>Desc.: Delete a specific group along with its items<br>Res. (JSON, 200): <code>{ 'status': 200 }</code><br>Res. (JSON, 4xx): <code>{ 'status': 4xx , 'msg' : 'error description' }</code></p>
		<p>DELETE /sel/{name}/{index}<br>Desc.: Deletes an item from a specified group and recovers it in the JSON response<br>Res. (JSON, 200): <code>{ 'status' : 200 , 'item' : ['some item','other data'] }</code><br>Res. (JSON, 4xx): <code>{ 'status' : 4xx , 'msg' : 'error description' }</code></p>
		<p>DELETE /sel/{name}/{index}/{qtty}<br>Desc.: Deletes multiple items selected in range and recovers the deleted elements in the JSON response<br>Res. (JSON, 200): <code>{ 'status':200 , 'slice' : ['thing1',...,'tail'] , ['thing2'] , ['head','data','more'] }</code><br>Res. (JSON, 4xx): <code>{ 'status':4xx , 'msg':'error description' }</code></p>
		<h3>Range selection</h3>
		<p>Range selection works by declaring a starting index and a quantity<br>If the quantity is zero, all items after the starting index are selected, including the item in the starting index</p>
		<p>Examples:</p>
		<p>DELETE /sel/queue1/3/2<br>Deletes from the group 'queue1' the items no. 3 and 4, because the starting index is 3 and the quantity is 2</p>
		<p>DELETE /sel/stack/4/0<br>Deletes all items in the group 'stack' leaving only the items 0, 1, 2 and 3. In this case the starting index is 3 and all the other items after the item no. 3 are also selected because the quantity is set to 0</p>
		<p>GET /sel/users/0/0<br>Gets all items from the group 'users', because the index is 0 and the quantity is also 0</p>
	</body>
</html>
";

// Group struct

struct Group { data: Vec<Vec<String>> }

impl Group
{

	fn new() -> Group { Group { data:Vec::new() } }

	fn get_size(&self) -> usize { self.data.len() }

	fn is_empty(&self) -> bool { let size=self.get_size();if size==0 { true } else { false } }

	fn index_exists(&self,index:usize) -> bool { let size=self.get_size();if index>size || size==0 || size==index { false } else { true } }

	fn get(&self,index: usize) -> Vec<String> { if self.index_exists(index) { self.data[index].clone() } else { Vec::new() } }

	fn has_head(&self,head: &String) -> bool
	{
		if self.is_empty()
		{
			return false;
		};
		let mut rep=false;
		for elem in &self.data
		{
			let elem_head=elem.first().unwrap();
			if elem_head==head
			{
				rep=true;break;
			};
		};
		rep
	}

	fn add(&mut self,value: Vec<String>) -> bool
	{
		if value.len()==0
		{
			return false;
		};
		let val_head=value.first().unwrap();
		if self.has_head(val_head)
		{
			return false;
		};
		self.data.push(value);
		true
	}

	fn kick(&mut self,index: usize) -> Vec<String> { if self.index_exists(index) { self.data.remove(index) } else { Vec::new() } }

	fn get_range(&mut self,index: usize, qtty: usize, steal: bool) -> Vec<Vec<String>>
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
			let elem:Vec<String>={
				if steal { self.kick(pos) } else { self.get(pos) }
			};
			if elem.len()==0
			{
				break;
			};
			result.push(elem.to_vec());
			if !steal
			{
				pos=pos+1;
			};
			added=added+1;
			if pos==size || added==qtty_real
			{
				break;
			};
		};
		result
	}
}

// Main Data struct

struct Storage { quecol: HashMap<String,Group> , password: String }

impl Storage
{
	fn get_size(&self) -> usize { self.quecol.len() }

	fn is_empty(&self) -> bool { return self.quecol.is_empty() }
}

// Application Data in a Mutex

struct TheAppState { holder: Mutex<Storage> }

// JSON schemas

#[derive(Deserialize)]
struct POST_BringOne
{
	name:String,
	item:Vec<String>,
}

#[derive(Deserialize)]
struct POST_BringMul
{
	name:String,
	list:Vec<Vec<String>>
}

#[derive(Deserialize)]
struct Configuration
{
	port: u16,
	password: String,
}

// HTTP Handlers

fn json_res(sc: u16,payload: serde_json::Value) -> HttpResponse
{
	HttpResponse::Ok()
	.status(StatusCode::from_u16(sc).unwrap())
	.json( payload )
}

#[get("/")]
async fn get_status() -> HttpResponse
{
	/*
	HttpResponse::Ok()
	.status(StatusCode::from_u16(200).unwrap())
	.json( json!({}) )*/
	json_res(200, json!({}) )
}

#[get("/help")]
async fn show_help() -> HttpResponse
{
	HttpResponse::Ok()
	.status(StatusCode::from_u16(200).unwrap())
	.insert_header(("Content-Type","text/html"))
	.body( RQUE_HELP.to_string() )
}

#[get("/all")]
async fn get_names(app_data: web::Data<TheAppState>) -> HttpResponse
{
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

#[get("/sel/{name}")]
async fn get_group(from_path: web::Path<String>,app_data: web::Data<TheAppState>) -> HttpResponse
{
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
			};
			200
		}
	};

	json_res(status_code,json!({ "status":status_code,"group":list }))
}

#[get("/sel/{name}/{index}")]
async fn get_index(from_path: web::Path<(String,usize)>,app_data: web::Data<TheAppState>) -> HttpResponse
{
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

#[get("/sel/{name}/{index}/{qtty}")]
async fn get_range(from_path: web::Path<(String,usize,usize)>,app_data: web::Data<TheAppState>) -> HttpResponse
{
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

#[post("/add/sin")]
async fn post_group_addsin(from_post: web::Json<POST_BringOne>,app_data: web::Data<TheAppState>) -> HttpResponse
{
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

#[post("/add/mul")]
async fn post_group_addmul(from_post: web::Json<POST_BringMul>,app_data: web::Data<TheAppState>) -> HttpResponse
{
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
			206=>json!({"status":status_code,"newgroup":newgroup,"details":res_arr}),
			_=>json!({"status":status_code,"msg":msg})
		}
	)
}

#[delete("/all")]
async fn delete_all(app_data: web::Data<TheAppState>) -> HttpResponse
{
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

#[delete("/sel/{name}")]
async fn delete_group(from_path: web::Path<String>,app_data: web::Data<TheAppState>) -> HttpResponse
{
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

#[delete("/sel/{name}/{index}")]
async fn delete_index(from_path: web::Path<(String,usize)>,app_data: web::Data<TheAppState>) -> HttpResponse
{
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

#[delete("/sel/{name}/{index}/{qtty}")]
async fn delete_range(from_path: web::Path<(String,usize,usize)>,app_data: web::Data<TheAppState>) -> HttpResponse
{
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

	let cfg_port:u16={
		println!("\n- From config: Obtaining the port");
		let mut args: Vec<String>=env::args().collect();
		let port_raw:String=args.remove(1);

		let (port,tryenv):(u16,bool)=parse_port(port_raw);
		if tryenv
		{
			match env::var("RQUE_PORT")
			{
				Err(_)=>port,
				Ok(raw_value)=>{
					let (port_ok,ok):(u16,bool)=parse_port(raw_value);
					println!("  {}", if ok { RQUE_MSG_CUS_PORT } else { RQUE_MSG_DEF_PORT } );
					port_ok
				}
			}
		}
		else { println!("  {}",RQUE_MSG_CUS_PORT);port }
	};

	let cfg_skey:String={
		println!("\n- From config: Obtaining token");
		let (msg,from_env):(&str,String)=match env::var("RQUE_SKEY")
		{
			Err(_)=>("RQUE_SKEY env var not found or not valid: no authorization will be needed",String::new()),
			Ok(value)=>("Found secret key env var",from_env),
		};
		println!("  {}",msg);
		from_env
	};

	let pdata=web::Data::new(TheAppState{
		holder: Mutex::new( Storage{ quecol: HashMap::new() , skey: cfg_skey } )
	});

	HttpServer::new(move ||
		App::new()
			.app_data(pdata.clone())
			.service(show_help)
			.service(get_status)
			.service(get_names)
			.service(get_group)
			.service(get_index)
			.service(get_range)
			.service(post_group_addsin)
			.service(post_group_addmul)
			.service(delete_all)
			.service(delete_group)
			.service(delete_index)
			.service(delete_range)
		)
		.bind(("127.0.0.1",cfg_port))?
		.run()
		.await
}
