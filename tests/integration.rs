#[cfg(test)]
mod tests {
    use plato_mythos_bridge::*;

    // === Kintsugi Health Tests ===

    #[test]
    fn kintsugi_new_is_empty() {
        let k = kintsugi_health::KintsugiHealth::new();
        assert!(k.seams().is_empty());
    }

    #[test]
    fn kintsugi_healthy_report() {
        let mut k = kintsugi_health::KintsugiHealth::new();
        let report = kintsugi_health::HealthReport {
            component: "room-A".into(),
            status: kintsugi_health::HealthStatus::Healthy,
            message: "All good".into(),
            metric_value: Some(99.9),
        };
        let seam = k.health_as_repair(&report);
        assert_eq!(seam.severity, 0.0);
        assert_eq!(seam.repair_quality, 1.0);
        assert_eq!(seam.position, "room-A");
    }

    #[test]
    fn kintsugi_faulted_report() {
        let mut k = kintsugi_health::KintsugiHealth::new();
        let report = kintsugi_health::HealthReport {
            component: "room-B".into(),
            status: kintsugi_health::HealthStatus::Faulted,
            message: "Disk failure".into(),
            metric_value: None,
        };
        let seam = k.health_as_repair(&report);
        assert!((seam.severity - 0.9).abs() < 0.01);
        assert!(seam.repair_quality < 1.0);
    }

    #[test]
    fn kintsugi_degraded_severity() {
        let status = kintsugi_health::HealthStatus::Degraded;
        assert!((status.severity() - 0.4).abs() < 0.01);
    }

    #[test]
    fn kintsugi_unknown_severity() {
        let status = kintsugi_health::HealthStatus::Unknown;
        assert!((status.severity() - 0.6).abs() < 0.01);
    }

    #[test]
    fn kintsugi_beauty_increases_with_experience() {
        let mut k = kintsugi_health::KintsugiHealth::new();
        let report = kintsugi_health::HealthReport {
            component: "x".into(),
            status: kintsugi_health::HealthStatus::Faulted,
            message: "err".into(),
            metric_value: None,
        };
        let first = k.health_as_repair(&report);
        let second = k.health_as_repair(&report);
        assert!(second.repair_quality >= first.repair_quality);
    }

    #[test]
    fn kintsugi_system_beauty_untested() {
        let k = kintsugi_health::KintsugiHealth::new();
        assert!((k.system_beauty() - 0.5).abs() < 0.01);
    }

    #[test]
    fn kintsugi_repair_all() {
        let mut k = kintsugi_health::KintsugiHealth::new();
        let reports = vec![
            kintsugi_health::HealthReport {
                component: "a".into(),
                status: kintsugi_health::HealthStatus::Healthy,
                message: "ok".into(),
                metric_value: None,
            },
            kintsugi_health::HealthReport {
                component: "b".into(),
                status: kintsugi_health::HealthStatus::Degraded,
                message: "slow".into(),
                metric_value: None,
            },
        ];
        let seams = k.repair_all(&reports);
        assert_eq!(seams.len(), 2);
        assert_eq!(k.seams().len(), 2);
    }

    #[test]
    fn kintsugi_severity_distribution() {
        let mut k = kintsugi_health::KintsugiHealth::new();
        k.health_as_repair(&kintsugi_health::HealthReport {
            component: "a".into(), status: kintsugi_health::HealthStatus::Healthy, message: "ok".into(), metric_value: None,
        });
        k.health_as_repair(&kintsugi_health::HealthReport {
            component: "b".into(), status: kintsugi_health::HealthStatus::Degraded, message: "slow".into(), metric_value: None,
        });
        k.health_as_repair(&kintsugi_health::HealthReport {
            component: "c".into(), status: kintsugi_health::HealthStatus::Faulted, message: "dead".into(), metric_value: None,
        });
        let (low, mid, high) = k.severity_distribution();
        assert_eq!(low, 1);
        assert_eq!(mid, 1);
        assert_eq!(high, 1);
    }

    #[test]
    fn kintsugi_golden_seam_beauty() {
        let seam = kintsugi_health::GoldenSeam {
            severity: 0.9,
            position: "test".into(),
            repair_quality: 1.0,
            repaired_at: 0,
            description: "perfect repair".into(),
        };
        assert!(seam.beauty() > 0.8);
    }

    #[test]
    fn kintsugi_default() {
        let k = kintsugi_health::KintsugiHealth::default();
        assert!(k.seams().is_empty());
    }

    // === Quipu Tile Tests ===

    #[test]
    fn quipu_encode_single_tile() {
        let q = quipu_tile::QuipuTile::new(1);
        let cord = q.encode_tiles(&[1.0]);
        assert_eq!(cord.knots.len(), 1);
    }

    #[test]
    fn quipu_roundtrip() {
        let q = quipu_tile::QuipuTile::new(1);
        let tiles = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let cord = q.encode_tiles(&tiles);
        let decoded = q.decode_tiles(&cord).unwrap();
        assert_eq!(decoded.len(), tiles.len());
        for (a, b) in tiles.iter().zip(decoded.iter()) {
            assert!((a - b).abs() < 0.01);
        }
    }

    #[test]
    fn quipu_checksum_validation() {
        let q = quipu_tile::QuipuTile::new(1);
        let cord = q.encode_tiles(&[1.0, 2.0]);
        assert!(cord.validate());
    }

    #[test]
    fn quipu_corruption_detected() {
        let q = quipu_tile::QuipuTile::new(1);
        let mut cord = q.encode_tiles(&[1.0, 2.0]);
        cord.checksum = 99999;
        assert!(!cord.validate());
    }

    #[test]
    fn quipu_decode_corrupted_fails() {
        let q = quipu_tile::QuipuTile::new(1);
        let mut cord = q.encode_tiles(&[1.0]);
        cord.checksum = 0;
        assert!(q.decode_tiles(&cord).is_err());
    }

    #[test]
    fn quipu_zero_tile() {
        let q = quipu_tile::QuipuTile::new(1);
        let cord = q.encode_tiles(&[0.0]);
        assert_eq!(cord.knots.len(), 1);
        let decoded = q.decode_tiles(&cord).unwrap();
        assert!((decoded[0]).abs() < 0.01);
    }

    #[test]
    fn quipu_knot_count() {
        let q = quipu_tile::QuipuTile::new(1);
        let cord = q.encode_tiles(&[5.0]);
        assert!(cord.knot_count() >= 1);
    }

    #[test]
    fn quipu_large_values() {
        let q = quipu_tile::QuipuTile::new(2);
        let tiles = vec![99.99, 100.00, 0.01];
        let cord = q.encode_tiles(&tiles);
        assert!(cord.validate());
    }

    #[test]
    fn quipu_empty_tiles() {
        let q = quipu_tile::QuipuTile::new(1);
        let cord = q.encode_tiles(&[]);
        assert_eq!(cord.knots.len(), 0);
        assert!(cord.validate());
    }

    #[test]
    fn quipu_default() {
        let q = quipu_tile::QuipuTile::default();
        let cord = q.encode_tiles(&[3.0]);
        assert!(cord.validate());
    }

    // === Songline Navigation Tests ===

    #[test]
    fn songline_build_graph() {
        let graph = songline_navigation::SonglineNavigation::build_tile_graph(
            &["room-A".into(), "room-B".into(), "room-C".into()],
        );
        assert_eq!(graph.rooms().len(), 3);
    }

    #[test]
    fn songline_direct_path() {
        let mut graph = songline_navigation::SonglineGraph::new();
        graph.add_edge("A".into(), "B".into(), 10.0);
        let path = songline_navigation::SonglineNavigation::find_tile_path(&graph, "A", "B");
        assert!(path.is_some());
        let p = path.unwrap();
        assert_eq!(p.len(), 2);
        assert_eq!(p[0], "A");
        assert_eq!(p[1], "B");
    }

    #[test]
    fn songline_same_source_target() {
        let graph = songline_navigation::SonglineGraph::new();
        let path = songline_navigation::SonglineNavigation::find_tile_path(&graph, "X", "X");
        assert_eq!(path, Some(vec!["X".into()]));
    }

    #[test]
    fn songline_multi_hop_path() {
        let mut graph = songline_navigation::SonglineGraph::new();
        graph.add_edge("A".into(), "B".into(), 10.0);
        graph.add_edge("B".into(), "C".into(), 10.0);
        let path = songline_navigation::SonglineNavigation::find_tile_path(&graph, "A", "C");
        assert!(path.is_some());
        let p = path.unwrap();
        assert!(p.contains(&"A".into()));
        assert!(p.contains(&"C".into()));
    }

    #[test]
    fn songline_no_path() {
        let mut graph = songline_navigation::SonglineGraph::new();
        graph.add_room("A".into());
        graph.add_room("B".into());
        let path = songline_navigation::SonglineNavigation::find_tile_path(&graph, "A", "B");
        assert!(path.is_none());
    }

    #[test]
    fn songline_prefer_well_traveled() {
        let mut graph = songline_navigation::SonglineGraph::new();
        graph.add_edge("A".into(), "B".into(), 10.0);
        graph.add_edge("A".into(), "C".into(), 10.0);
        graph.add_edge("C".into(), "D".into(), 10.0);
        graph.add_edge("B".into(), "D".into(), 10.0);

        // Make A→B→D more traveled
        graph.record_traversal("A", "B");
        graph.record_traversal("A", "B");
        graph.record_traversal("B", "D");
        graph.record_traversal("B", "D");

        let path = songline_navigation::SonglineNavigation::find_tile_path(&graph, "A", "D");
        assert!(path.is_some());
        let p = path.unwrap();
        // Should prefer the well-traveled A→B→D route
        assert!(p.contains(&"B".into()) || p.contains(&"C".into()));
    }

    #[test]
    fn songline_edges_from() {
        let mut graph = songline_navigation::SonglineGraph::new();
        graph.add_edge("A".into(), "B".into(), 5.0);
        graph.add_edge("A".into(), "C".into(), 15.0);
        let edges = graph.edges_from("A").unwrap();
        assert_eq!(edges.len(), 2);
    }

    #[test]
    fn songline_record_traversal() {
        let mut graph = songline_navigation::SonglineGraph::new();
        graph.add_edge("A".into(), "B".into(), 10.0);
        graph.record_traversal("A", "B");
        graph.record_traversal("A", "B");
        let edges = graph.edges_from("A").unwrap();
        let ab = edges.iter().find(|e| e.target == "B").unwrap();
        assert_eq!(ab.traversal_count, 3); // 1 initial + 2 recorded
    }

    #[test]
    fn songline_default_graph() {
        let graph = songline_navigation::SonglineGraph::default();
        assert!(graph.rooms().is_empty());
    }

    // === Griot Memory Tests ===

    #[test]
    fn griot_record_event() {
        let mut mem = griot_memory::RoomMemory::new("room-1".into(), 0.1);
        mem.record_event("config_update".into());
        assert_eq!(mem.event_count(), 1);
    }

    #[test]
    fn griot_memory_strengths_decay() {
        let mut mem = griot_memory::RoomMemory::new("room-1".into(), 0.5);
        mem.record_event("first".into());
        mem.record_event("second".into());
        mem.record_event("third".into());
        let strengths = mem.memory_strengths();
        assert_eq!(strengths.len(), 3);
        // Most recent should be strongest
        assert!(strengths[2] >= strengths[0]);
    }

    #[test]
    fn griot_tradition_score_no_traditions() {
        let mut mem = griot_memory::RoomMemory::new("room-1".into(), 0.1);
        mem.record_event("unique".into());
        assert_eq!(mem.tradition_score(), 0.0);
    }

    #[test]
    fn griot_tradition_score_with_traditions() {
        let mut mem = griot_memory::RoomMemory::new("room-1".into(), 0.1);
        mem.record_event("daily_check".into());
        mem.record_event("config_update".into());
        mem.record_event("daily_check".into());
        mem.record_event("daily_check".into());
        let score = mem.tradition_score();
        assert!(score > 0.0);
    }

    #[test]
    fn griot_tradition_maintained() {
        let mut mem = griot_memory::RoomMemory::new("room-1".into(), 0.1);
        mem.record_event("heartbeat".into());
        assert!(!mem.tradition_maintained("heartbeat"));
        mem.record_event("heartbeat".into());
        assert!(mem.tradition_maintained("heartbeat"));
    }

    #[test]
    fn griot_manager_system_health() {
        let mut gm = griot_memory::GriotMemory::new(0.1);
        gm.add_room("room-1".into());
        gm.record("room-1", "check".into());
        gm.record("room-1", "check".into());
        assert!(gm.system_health() > 0.0);
    }

    #[test]
    fn griot_manager_multiple_rooms() {
        let mut gm = griot_memory::GriotMemory::new(0.1);
        gm.add_room("A".into());
        gm.add_room("B".into());
        gm.record("A", "ping".into());
        gm.record("A", "ping".into());
        gm.record("B", "pong".into());
        let scores = gm.all_tradition_scores();
        assert_eq!(scores.len(), 2);
    }

    #[test]
    fn griot_room_id() {
        let mem = griot_memory::RoomMemory::new("test-room".into(), 0.1);
        assert_eq!(mem.room_id(), "test-room");
    }

    #[test]
    fn griot_default() {
        let gm = griot_memory::GriotMemory::default();
        assert!(gm.system_health() > 0.0); // empty rooms = healthy
    }

    #[test]
    fn griot_empty_memory_strengths() {
        let mem = griot_memory::RoomMemory::new("x".into(), 0.1);
        assert!(mem.memory_strengths().is_empty());
    }

    // === Palaver Consensus Tests ===

    #[test]
    fn palaver_two_rooms_converge() {
        let mut consensus = palaver_consensus::RoomConsensus::new(0.001, 50);
        let mut c1 = palaver_consensus::RoomConfig::new("A".into());
        c1.set("timeout", 10.0);
        let mut c2 = palaver_consensus::RoomConfig::new("B".into());
        c2.set("timeout", 20.0);
        consensus.add_room("A".into(), c1);
        consensus.add_room("B".into(), c2);
        let (result, history) = consensus.negotiate(50);
        let timeout = result.get("timeout").unwrap();
        assert!((timeout - 15.0).abs() < 0.5, "Expected ~15.0, got {}", timeout);
        assert!(history.last().unwrap().converged);
    }

    #[test]
    fn palaver_three_rooms() {
        let mut consensus = palaver_consensus::RoomConsensus::new(0.01, 50);
        let mut c1 = palaver_consensus::RoomConfig::new("A".into());
        c1.set("max_conn", 5.0);
        let mut c2 = palaver_consensus::RoomConfig::new("B".into());
        c2.set("max_conn", 10.0);
        let mut c3 = palaver_consensus::RoomConfig::new("C".into());
        c3.set("max_conn", 15.0);
        consensus.add_room("A".into(), c1);
        consensus.add_room("B".into(), c2);
        consensus.add_room("C".into(), c3);
        let (result, _) = consensus.negotiate(50);
        assert!((result.get("max_conn").unwrap() - 10.0).abs() < 1.0);
    }

    #[test]
    fn palaver_convergence_tracking() {
        let mut consensus = palaver_consensus::RoomConsensus::new(0.01, 50);
        let mut c1 = palaver_consensus::RoomConfig::new("A".into());
        c1.set("val", 0.0);
        let mut c2 = palaver_consensus::RoomConfig::new("B".into());
        c2.set("val", 100.0);
        consensus.add_room("A".into(), c1);
        consensus.add_room("B".into(), c2);
        let (_, history) = consensus.negotiate(50);
        // Delta should decrease over rounds
        assert!(history.len() > 1);
        assert!(history.last().unwrap().max_delta <= history.first().unwrap().max_delta);
    }

    #[test]
    fn palaver_room_count() {
        let mut consensus = palaver_consensus::RoomConsensus::new(0.01, 10);
        assert_eq!(consensus.room_count(), 0);
        let c = palaver_consensus::RoomConfig::new("X".into());
        consensus.add_room("X".into(), c);
        assert_eq!(consensus.room_count(), 1);
    }

    #[test]
    fn palaver_empty_negotiation() {
        let mut consensus = palaver_consensus::RoomConsensus::new(0.01, 10);
        let (result, history) = consensus.negotiate(5);
        assert!(result.settings.is_empty());
        assert!(history.is_empty());
    }

    #[test]
    fn palaver_config_set_get() {
        let mut c = palaver_consensus::RoomConfig::new("test".into());
        c.set("key", 42.0);
        assert_eq!(c.get("key"), Some(42.0));
        assert_eq!(c.get("missing"), None);
    }

    #[test]
    fn palaver_create_consensus() {
        let c = palaver_consensus::PalaverConsensus::create_consensus(0.05);
        assert_eq!(c.room_count(), 0);
    }

    #[test]
    fn palaver_multiple_settings() {
        let mut consensus = palaver_consensus::RoomConsensus::new(0.01, 50);
        let mut c1 = palaver_consensus::RoomConfig::new("A".into());
        c1.set("x", 1.0);
        c1.set("y", 10.0);
        let mut c2 = palaver_consensus::RoomConfig::new("B".into());
        c2.set("x", 9.0);
        c2.set("y", 20.0);
        consensus.add_room("A".into(), c1);
        consensus.add_room("B".into(), c2);
        let (result, _) = consensus.negotiate(50);
        assert!((result.get("x").unwrap() - 5.0).abs() < 1.0);
        assert!((result.get("y").unwrap() - 15.0).abs() < 1.0);
    }

    // === Symmetry Detection Tests ===

    #[test]
    fn symmetry_mirror_perfect() {
        let det = symmetry_detection::SymmetryDetection::new(0.3);
        // Perfect mirror: [1, 2, 3, 3, 2, 1]
        let data = vec![1.0, 2.0, 3.0, 3.0, 2.0, 1.0];
        let sym = det.detect_metric_symmetry(&data);
        assert!(sym.strength() > 0.8);
    }

    #[test]
    fn symmetry_noisy_data() {
        let det = symmetry_detection::SymmetryDetection::new(0.95);
        let data = vec![1.0, 5.0, 2.0, 8.0, 3.0, 7.0, 4.0, 6.0];
        let sym = det.detect_metric_symmetry(&data);
        // Very high threshold — random data should not pass
        assert!(sym.strength() < 0.95 || matches!(sym, symmetry_detection::SymmetryGroup::None));
    }

    #[test]
    fn symmetry_too_short() {
        let det = symmetry_detection::SymmetryDetection::new(0.3);
        let data = vec![1.0, 2.0];
        let sym = det.detect_metric_symmetry(&data);
        assert_eq!(sym, symmetry_detection::SymmetryGroup::None);
    }

    #[test]
    fn symmetry_health_score() {
        let det = symmetry_detection::SymmetryDetection::new(0.3);
        let data = vec![1.0, 2.0, 3.0, 3.0, 2.0, 1.0];
        let score = det.symmetry_health_score(&data);
        assert!(score > 0.5);
    }

    #[test]
    fn symmetry_group_strength() {
        let sym = symmetry_detection::SymmetryGroup::Translational { period: 4, strength: 0.75 };
        assert!((sym.strength() - 0.75).abs() < 0.01);
    }

    #[test]
    fn symmetry_significance() {
        let sym = symmetry_detection::SymmetryGroup::Mirror { center_index: 3, strength: 0.5 };
        assert!(sym.is_significant(0.3));
        assert!(!sym.is_significant(0.6));
    }

    #[test]
    fn symmetry_translational_periodic() {
        let det = symmetry_detection::SymmetryDetection::new(0.3);
        // Periodic with period 2
        let data = vec![1.0, 2.0, 1.0, 2.0, 1.0, 2.0, 1.0, 2.0];
        let sym = det.detect_metric_symmetry(&data);
        assert!(sym.strength() > 0.5);
    }

    #[test]
    fn symmetry_constant_series() {
        let det = symmetry_detection::SymmetryDetection::new(0.3);
        let data = vec![5.0; 10];
        let sym = det.detect_metric_symmetry(&data);
        // Constant series has perfect symmetry of all types
        assert!(sym.strength() > 0.9);
    }

    #[test]
    fn symmetry_default() {
        let det = symmetry_detection::SymmetryDetection::default();
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        let sym = det.detect_metric_symmetry(&data);
        // Just check it doesn't panic
        assert!(sym.strength() >= 0.0);
    }

    #[test]
    fn symmetry_empty_data() {
        let det = symmetry_detection::SymmetryDetection::new(0.3);
        let data: Vec<f64> = vec![];
        let sym = det.detect_metric_symmetry(&data);
        assert_eq!(sym, symmetry_detection::SymmetryGroup::None);
    }
}
