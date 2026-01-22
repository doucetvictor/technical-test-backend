import os
from celery import Celery
import psycopg2
from src.config import config

celery_app = Celery(
    "tasks",
    broker=config["celery"]["broker_url"],
    backend=config["celery"]["result_backend"]
)

celery_app.conf.update(
    task_serializer="json",
    accept_content=["json"],
    result_serializer="json",
    timezone="UTC",
    enable_utc=True,
    worker_concurrency=config["celery"]["concurrency"],
    worker_prefetch_multiplier=1,
)


@celery_app.task
def compute_average_price(start_date: str, end_date: str):
    db_url = config["database"]["url"]
    conn = psycopg2.connect(db_url)
    cur = conn.cursor()

    try:
        query = """
            SELECT
                date_trunc('day', recorded_at) AS day,
                AVG(price) AS average_price
            FROM
                prices
            WHERE
                recorded_at >= %s AND recorded_at <= %s
            GROUP BY
                day
            ORDER BY
                day;
        """
        cur.execute(query, (start_date, end_date))
        results = cur.fetchall()
        return [
            {"day": row[0].strftime("%Y-%m-%d"), "average_price": float(row[1])}
            for row in results
        ]
    finally:
        cur.close()
        conn.close()

if __name__ == "__main__":
    celery_app.start()
