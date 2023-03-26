#!/usr/bin/python3.9

import asyncio
import aiohttp

from aiorque import rque_Client

async def main():

	print("\n→ Using with a context manager")
	async with rque_Client(port=80) as client:
		await client.add_sin("queue1",["title","tail"])

	await asyncio.sleep(1)

	print("\n→ Opening new client...")
	client=rque_Client(port=80)

	await client.get_item("queue1",0)
	await asyncio.sleep(1)
	await client.del_item("queue1",0)

	print("\n→ Manually closing...")
	await client.close()

asyncio.run(main())
