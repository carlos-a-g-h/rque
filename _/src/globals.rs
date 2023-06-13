pub static RQUE_DEFAULT_PORT:u16=8080;

pub static RQUE_MSG_DEF_PORT:&str="Using the default port";
pub static RQUE_MSG_CUS_PORT:&str="Using a custom port";

pub static RQUE_ERROR_ZERO_GROUPS:&str="There are no groups yet";
pub static RQUE_ERROR_GROUP_NOT_FOUND:&str="The specified group does not exist";
pub static RQUE_ERROR_GROUP_EMPTY:&str="The specified group is empty";
pub static RQUE_ERROR_ITEM_NOT_FOUND:&str="The item that correspond the specified index does not exist";
pub static RQUE_ERROR_ITEM_NOT_VALID:&str="The provided item is not valid";
pub static RQUE_ERROR_SLICE:&str="Try lowering the starting index";

pub static RQUE_INFO:&str="Written by Carlos Alberto González Hernández - 2023-04-05";
pub static RQUE_HELP:&str="
<!DOCTYPE html>
<html lang=\"en\">
	<meta charset=\"UTF-8\">
	<meta name=\"viewport\" content=\"width=device-width,initial-scale=1\">
	<head>
		<title>rQUE quick help</title>
	</head>
	<body>
		<h1>rQUE</h1>
		<h2>Running the server</h2>
		<h3>Usage and examples</h3>

		<p>The only (optional) argument is the port<br>
		<strong><code>$ rque {PORT}</code></strong></p>

		<p>Example1: Runs the server normally at the default port (8080) or at a custom port specified by an environment variable<br>
		<strong><code>$ rque</strong></code></p>

		<p>Example2: Runs the server at port 23456<br>
		<strong><code>$ rque 23456</strong></code></p>

		<h3>Environment variables</h3>
		<p><strong>RQUE_CUSTOMPORT</strong><br>Type: Number<br>Descr.: Custom port. The server will first look into the port argument before this environment variable</p>
		<p><strong>RQUE_SECRETKEY</strong><br>Type: String<br>Descr.: Secret key that acts as a token for authorising all requests. All request must include an 'Authorization' header of type 'Bearer' like this one: <code>{ 'Authorization' : 'Bearer TheSecretKey' }</code>. The only exception of this is the GET request to the '/help' route if the client is '127.0.0.1'</p>

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
		<p>GET /g/{name}<br>Desc.: Recovers all the items of the specified group and its size. Returns HTTP 206 (Partial response) if the group is empty<br>Res. (JSON, 200): <code>{ 'status':200 , 'group_size':9999 ,'group' : [ ['thing1',...,'qwe'] , ['thing2',...,'rty'] , ... , ['thingN',...,'uio'] ] }</code><br>Res. (JSON, 206): <code>{ 'status':206 , group_size:0 , 'group':[] }</code><br>Res. (JSON, 4xx): <code>{ 'status':4xx , 'msg':'error description' }</code></p>
		<p>GET /g/{name}/s/{index}<br>Desc.: Recovers a selected item from a group by its index<br>Res. (JSON, 200): <code>{ 'status':200 ,'item':['thing','content',...,'qwe'] }</code><br>Res. (JSON, 4xx): <code>{ 'status':4xx , 'msg':'error description' }</code></p>
		<p>GET /g/{name}/s/{index}/{qtty}<br>Desc.: Recovers a slice of a group by selecting in range<br>Res. (JSON, 200): <code>{ 'status':200 , 'slice' : ['thing1',...,'tail'] , ['thing2'] , ['head','data','more'] }</code><br>Res. (JSON, 4xx): <code>{ 'status':4xx , 'msg':'error description' }</code></p>
		<p>POST /add-one<br>JSON <code>{ 'name':'some group' , 'item': ['head','content',...,'tail']}</code><br>Desc.: Adds a new item to the bottom of an existing group (yes, it's like a queue)<br>Res. (JSON, 200): <code>{ 'status' : 200 , 'newgroup' : bool }</code><br>Res. (JSON, 4xx): <code>{ 'status' : 4xx , 'msg' : 'error description' }</code></p>
		<p>POST /add-mul<br>JSON <code>{ 'name':'some group' , 'list': ['head','content'] , ... , ['other','tail'] , ['thing'] }</code><br>Desc.: Adds multiple new items to a group. Returns 206 if partially successful<br>Res. (JSON, 200): <code>{ 'status' : 200 , 'newgroup' : bool }</code><br>Res. (JSON, 206): <code>{ 'status' : 206 , 'newgroup' : bool , details: [...] }</code><br>Res. (JSON, 4xx): <code>{ 'status' : 4xx , 'msg' : 'error description' }</code></p>
		<p>DELETE /all<br>Desc.: Deletes all groups. Use with caution<br>Res. (JSON, 200): <code>{ 'status': 200 }</code><br>Res. (JSON, 4xx): <code>{ 'status': 4xx , 'error description' }</code></p>
		<p>DELETE /d/{name}<br>Desc.: Delete a specific group along with its items<br>Res. (JSON, 200): <code>{ 'status': 200 }</code><br>Res. (JSON, 4xx): <code>{ 'status': 4xx , 'msg' : 'error description' }</code></p>
		<p>DELETE /d/{name}/{index}<br>Desc.: Deletes an item from a specified group and recovers it in the JSON response<br>Res. (JSON, 200): <code>{ 'status' : 200 , 'item' : ['some item','other data'] }</code><br>Res. (JSON, 4xx): <code>{ 'status' : 4xx , 'msg' : 'error description' }</code></p>
		<p>DELETE /d/{name}/{index}/{qtty}<br>Desc.: Deletes multiple items selected in range and recovers the deleted elements in the JSON response<br>Res. (JSON, 200): <code>{ 'status':200 , 'slice' : ['thing1',...,'tail'] , ['thing2'] , ['head','data','more'] }</code><br>Res. (JSON, 4xx): <code>{ 'status':4xx , 'msg':'error description' }</code></p>
		<h3>Range selection</h3>
		<p>Range selection works by declaring a starting index and a quantity<br>If the quantity is zero, all items after the starting index are selected, including the item in the starting index</p>
		<p>Examples:</p>
		<p>DELETE /group/queue1/3/2<br>Deletes from the group 'queue1' the items no. 3 and 4, because the starting index is 3 and the quantity is 2</p>
		<p>DELETE /group/stack/4/0<br>Deletes all items in the group 'stack' leaving only the items 0, 1, 2 and 3. In this case the starting index is 3 and all the other items after the item no. 3 are also selected because the quantity is set to 0</p>
		<p>GET /group/users/0/0<br>Gets all items from the group 'users', because the index is 0 and the quantity is also 0</p>
	</body>
</html>
";
