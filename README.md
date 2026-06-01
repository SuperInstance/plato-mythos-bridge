# plato-mythos-bridge

> Seven cultural math traditions wired into PLATO's monitoring, health, and routing systems

## What This Does

plato-mythos-bridge bridges PLATO's technical monitoring pipeline with mathematical traditions from cultures around the world. Each tradition solves a real PLATO problem — graph pathfinding, exponential decay memory, consensus negotiation, hierarchical encoding, symmetry detection, and fault-tolerant health scoring — with a metaphor that makes the solution intuitive.

This isn't decorative. The cultural framing makes code intent legible at a glance. A room that remembers history with exponential decay could be called `ExponentialDecayEventStore`. Calling it `GriotMemory` — after West African storytellers who preserve oral history across generations — tells you memory decays but traditions persist.

## Install

```bash
cargo add plato-mythos-bridge
```

## Quick Start

```rust
use plato_mythos_bridge::*;

// Kintsugi: faults are golden repairs, not failures
let mut kintsugi = KintsugiHealth::new();
let seam = kintsugi.health_as_repair(&HealthReport {
    component: "room-A".into(),
    status: kintsugi_health::HealthStatus::Faulted,
    message: "Disk failure".into(),
    metric_value: None,
});
println!("Beauty: {:.2}", seam.beauty());

// Griot: rooms remember events, traditions score health
let mut griot = GriotMemory::new(0.1);
griot.record("room-A", "health_check".into());
griot.record("room-A", "health_check".into());
println!("Health: {:.2}", griot.system_health());
```

## API Reference

### Kintsugi Health — Errors as Golden Seams

| Type | Description |
|---|---|
| `GoldenSeam { severity, position, repair_quality, ... }` | A repaired fault. Beauty = 0.4×severity + 0.6×quality |
| `HealthReport { component, status, message }` | PLATO health report input |
| `HealthStatus` | `Healthy` / `Degraded` / `Faulted` / `Unknown` |
| `KintsugiHealth` | Analyzer. `health_as_repair()`, `system_beauty()`, `severity_distribution()` |

### Quipu Tile — Incan Knot Encoding

| Type | Description |
|---|---|
| `Knot::Digit(u8)` / `Knot::Branch(CordTree)` | Hierarchical digit encoding |
| `CordTree { knots, checksum }` | Knot sequence with corruption detection |
| `QuipuTile` | Encoder/decoder with configurable decimal precision |

```rust
let q = QuipuTile::new(2);
let cord = q.encode_tiles(&[1.23, 4.56]);
assert!(cord.validate());
let decoded = q.decode_tiles(&cord).unwrap();
```

### Songline Navigation — Aboriginal Pathfinding

| Type | Description |
|---|---|
| `SonglineGraph` | Bidirectional room graph with traversal counts and latency |
| `SonglineNavigation` | Pathfinding that prefers well-traveled routes (cost = latency / traversal_count) |

### Griot Memory — West African Oral History

| Type | Description |
|---|---|
| `RoomMemory` | Per-room event memory with exponential decay. `tradition_score()` checks if event types recur. |
| `GriotMemory` | Multi-room manager. `system_health()` = avg tradition score. |

### Palaver Consensus — African Negotiation

| Type | Description |
|---|---|
| `RoomConfig` | Key-value config proposal |
| `RoomConsensus` | Negotiation engine (50% convergence per round toward mean) |
| `PalaverConsensus::create_consensus(threshold)` | Factory |

### Symmetry Detection — Islamic Geometry

| Type | Description |
|---|---|
| `SymmetryGroup` | `None` / `Mirror` / `Translational` / `Rotational` / `Combined` |
| `SymmetryDetection` | `detect_metric_symmetry()` — symmetry = stability = health |

## Testing

58 tests covering all six modules: Kintsugi beauty/scoring/distribution, Quipu encode/decode/checksum, Songline pathfinding/preference, Griot decay/traditions/health, Palaver convergence, Symmetry detection.

## License

MIT
