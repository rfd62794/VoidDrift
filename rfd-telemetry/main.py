from fastapi import FastAPI
from fastapi.middleware.cors import CORSMiddleware
from pydantic import BaseModel
from datetime import datetime, timezone
import uuid
import sqlite3
import json

DB_PATH = "telemetry.db"

def get_db():
    conn = sqlite3.connect(DB_PATH)
    conn.execute("PRAGMA journal_mode=WAL")
    conn.execute("""
        CREATE TABLE IF NOT EXISTS events (
            id TEXT PRIMARY KEY,
            event_type TEXT NOT NULL,
            timestamp TEXT NOT NULL,
            session_id TEXT NOT NULL,
            client_version TEXT DEFAULT 'unknown',
            platform TEXT DEFAULT 'unknown',
            meta TEXT DEFAULT '{}',
            received_at TEXT NOT NULL
        )
    """)
    conn.commit()
    return conn

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

events_log = []  # Deprecated — now using SQLite

@app.get("/health")
def health():
    return {"status": "nominal", "timestamp": datetime.utcnow().isoformat()}

@app.post("/v1/event")
def receive_event(event: Event):
    event_id = str(uuid.uuid4())
    received_at = datetime.now(timezone.utc).isoformat()

    with get_db() as conn:
        conn.execute(
            "INSERT INTO events VALUES (?,?,?,?,?,?,?,?)",
            (event_id, event.event_type, event.timestamp,
             event.session_id, event.client_version,
             event.platform, json.dumps(event.meta), received_at)
        )

    print(f"[EVENT] {event.event_type} | session={event.session_id[:8]} | platform={event.platform}")
    return {"status": "accepted", "event_id": event_id}

@app.get("/v1/events")
def list_events(limit: int = 50, platform: str = None, event_type: str = None):
    query = "SELECT * FROM events"
    filters = []
    params = []

    if platform:
        filters.append("platform = ?")
        params.append(platform)
    if event_type:
        filters.append("event_type = ?")
        params.append(event_type)
    if filters:
        query += " WHERE " + " AND ".join(filters)

    query += " ORDER BY received_at DESC LIMIT ?"
    params.append(limit)

    with get_db() as conn:
        rows = conn.execute(query, params).fetchall()

    return {"events": [dict(zip(
        ["id","event_type","timestamp","session_id",
         "client_version","platform","meta","received_at"], row
    )) for row in rows]}
