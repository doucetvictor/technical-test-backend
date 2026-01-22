from fastapi import APIRouter, Response
from datetime import datetime
from src.celery import compute_average_price
from src.config import config
import json

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

    try:
        result = compute_average_price.apply_async(
            args=(start_date, end_date)
        )
        data = result.get(timeout=config["compute_average_price"]["timeout"])
        return Response(content=json.dumps(data), status_code=200, media_type="application/json")
    except Exception as e:
        return Response(content=str(e), status_code=500)
