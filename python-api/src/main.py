from fastapi import FastAPI, Request, Response
from .routes import compare

app = FastAPI()

@app.exception_handler(404)
async def custom_404_handler(request: Request, exc):
    return Response(status_code=404)

app.include_router(compare.router, prefix="/v1")
