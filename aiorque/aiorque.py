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

async def rque_api(the_method="GET",api_path="/all",the_json=None,session=None,rque_home=_rque_default_addr,rque_port=_rque_default_port):

	local=False
	if not session:
		local=True
		session=aiohttp.ClientSession()
		session.headers.update({"Content-Type":"application/json","Accept":"application/json",
		})

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

async def rque_get_all(rque_home=_rque_default_addr,rque_port=_rque_default_port,session=None):
	res=await rque_api(session=session,rque_home=rque_home,rque_port=rque_port)
	return res

async def rque_get_group(group_name,rque_home=_rque_default_addr,rque_port=_rque_default_port,session=None):
	res=await rque_api(session=session,api_path=f"/sel/{group_name}",rque_home=rque_home,rque_port=rque_port)
	return res

async def rque_get_item(group_name,index,rque_home=_rque_default_addr,rque_port=_rque_default_port,session=None):
	res=await rque_api(session=session,api_path=f"/sel/{group_name}/{index}",rque_home=rque_home,rque_port=rque_port)
	return res

async def rque_get_range(group_name,index,qtty,rque_home=_rque_default_addr,rque_port=_rque_default_port,session=None):
	res=await rque_api(session=session,api_path=f"/sel/{group_name}/{index}/{qtty}",rque_home=rque_home,rque_port=rque_port)
	return res

async def rque_addsin(group_name,item,rque_home=_rque_default_addr,rque_port=_rque_default_port,session=None):
	res=await rque_api(session=session,the_method="POST",the_json={"name":group_name,"item":item},api_path="/add/sin",rque_home=rque_home,rque_port=rque_port)
	return res

async def rque_addmul(group_name,item_list,rque_home=_rque_default_addr,rque_port=_rque_default_port,session=None):
	res=await rque_api(session=session,the_method="POST",the_json={"name":group_name,"list":item_list},api_path="/add/mul",rque_home=rque_home,rque_port=rque_port)
	return res

async def rque_del_all(rque_home=_rque_default_addr,rque_port=_rque_default_port,session=None):
	res=await rque_api(session=session,the_method="DELETE",rque_port=rque_port)
	return res

async def rque_del_group(group_name,rque_home=_rque_default_addr,rque_port=_rque_default_port,session=None):
	res=await rque_api(session=session,the_method="DELETE",api_path=f"/sel/{group_name}",rque_home=rque_home,rque_port=rque_port)
	return res

async def rque_del_item(group_name,index,rque_home=_rque_default_addr,rque_port=_rque_default_port,session=None):
	res=await rque_api(session=session,the_method="DELETE",api_path=f"/sel/{group_name}/{index}",rque_home=rque_home,rque_port=rque_port)
	return res

async def rque_del_range(group_name,index,qtty,rque_home=_rque_default_addr,rque_port=_rque_default_port,session=None):
	res=await rque_api(session=session,the_method="DELETE",api_path=f"/sel/{group_name}/{index}/{qtty}",rque_home=rque_home,rque_port=rque_port)
	return res

class rque_Client:

	def __init__(self,session=None,addr=_rque_default_addr,port=_rque_default_port):
		self.addr=addr
		self.port=port

		if session:
			self.session=session
			self.owner=False

		if not session:
			self.session=aiohttp.ClientSession()
			self.prep_session()
			self.owner=True

	def prep_session(self):
		self.session.headers.update({
			"Content-Type":"application/json",
			"Accept":"application/json",
		})
		if "Host" in session.headers:
			session.headers.pop("Host")

		if "Origin" in session.headers:
			session.headers.pop("Origin")

		if "Referer" in session.headers:
			session.headers.pop("Referer")

	async def close(self):
		if not self.owner:
			print("\n[!] Not the owner of this session")
			return

		if self.session:
			await self.session.close()
			print("\n[!] Client closed")
		if not self.session:
			print("\n[!] Nothing to close")

	async def __aenter__(self):
		return self

	async def __aexit__(self,*excinfo):
		await self.close()

	async def get_item(self,gname,index):
		res=await rque_api(session=self.session,api_path=f"/sel/{gname}/{index}",rque_home=self.addr,rque_port=self.port)
		return res

	async def get_group(self,gname):
		res=await rque_api(session=self.session,api_path=f"/sel/{gname}",rque_home=self.addr,rque_port=self.port)
		return res

	async def get_range(self,gname,index,qtty):
		res=await rque_api(session=self.session,api_path=f"/sel/{gname}/{index}/{qtty}",rque_home=self.addr,rque_port=self.port)
		return res

	async def add_sin(self,gname,item):
		res=await rque_api(session=self.session,the_method="POST",the_json={"name":gname,"item":item},api_path="/add/sin",rque_home=self.addr,rque_port=self.port)
		return res

	async def add_mul(self,gname,item_list):
		res=await rque_api(session=self.session,the_method="POST",the_json={"name":gname,"list":item_list},api_path="/add/mul",rque_home=self.addr,rque_port=self.port)
		return res

	async def del_item(self,gname,index):
		res=await rque_api(session=self.session,the_method="DELETE",api_path=f"/sel/{gname}/{index}",rque_home=self.addr,rque_port=self.port)
		return res

	async def del_range(self,gname,index,qtty):
		res=await rque_api(session=self.session,the_method="DELETE",api_path=f"/sel/{gname}/{index}/{qtty}",rque_home=self.addr,rque_port=self.port)
		return res
