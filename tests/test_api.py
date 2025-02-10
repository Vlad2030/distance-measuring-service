import os
import string
import uuid
import random
import http

import dotenv
import requests

dotenv.load_dotenv()

backend_host = os.getenv("SERVICE_HOST")
backend_port = os.getenv("SERVICE_PORT")

backend_url = f"http://{backend_host}:{backend_port if backend_port else 80}"


def test_get_healthcheck() -> None:
    healthcheck = requests.get(backend_url + "/health/")
    assert healthcheck.status_code == http.HTTPStatus.NOT_FOUND

    healthcheck = requests.get(backend_url + "/health")
    assert healthcheck.status_code == http.HTTPStatus.OK
    assert healthcheck.json()["successful"] == True


def test_get_units() -> None:
    units = requests.get(backend_url + "/units/")
    assert units.status_code == http.HTTPStatus.NOT_FOUND

    units = requests.get(backend_url + "/units")
    assert units.status_code == http.HTTPStatus.OK
    assert isinstance(units.json()["units"], list) == True
    assert len(units.json()["units"]) > 0


def test_get_distance() -> None:
    distance = requests.get(backend_url + "/distance/")
    assert distance.status_code == http.HTTPStatus.NOT_FOUND

    distance = requests.post(backend_url + "/distance/")
    assert distance.status_code == http.HTTPStatus.NOT_FOUND


def test_get_distance_missing_json() -> None:
    distance = requests.post(
        backend_url + "/distance",
        json={},
    )
    assert distance.status_code == http.HTTPStatus.INTERNAL_SERVER_ERROR


def test_get_distance_one_point() -> None:
    distance = requests.post(
        backend_url + "/distance",
        json={
            "points": [
                {
                    "latitude": 1,
                    "longitude": 1,
                },
            ],
            "unit": "Kilometers",
        },
    )
    assert distance.status_code == http.HTTPStatus.BAD_REQUEST


def test_get_distance_missing_units() -> None:
    distance = requests.post(
        backend_url + "/distance",
        json={
            "points": [
                {
                    "latitude": 0,
                    "longitude": 0,
                },
                {
                    "latitude": 1,
                    "longitude": 1,
                },
            ],
        },
    )
    assert distance.status_code == http.HTTPStatus.INTERNAL_SERVER_ERROR


def test_get_distance_km() -> None:
    distance = requests.post(
        backend_url + "/distance",
        json={
            "points": [
                {
                    "latitude": 50.675462,
                    "longitude": 3.079845,
                },
                {
                    "latitude": 50.673461,
                    "longitude": 3.083647,
                },
                # aprox 350m,
            ],
            "unit": "Kilometers",
        },
    )
    assert distance.status_code == http.HTTPStatus.OK
    assert distance.json()["distance"] >= 0.3478125  # -0.625%
    assert distance.json()["distance"] <= 0.3521875  # +0.625%


def test_get_distance_mi() -> None:
    distance = requests.post(
        backend_url + "/distance",
        json={
            "points": [
                {
                    "latitude": 50.675462,
                    "longitude": 3.079845,
                },
                {
                    "latitude": 50.673461,
                    "longitude": 3.083647,
                },
                # aprox 350m,
            ],
            "unit": "Miles",
        },
    )
    assert distance.status_code == http.HTTPStatus.OK
    assert distance.json()["distance"] >= 0.21612075  # -0.625%
    assert distance.json()["distance"] <= 0.21883925  # +0.625%
