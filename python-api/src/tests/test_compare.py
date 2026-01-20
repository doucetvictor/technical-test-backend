from fastapi.testclient import TestClient
from src.main import app

client = TestClient(app)

def test_compare_valid():
    response = client.get("/v1/compare?start_date=2023-01-01&end_date=2023-01-31")
    assert response.status_code == 200
    assert response.json() == {"message": "Dates are valid"}

def test_compare_invalid_start_date():
    response = client.get("/v1/compare?start_date=invalid&end_date=2023-01-31")
    assert response.status_code == 400
    assert response.text == "start_date is invalid"

def test_compare_invalid_end_date():
    response = client.get("/v1/compare?start_date=2023-01-01&end_date=invalid")
    assert response.status_code == 400
    assert response.text == "end_date is invalid"

def test_compare_start_after_end():
    response = client.get("/v1/compare?start_date=2023-02-01&end_date=2023-01-31")
    assert response.status_code == 400
    assert response.text == "start_date is after end_date"
