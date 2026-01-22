from fastapi.testclient import TestClient
from unittest.mock import patch, MagicMock
from src.main import app
import json

client = TestClient(app)

def create_mock_redis_task(result):
    mock_task = MagicMock()
    mock_task.get.return_value = result
    return mock_task

@patch('src.celery.compute_average_price.apply_async')
def test_compare_valid(mock_celery):
    mock_result = [
        {"day": "2023-01-01", "average_price": 100.5},
        {"day": "2023-01-02", "average_price": 101.2},
    ]
    mock_celery.return_value = create_mock_redis_task(mock_result)

    response = client.get("/v1/compare?start_date=2023-01-01&end_date=2023-01-31")

    assert response.status_code == 200
    data = response.json()
    assert len(data) == 2
    assert data[0]["day"] == "2023-01-01"
    assert data[0]["average_price"] == 100.5
    assert mock_celery.called

@patch('src.celery.compute_average_price.apply_async')
def test_compare_invalid_start_date(mock_celery):
    mock_celery.return_value = create_mock_redis_task([])

    response = client.get("/v1/compare?start_date=invalid&end_date=2023-01-31")
    assert response.status_code == 400
    assert response.text == "start_date is invalid"
    assert not mock_celery.called

@patch('src.celery.compute_average_price.apply_async')
def test_compare_invalid_end_date(mock_celery):
    mock_celery.return_value = create_mock_redis_task([])

    response = client.get("/v1/compare?start_date=2023-01-01&end_date=invalid")
    assert response.status_code == 400
    assert response.text == "end_date is invalid"
    assert not mock_celery.called

@patch('src.celery.compute_average_price.apply_async')
def test_compare_start_after_end(mock_celery):
    mock_celery.return_value = create_mock_redis_task([])

    response = client.get("/v1/compare?start_date=2023-02-01&end_date=2023-01-31")
    assert response.status_code == 400
    assert response.text == "start_date is after end_date"
    assert not mock_celery.called
