from fastapi import APIRouter, Response
from datetime import datetime

router = APIRouter()

@router.get("/compare")
async def compare(start_date: str, end_date: str):
    try:
        start = datetime.strptime(start_date, "%Y-%m-%d")
    except ValueError:
        return Response(content="start_date is invalid", status_code=400)

    try:
        end = datetime.strptime(end_date, "%Y-%m-%d")
    except ValueError:
        return Response(content="end_date is invalid", status_code=400)

    if start > end:
        return Response(content="start_date is after end_date", status_code=400)

    return {"message": "Dates are valid"}
