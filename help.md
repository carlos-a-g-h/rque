# rQUE docs

## How to run

The only (optional) argument is the port number:
```
./rque {PORT}
```
Example: Run in the default port (8080)
```
./rque
```
Example: Run in a custom port
```
./rque 23456
```
If the given argument for the port is NaN, the program will just use the default port

## What kind of data is stored and how it stores it

rQUE stores data in a hashmap, each key is a group name and each value is a group

A group is a variable length array that stores items

Items are variable length arrays with the first index being the 'head'

This is a representation of how it would look like:
```
{
	'group 1':
	[
		['thing1'],
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
```

Important to know
- Each group name is unique. When you try to add a new item, the group will be created if it does not exist yet
- Items cannot have a length of zero, they must at least have one element that serves as the head
- 2 or more items in the same group cannot have the same head: attempting to add an item that has the same head as another item will fail

## API reference

### Endpoints

GET requests read existing data only, POST requests add data, and DELETE requests delete and retrieve data

Any modifications made with POST and DELETE requests are printed in the console output

```
GET /help
Desc.: A help page like the one you're reading, but in HTML
```
```
GET /
Desc.: It always returns HTTP 200
Res. (200): {}
```
```
GET /all
Desc.: Recovers a list of existing group names
Res. (JSON, 200): { 'status':200 , 'result':['name1','name2',...,'nameN']}
Res. (JSON, 4xx): { 'status':4xx , 'msg':'error description' }
```
```
GET /sel/{name}
Desc.: Recovers all the items of the specified group. It returns HTTP 206 (partial) if the group is empty
Res. (JSON, 200): { 'status':200 , 'group' : [ ['thing1',...,'qwe'] , ['thing2',...,'rty'] , ... , ['thingN',...,'uio'] ] }
Res. (JSON, 206): { 'status':206 , 'group' : [] }
Res. (JSON, 4xx): { 'status':4xx , 'msg':'error description' }
```
```
GET /sel/{name}/{index}
Desc.: Recovers a selected item from a group by its index
Res. (JSON, 200): { 'status':200 ,'item':['thing','content',...,'qwe'] }
Res. (JSON, 4xx): { 'status':4xx , 'msg':'error description' }
```
```
GET /sel/{name}/{index}/{qtty}
Desc.: Recovers a slice of a group by selecting in range
Res. (JSON, 200): { 'status':200 , 'slice' : ['thing1',...,'tail'] , ['thing2'] , ['head','data','more'] }
Res. (JSON, 4xx): { 'status':4xx , 'msg':'error description' }
```
```
POST /add/sin
JSON {'name':'some group','item':['head','content',...,'tail']}
Desc.: Adds a new item to the bottom of an existing group (yes, it's like a queue)
Res. (JSON, 200): { 'status': 200 }
Res. (JSON, 4xx): { 'status': 4xx , 'msg' : 'error description' }
```
```
POST /add/mul
JSON { 'name' : 'some group' , 'list' : ['head','content'] , ... , ['other','tail'] , ['thing'] }
Desc.: Adds multiple new items to a group. Returns HTTP 206 if partially successful
Res. (JSON, 200): { 'status' : 200 }
Res. (JSON, 206): { 'status' : 206 , details: [...] }
Res. (JSON, 4xx): { 'status' : 4xx , 'msg' : 'error description' }
```
```
DELETE /all
Desc.: Deletes all groups. Use with caution
Res. (JSON, 200): { 'status': 200 }
Res. (JSON, 4xx): { 'status': 4xx , 'error description' }
```
```
DELETE /sel/{name}
Desc.: Delete a specific group along with its items
Res. (JSON, 200): { 'status': 200 }
Res. (JSON, 4xx): { 'status': 4xx , 'msg' : 'error description' }
```
```
DELETE /sel/{name}/{index}
Desc.: Deletes an item from a specified group and recovers it in the JSON response
Res. (JSON, 200): { 'status' : 200 , 'item' : ['some item','other data'] }
Res. (JSON, 4xx): { 'status' : 4xx , 'msg' : 'error description' }
```
```
DELETE /sel/{name}/{index}/{qtty}
Desc.: Deletes multiple items selected in range and recovers the deleted elements in the JSON response
Res. (JSON, 200): { 'status':200 , 'slice' : ['thing1',...,'tail'] , ['thing2'] , ['head','data','more'] }
Res. (JSON, 4xx): { 'status':4xx , 'msg':'error description' }
```

### Range selection

Range selection works by declaring a starting index and a quantity

If the case that the quantity is zero, all items after the starting index are selected, including the item in the starting index

Examples:

```
DELETE /sel/queue1/3/2
```
Deletes from the group 'queue1' the items no. 3 and 4, because the starting index is 3 and the quantity is 2

```
DELETE /sel/stack/4/0
```
Deletes all items in the group 'stack' leaving only the items 0, 1, 2 and 3. In this case the starting index is 3 and all the other items after the item no. 3 are also selected because the quantity is set to 0

```
GET /sel/users/0/0
```
Gets all items from the group 'users', because the index is 0 and the quantity is also 0
