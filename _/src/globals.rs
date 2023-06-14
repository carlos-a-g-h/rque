pub static RQUE_DEFAULT_PORT:u16=8080;

pub static RQUE_MSG_DEF_PORT:&str="Using the default port";
pub static RQUE_MSG_CUS_PORT:&str="Using a custom port";

pub static RQUE_ERROR_ZERO_GROUPS:&str="There are no groups yet";
pub static RQUE_ERROR_GROUP_NOT_FOUND:&str="The specified group does not exist";
pub static RQUE_ERROR_GROUP_EMPTY:&str="The specified group is empty";
pub static RQUE_ERROR_ITEM_NOT_FOUND:&str="The item that correspond the specified index does not exist";
pub static RQUE_ERROR_ITEM_NOT_VALID:&str="The provided item is not valid";
pub static RQUE_ERROR_SLICE:&str="Try lowering the starting index";

pub static RQUE_INFO:&str="Written by Carlos Alberto González Hernández - 2023-06-14";
pub static RQUE_HELP:&str="
<!DOCTYPE html>
<html lang=\"en\">
	<meta charset=\"UTF-8\">
	<meta name=\"viewport\" content=\"width=device-width,initial-scale=1\">
	<head>
		<title>rQUE documentation</title>
	</head>
	<body>
		<h1>rQUE</h1>
		<h2>Running the server</h2>
		<h3>Usage and examples</h3>
		<p>The only (optional) argument is the port
		<br><strong><code>$ rque {PORT}</code></strong></p>
		<p>Example1: Runs the server normally at the default port (8080) or at a custom port specified by an environment variable
		<br><strong><code>$ rque</strong></code></p>
		<p>Example2: Runs the server at port 23456
		<br><strong><code>$ rque 23456</strong></code></p>
		<h3>Environment variables</h3>
		<p><strong>RQUE_CUSTOMPORT</strong>
		<br>Type: Number
		<br>Descr.: Custom port. The server will first look into the port argument before this environment variable</p>
		<p><strong>RQUE_SECRETKEY</strong>
		<br>Type: String
		<br>Descr.: Secret key that acts as a token for authorising all requests. All request must include an 'Authorization' header of type 'Bearer' like this one: <code>{ 'Authorization' : 'Bearer TheSecretKey' }</code>. The only exception of this is the GET request to the '/help' route if the client is '127.0.0.1'</p>

		<h2>How data is stored</h2>
		<h3>Terminology</h3>
		<p>→ The data is stored in a large hashmap, where each key is the name of a group and each value is the contents of the group
		<br>→ Groups are lists of lists of strings ( List[List[String]] ), they contain items
		<br>→ Each item inside a group is a list o strings ( [List[String] ) with the first index being the 'head' of the item
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

		<p>→ Items cannot have a length of zero, they must at least have the head
		<br>→ 2 or more items in the same group cannot have the same head, and this is checked automatically by the program before adding new items to a group
		<br>→ Each group name is unique and if a group does not exist when adding items, the group is created automatically before adding the new items
		<br>→ A group can only exist as empty if all of its items have been removed manually</p>

		<h2>API reference</h2>
		<h3>Endpoints</h3>

		<p>GET requests read existing data only, POST requests add data, and DELETE requests delete data
		<br>All data modifications with POST and DELETE requests are printed in the console output

		<p>GET /help
		<br>Desc.: This help</p>

		<p>GET /
		<br>Desc.: It always returns HTTP 200
		<br>Res. (200): <code>{}</code></p>

		<p>GET /all
		<br>Desc.: Recovers a list of existing group names
		<br>Res. (JSON, 200): <code>{ 'status':200 , 'result' : List[String] }</code>
		<br>Res. (JSON, 4xx): <code>{ 'status':4xx , 'msg' : String }</code></p>

		<p>GET /r/{name} | name: String
		<br>Desc.: Recovers all the items of the specified group and its size (group_size key). Returns HTTP 206 (Partial response) if the group is empty

		<br>Res. (JSON, 200): <code>{ 'status':200 , 'group_size' : Integer , 'group' : List[List[String]] }</code>
		<br>Res. (JSON, 206): <code>{ 'status':206 , 'group_size':0 , 'group':[] }</code>
		<br>Res. (JSON, 4xx): <code>{ 'status':4xx , 'msg': String }</code></p>

		<p>GET /r/{name}/size | name: String
		<br>Desc.: Recovers only the size of the specified group (group_size key). Returns HTTP 206 (Partial response) if the group is empty

		<br>Res. (JSON, 200): <code>{ 'status':200 , 'group_size' : Integer }</code>
		<br>Res. (JSON, 206): <code>{ 'status':206 , 'group_size':0 }</code>
		<br>Res. (JSON, 4xx): <code>{ 'status':4xx , 'msg': String }</code></p>

		<p>GET /r/{name}/s/{index} | name: String | index: Integer
		<br>Desc.: Recovers a selected item from a group by its index
		<br>Res. (JSON, 200): <code>{ 'status':200 , 'item' : List[String] }</code>
		<br>Res. (JSON, 4xx): <code>{ 'status':4xx , 'msg' : String }</code></p>

		<p>GET /r/{name}/s/{index}/{qtty} | name: String | index: Integer | qtty: Integer
		<br>Desc.: Recovers a slice of a group by selecting in range
		<br>Res. (JSON, 200): <code>{ 'status':200 , 'slice' : List[List[String]] }</code>
		<br>Res. (JSON, 4xx): <code>{ 'status':4xx , 'msg': String }</code></p>

		<p>POST /add-one
		<br>JSON <code>{ 'name' : String , 'item': List[String] }</code>
		<br>Desc.: Adds a new item to the bottom of an existing group (yes, it's like a queue) and tells wether the group has been created from scratch or not (newgroup key)
		<br>Res. (JSON, 200): <code>{ 'status' : 200 , 'newgroup' : Bool }</code>
		<br>Res. (JSON, 4xx): <code>{ 'status' : 4xx , 'msg' : String }</code></p>

		<p>POST /add-mul
		<br>JSON <code>{ 'name' : String , 'list': List[List[String]] , 'details': Bool }</code>
		<br>Desc.: Adds multiple new items to a group and tells wether the group has been created from scratch or not ('newgroup' key in the response). Returns 206 if partially successful and in this case it will say the amount of items added ('items_succ' key), the ammount of items that could not be added ('items_fail' key), and if the 'details' key is set to true, a list of booleans will be returned, you can use this list to get a detailed view of the items that were added and the items that were not added
		<br>Res. (JSON, 200): <code>{ 'status' : 200 , 'newgroup' : Bool }</code>
		<br>Res. { 'details':true } (JSON, 206): <code>{ 'status' : 206 , 'newgroup' : Bool , items_succ: Integer , items_fail : Integer , details: List[Bool] }</code>
		<br>Res. { 'details':false } (JSON, 206): <code>{ 'status' : 206 , 'newgroup' : Bool , items_succ: Integer , items_fail : Integer }</code>
		<br>Res. (JSON, 4xx): <code>{ 'status' : 4xx , 'msg' : String }</code></p>

		<p>DELETE /all
		<br>Desc.: Deletes all groups. Use with caution<br>Res. (JSON, 200): <code>{ 'status': 200 }</code>
		<br>Res. (JSON, 4xx): <code>{ 'status': 4xx , 'msg' : String }</code></p>

		<p>DELETE /d/{name} | name: String
		<br>Desc.: Delete a specific group along with its items
		<br>Res. (JSON, 200): <code>{ 'status': 200 }</code>
		<br>Res. (JSON, 4xx): <code>{ 'status': 4xx , 'msg' : String }</code></p>

		<p>DELETE /d/{name}/{index}
		<br>JSON <code>{ 'recover' : Bool }</code>
		<br>Desc.: Deletes an item from a specified group. You can recover the deleted item in the response (with the 'recover' key)
		<br>Res. { 'recover' : true } (JSON, 200): <code>{ 'status' : 200 , 'item' : List[String] }</code>
		<br>Res. { 'recover' : false } (JSON, 200): <code>{ 'status' : 200 }</code>
		<br>Res. (JSON, 4xx): <code>{ 'status' : 4xx , 'msg' : String }</code></p>

		<p>DELETE /d/{name}/{index}/{qtty}
		<br>JSON <code>{ 'recover' : Bool }</code>
		<br>Desc.: Deletes multiple items selected in range. You can recover a slice of the deleted items in the response (with the 'recover' key)
		<br>Res. { 'recover' : true } (JSON, 200): <code>{ 'status':200 , 'slice_size' : Integer , 'slice' : List[List[String]] }</code>
		<br>Res. { 'recover' : false } (JSON, 200): <code>{ 'status':200 }</code>
		<br>Res. (JSON, 4xx): <code>{ 'status':4xx , 'msg' : String }</code></p>

		<h3>Range selection</h3>
		<p>Range selection works by declaring a starting index and a quantity
		<br>If the quantity is zero, all items after the starting index are selected, including the item in the starting index</p>

		<p>Examples:</p>
		<p>DELETE /d/queue1/3/2
		<br>Deletes from the group 'queue1' the items no. 3 and 4, because the starting index is 3 and the quantity is 2</p>

		<p>DELETE /d/stack/4/0
		<br>Deletes all items in the group 'stack' leaving only the items 0, 1, 2 and 3. In this case the starting index is 3 and all the other items after the item no. 3 are also selected because the quantity is set to 0</p>

		<p>GET /r/users/0/0
		<br>Gets all items from the group 'users', because the index is 0 and the quantity is also 0</p>

	</body>
</html>
";
