#!/usr/bin/python3.9

import asyncio
import aiohttp
import yarl

_rque_default_addr="http://127.0.0.1"
_rque_default_port=8080

def url_build(api_path,rque_home,rque_port):
	y=yarl.URL(rque_home)
	u=y.scheme+"://"+y.host
	if rque_port:
		try:
			int(rque_port)
		except:
			pass
		else:
			u=u+":"+str(rque_port)

	return u+api_path

async def rque_api(the_method="GET",api_path="/all",the_json=None,session=None,skey=None,rque_home=_rque_default_addr,rque_port=_rque_default_port):

	local=False
	if not session:
		local=True
		session=aiohttp.ClientSession()
		session.headers.update({"Content-Type":"application/json","Accept":"application/json",
		})

	if skey:
		session.headers.update({"Authorization":"Bearer "+skey})

	the_url=url_build(api_path,rque_home,rque_port)
	print(f"\n- rQUE call: {the_method} {the_url}\n  Data: {the_json}")
	try:
		async with session.request(method=the_method,url=the_url,json=the_json,verify_ssl=False) as response:
			rsp_json=await response.json()
			if not str(response.status).startswith("2"):
				raise Exception(f"Non-valid status code of {response.status}")
	except Exception as e:
		print("  error:",e)
		result={}
	else:
		result=rsp_json

	if local:
		await session.close()

	print("  result:",result)
	return result

async def rque_get_all(rque_home=_rque_default_addr,rque_port=_rque_default_port,session=None,skey=None):
	res=await rque_api(session=session,rque_home=rque_home,rque_port=rque_port,skey=skey)
	return res

async def rque_get_group(group_name,rque_home=_rque_default_addr,rque_port=_rque_default_port,session=None,skey=None):
	res=await rque_api(session=session,api_path=f"/sel/{group_name}",rque_home=rque_home,rque_port=rque_port,skey=skey)
	return res

async def rque_get_item(group_name,index,rque_home=_rque_default_addr,rque_port=_rque_default_port,session=None,skey=None):
	res=await rque_api(session=session,api_path=f"/sel/{group_name}/{index}",rque_home=rque_home,rque_port=rque_port,skey=skey)
	return res

async def rque_get_range(group_name,index,qtty,rque_home=_rque_default_addr,rque_port=_rque_default_port,session=None,skey=None):
	res=await rque_api(session=session,api_path=f"/sel/{group_name}/{index}/{qtty}",rque_home=rque_home,rque_port=rque_port,skey=skey)
	return res

async def rque_addsin(group_name,item,rque_home=_rque_default_addr,rque_port=_rque_default_port,session=None,skey=None):
	res=await rque_api(session=session,the_method="POST",the_json={"name":group_name,"item":item},api_path="/add/sin",rque_home=rque_home,rque_port=rque_port,skey=skey)
	return res

async def rque_addmul(group_name,item_list,rque_home=_rque_default_addr,rque_port=_rque_default_port,session=None,skey=None):
	res=await rque_api(session=session,the_method="POST",the_json={"name":group_name,"list":item_list},api_path="/add/mul",rque_home=rque_home,rque_port=rque_port,skey=skey)
	return res

async def rque_del_all(rque_home=_rque_default_addr,rque_port=_rque_default_port,session=None,skey=None):
	res=await rque_api(session=session,the_method="DELETE",rque_port=rque_port,skey=skey)
	return res

async def rque_del_group(group_name,rque_home=_rque_default_addr,rque_port=_rque_default_port,session=None,skey=None):
	res=await rque_api(session=session,the_method="DELETE",api_path=f"/sel/{group_name}",rque_home=rque_home,rque_port=rque_port,skey=skey)
	return res

async def rque_del_item(group_name,index,rque_home=_rque_default_addr,rque_port=_rque_default_port,session=None,skey=None):
	res=await rque_api(session=session,the_method="DELETE",api_path=f"/sel/{group_name}/{index}",rque_home=rque_home,rque_port=rque_port,skey=skey)
	return res

async def rque_del_range(group_name,index,qtty,rque_home=_rque_default_addr,rque_port=_rque_default_port,session=None,skey=None):
	res=await rque_api(session=session,the_method="DELETE",api_path=f"/sel/{group_name}/{index}/{qtty}",rque_home=rque_home,rque_port=rque_port,skey=skey)
	return res

class rque_Client:

	def __init__(self,own_session=True,addr=_rque_default_addr,port=_rque_default_port):
		self.addr=addr
		self.port=port

		if not own_session:
			self.session=None

		if own_session:
			self.session=aiohttp.ClientSession()
			self.session.headers.update({
				"Content-Type":"application/json",
				"Accept":"application/json",
			})

			if "Host" in self.session.headers:
				self.session.headers.pop("Host")

			if "Origin" in self.session.headers:
				self.session.headers.pop("Origin")

			if "Referer" in self.session.headers:
				self.session.headers.pop("Referer")

			print(self.session.headers)

	async def close(self):
		if not self.session:
			print("\n[!] Not a session owner")
			return

		await self.session.close()

	async def __aenter__(self):
		return self

	async def __aexit__(self,*excinfo):
		await self.close()

	async def get_item(self,gname,index,session=None):

		if session==None:
			s=self.session
		else:
			s=session

		res=await rque_api(session=s,api_path=f"/sel/{gname}/{index}",rque_home=self.addr,rque_port=self.port)
		return res

	async def get_group(self,gname):
		if session==None:
			s=self.session
		else:
			s=session

		res=await rque_api(session=s,api_path=f"/sel/{gname}",rque_home=self.addr,rque_port=self.port)
		return res

	async def get_range(self,gname,index,qtty,session=None):
		if session==None:
			s=self.session
		else:
			s=session

		res=await rque_api(session=s,api_path=f"/sel/{gname}/{index}/{qtty}",rque_home=self.addr,rque_port=self.port)
		return res

	async def add_sin(self,gname,item,session=None):
		if session==None:
			s=self.session
		else:
			s=session

		res=await rque_api(session=s,the_method="POST",the_json={"name":gname,"item":item},api_path="/add/sin",rque_home=self.addr,rque_port=self.port)
		return res

	async def add_mul(self,gname,item_list,session=None):
		if session==None:
			s=self.session
		else:
			s=session

		res=await rque_api(session=s,the_method="POST",the_json={"name":gname,"list":item_list},api_path="/add/mul",rque_home=self.addr,rque_port=self.port)
		return res

	async def del_item(self,gname,index,session=None):
		if session==None:
			s=self.session
		else:
			s=session

		res=await rque_api(session=s,the_method="DELETE",api_path=f"/sel/{gname}/{index}",rque_home=self.addr,rque_port=self.port)
		return res

	async def del_range(self,gname,index,qtty,session=None):
		if session==None:
			s=self.session
		else:
			s=session

		res=await rque_api(session=s,the_method="DELETE",api_path=f"/sel/{gname}/{index}/{qtty}",rque_home=self.addr,rque_port=self.port)
		return res
