import asyncio

import py_core.py_core


async def main():
    await py_core.py_core.start_ping_service("/ip4/159.223.200.148/tcp/36047")


if __name__ == '__main__':
    asyncio.run(main())
