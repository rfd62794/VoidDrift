# ADR-014: Opt-in anonymous telemetry via FastAPI/SQLite
**Date:** May 2026
**Status:** Accepted

## Context
To understand how players interact with the game and improve the experience, we need telemetry data. However, privacy is a core value — players must have full control over whether their data is collected. The game targets both Android (mobile) and WASM (web), requiring a platform-agnostic solution.

Initial considerations:
- Third-party analytics (Google Analytics, Firebase) require account setup, have privacy concerns, and add external dependencies
- Direct HTTP POST to a simple endpoint is lightweight but requires infrastructure
- Local-only storage (SQLite) doesn't provide aggregate insights across players
- Mandatory telemetry violates our privacy-first principle

## Decision
Implement an opt-in anonymous telemetry system with the following architecture:
- **Client-side**: Rust-based telemetry client in the game that collects anonymous session data
- **Consent**: Explicit opt-in prompt shown to players (can be declined, changed later)
- **Server**: FastAPI backend receiving telemetry POST requests
- **Storage**: SQLite database for session data (simple, file-based, no external DB required)
- **Data collected**: Session duration, platform (Android/WASM), version number, basic gameplay milestones (first drone built, first ore refined)
- **No PII**: No device IDs, no IP addresses, no user-identifiable information

## Rationale
This approach provides:
- **Privacy-first**: Explicit opt-in with ability to revoke consent at any time
- **Lightweight**: FastAPI + SQLite is minimal infrastructure (no cloud services, no accounts)
- **Platform-agnostic**: Works identically on Android and WASM via HTTP POST
- **Anonymous**: No PII, no tracking across sessions, no device fingerprinting
- **Actionable insights**: Aggregate data helps understand session length, platform breakdown, progression bottlenecks
- **Self-hosted**: Full control over data, no third-party dependencies
- **Simple**: SQLite file-based storage requires no DB server setup

## Consequences
- **Positive**: Respects player privacy while gathering useful data
- **Positive**: Minimal infrastructure overhead (FastAPI server + SQLite file)
- **Positive**: No external service dependencies or account management
- **Positive**: Data ownership — we control the database and can delete it anytime
- **Constraint**: Requires hosting a FastAPI server (can be low-cost VPS or local)
- **Constraint**: Telemetry only works when online (offline sessions won't be reported)
- **Constraint**: Requires consent management in the UI (prompt, settings toggle)
- **Maintenance**: Server must be kept running and updated with new telemetry fields
