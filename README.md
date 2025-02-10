# distance-measuring-service

Microservice to calculate distance between location points

## Endpoints

### GET `/units`

Available units to calculate

#### Response body

```json
{
    "units": ["Kilometers", "Miles"]
}
```

### POST /distance

Calculate distance

#### Request body

```json
{
    "points": [
        {
            "latitude": 0.0,
            "longitude": 0.0
        },
        {
            "latitude": 1.0,
            "longitude": 1.0
        },
        {
            "latitude": 0.0,
            "longitude": 0.0
        }
    ],
    "unit": "Kilometers"
}
```

#### Response body

```json
{
    "points": [
        {
            "latitude": 0,
            "longitude": 0
        },
        {
            "latitude": 1,
            "longitude": 1
        },
        {
            "latitude": 0,
            "longitude": 0
        }
    ],
    "unit": "Kilometers",
    "distance": 314.4992
}
```

## How to run

### Create `.env` file

example from `.env.example`:

```bash
SERVICE_TITLE=distance-measuring-service
SERVICE_VERSION=0.0.1
SERVICE_CONTAINER_NAME=distance-measuring-service
SERVICE_HOST=0.0.0.0
SERVICE_PORT=6969
SERVICE_WORKERS=8

LOG_LEVEL=info
```

### Start docker containers

```
docker-compose -f docker-compose.prod.yaml up -d
```

## Tests

```bash
pip3 install -r requirements.txt
cd tests && pytest -v
```
