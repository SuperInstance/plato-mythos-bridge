# plato-mythos-bridge

> Seven cultural math traditions wired into PLATO's monitoring, health, and routing systems

## What This Does

plato-mythos-bridge bridges PLATO's technical monitoring pipeline with mathematical traditions from cultures around the world. Each tradition solves a real PLATO problem — graph pathfinding, exponential decay memory, consensus negotiation, hierarchical encoding, symmetry detection, and fault-tolerant health scoring — with a metaphor that makes the solution intuitive.

This isn't decorative. The cultural framing makes code intent legible at a glance. A room that remembers history with exponential decay could be called `ExponentialDecayEventStore`. Calling it `GriotMemory` — after West African storytellers who preserve oral history across generations — tells you memory decays but traditions persist.

## The Key Idea

Every module maps a cultural metaphor to a concrete algorithm:

| Tradition | Culture | PLATO Problem | Algorithm |
|---|---|---|---|
| **Kintsugi** | Japanese | Fault tolerance | Error severity → golden repair beauty score |
| **Quipu** | Incan | Tile encoding | Knot-encoding with hierarchical checksum |
| **Songline** | Aboriginal Australian | Pathfinding | Graph routing that prefers well-traveled edges |
| **Griot** | West African | Event memory | Exponential-decay recall with tradition scoring |
| **Palaver** | African | Config consensus | Multi-round convergence toward mean |
| **Islamic Geometry** | Islamic | Anomaly detection | Symmetry in metrics = stability = health |

The cultural framing isn't decoration — it's compression. `GriotMemory` communicates "exponential decay + tradition persistence" faster than `ExponentialDecayEventStoreWithRecurringEventScoring`.

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

## How It Works

**Kintsugi Health**: Each `HealthReport` is scored by a repair metaphor. Faults generate `GoldenSeam` objects with `beauty = 0.4 × severity + 0.6 × repair_quality`. `system_beauty()` aggregates all seams. More faults → more seams → potentially more beauty, mirroring the kintsugi philosophy that breakage + repair makes something more beautiful.

**Quipu Tile**: Values are decomposed into `Knot::Digit` nodes arranged in `CordTree` structures (one cord per value). Each cord carries a checksum (sum of digits mod 10). `validate()` recomputes and compares — any corruption is detected. Encoding handles configurable decimal precision.

**Songline Navigation**: A `SonglineGraph` stores rooms as nodes with bidirectional edges carrying `traversal_count` and `latency_ms`. Pathfinding uses cost = `latency_ms / traversal_count` — well-traveled edges have lower effective cost, mimicking how Aboriginal songlines make frequently-sung routes more navigable.

**Griot Memory**: Each `RoomMemory` stores events with timestamps. Recall uses exponential decay: weight = e^(-λ × age). `tradition_score()` checks if certain event types recur (like griots preserving oral traditions across generations). `system_health()` averages tradition scores across rooms.

**Palaver Consensus**: `RoomConsensus` negotiates config values across rooms. Each round, values move 50% toward the group mean (configurable threshold). This converges to consensus over multiple rounds, mirroring the African palaver tradition of extended deliberation.

**Symmetry Detection**: Checks time-series metrics for mirror, translational, and rotational symmetry. Symmetric patterns indicate stable, predictable systems. `detect_metric_symmetry()` tries all types and returns the strongest match. Asymmetric metrics signal potential issues.

## The Math

- **Kintsugi Beauty**: B = 0.4 × severity + 0.6 × repair_quality. System beauty = mean across all golden seams.
- **Exponential Decay (Griot)**: w(t) = e^(-λΔt) where λ is the decay rate and Δt is age. Events older than ~10/λ contribute negligibly.
- **Tradition Score**: T = (number of recurring event types) / (total unique event types) for a room.
- **Songline Edge Cost**: cost(e) = latency_ms / traversal_count. Effective cost decreases with use — a reinforcement signal.
- **Palaver Convergence**: v_{t+1} = v_t + 0.5 × (μ - v_t) where μ is the group mean. Converges geometrically: error halves each round.
- **Mirror Symmetry**: S_m = 1 - (2/N) × Σ|x[i] - x[N-1-i]|. Perfect mirror → S_m = 1.0.
- **Translational Symmetry**: S_t = 1 - min_period_corr. Finds the period p minimizing cross-correlation between x[0..N-p] and x[p..N].
- **Rotational Symmetry**: S_r via circular cross-correlation at various shifts.

## Testing

58 tests covering all six modules: Kintsugi beauty/scoring/distribution, Quipu encode/decode/checksum, Songline pathfinding/preference, Griot decay/traditions/health, Palaver convergence, Symmetry detection.

## License

MIT
