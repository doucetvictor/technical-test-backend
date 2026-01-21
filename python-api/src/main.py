from fastapi import FastAPI, Request, Response
from fastapi.exceptions import RequestValidationError
from .routes import compare
from starlette.exceptions import HTTPException as StarletteHTTPException

app = FastAPI()

@app.exception_handler(StarletteHTTPException)
async def http_exception_handler(request: Request, exc: StarletteHTTPException):
    return Response(status_code=exc.status_code)

@app.exception_handler(RequestValidationError)
async def validation_exception_handler(request: Request, exc: RequestValidationError):
    return Response(status_code=400)

app.include_router(compare.router, prefix="/v1")
