#!/usr/bin/python3.9

import asyncio
import aiohttp

from aiorque import rque_Client

async def job(item):

	head,tail=item
	await asyncio.sleep(1)
	print("head =",head)
	await asyncio.sleep(1)
	print("tail =",tail)

async def queue_next(rclient,qname):
	res=await rclient.del_item(qname,0)
	if not res.get("status")==200:
		return []

	return res.get("item")

async def main():

	client=rque_Client()

	print("\nCreate a small queue")

	qn="Queue1"

	await client.add_mul(qn,[ ["item1","metadata"] , ["item2","thrash"] , ["item3","ok"] , ["item4","somedata"] ])

	await asyncio.sleep(1)

	print("Consume all items in a loop")

	while True:
		await asyncio.sleep(1)
		item=await queue_next(client,qn)
		if not item:
			break

		await job(item)

	print("\nReached the end. There are no items left in the queue")

	await client.close()

asyncio.run(main())
