from fastapi import APIRouter, Response

router = APIRouter()

@router.get("/healthy")
async def healthy():
    return Response(status_code=200)
