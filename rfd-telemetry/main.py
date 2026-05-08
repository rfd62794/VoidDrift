from fastapi import FastAPI
from fastapi.middleware.cors import CORSMiddleware
from pydantic import BaseModel
from datetime import datetime
import uuid

app = FastAPI(title="RFD-Telemetry", version="0.1.0")

app.add_middleware(
    CORSMiddleware,
    allow_origins=[
        "https://rdug627.itch.io",
        "http://localhost:8080",
        "https://voidrift.rfditservices.com",
    ],
    allow_methods=["POST", "OPTIONS"],
    allow_headers=["Content-Type", "X-RFD-Client-Version"],
    max_age=86400,
)

class Event(BaseModel):
    event_type: str
    timestamp: str
    session_id: str
    client_version: str = "unknown"
    platform: str = "unknown"
    meta: dict = {}

events_log = []  # In-memory for now — SQLite in Sprint 8 proper

@app.get("/health")
def health():
    return {"status": "nominal", "timestamp": datetime.utcnow().isoformat()}

@app.post("/v1/event")
def receive_event(event: Event):
    event_id = str(uuid.uuid4())
    events_log.append({"id": event_id, **event.dict()})
    print(f"[EVENT] {event.event_type} | session={event.session_id} | platform={event.platform}")
    return {"status": "accepted", "event_id": event_id}
