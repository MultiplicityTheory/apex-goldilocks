// tests.rs
// =========
//
// Below is a collection of unit tests designed to comprehensively assess the
// fundamental functionality of the UOR Cognitive Stack. These tests focus on
// creating and manipulating charts, manifolds, concurrency schedulers, HPC
// operators, foundation models, embeddings, the kernel, and the overall
// cognitive stack pipeline.
//
// Note: These tests are minimal demonstrations. In a real project, you might
// add far more edge cases and domain-specific logic.

#[cfg(test)]
mod tests {
    use reality_engine::uor_framework::*;
    use reality_engine::uor_framework::foundation_model::NullFoundationModel;
    use reality_engine::uor_framework::kernel::UorKernel;

    // 1. Chart Tests
    // ---------------
    #[test]
    fn test_chart_creation_and_parsing() {
        let json_data = r#"{"key":"value"}"#;
        let chart = Chart::from_json("demo_chart", "1.0", json_data)
            .expect("Should create chart successfully");
        assert_eq!(chart.name, "demo_chart");
        assert_eq!(chart.version, "1.0");
        assert_eq!(chart.raw_json, json_data);

        // Test parsing stub
        chart.parse().expect("Should parse chart without errors");
    }

    #[test]
    fn test_empty_chart_fails() {
        let result = Chart::from_json("empty_chart", "1.0", "");
        assert!(result.is_err(), "Empty JSON should fail to create chart");
    }

    // 2. Manifold Tests
    // ------------------
    #[test]
    fn test_manifold_basic_operations() {
        let mut manifold = Manifold::new();
        assert_eq!(manifold.nodes.len(), 0);
        assert_eq!(manifold.edges.len(), 0);

        let node_a = ManifoldNode { id: "A".to_string(), data: "DataA".to_string() };
        let node_b = ManifoldNode { id: "B".to_string(), data: "DataB".to_string() };

        // Add nodes
        manifold.add_node(node_a);
        manifold.add_node(node_b);
        assert_eq!(manifold.nodes.len(), 2);

        // Add edge
        manifold.add_edge("A", "B").expect("Edge A->B should succeed");
        assert_eq!(manifold.edges.len(), 1);
        let adj = manifold.edges.get("A").unwrap();
        assert_eq!(adj.len(), 1);
        assert_eq!(adj[0], "B");
    }

    #[test]
    fn test_manifold_missing_nodes_for_edge() {
        let mut manifold = Manifold::new();
        // No nodes added
        let result = manifold.add_edge("X", "Y");
        assert!(result.is_err(), "Adding an edge without valid nodes should fail");
    }

    // 3. Concurrency Tests
    // ---------------------
    #[test]
    fn test_round_robin_scheduler() {
        let mut scheduler = RoundRobinScheduler::default();
        let manifold = Manifold::new();
        let result = scheduler.schedule(&manifold);
        assert!(result.is_ok(), "RoundRobinScheduler scheduling should succeed in this stub test");
    }

    // 4. HPC Operator Tests
    // ----------------------
    #[test]
    fn test_example_operator() {
        let operator = ExampleOperator::default();
        let manifold_in = Manifold::new();
        let result = operator.apply(&manifold_in);
        assert!(result.is_ok(), "Applying ExampleOperator should succeed");
        let manifold_out = result.unwrap();
        // By default, ExampleOperator returns the input manifold unchanged
        assert_eq!(manifold_in.nodes.len(), manifold_out.nodes.len());
        assert_eq!(manifold_in.edges.len(), manifold_out.edges.len());
    }

    // 5. Foundation Model Tests
    // --------------------------
    #[test]
    fn test_null_foundation_model() {
        let mut model = NullFoundationModel::default();
        let manifold_in = Manifold::new();
        let result = model.process_manifold(&manifold_in);
        assert!(result.is_ok());
        let manifold_out = result.unwrap();
        // Null model returns the input unchanged
        assert_eq!(manifold_in.nodes.len(), manifold_out.nodes.len());
        assert_eq!(manifold_in.edges.len(), manifold_out.edges.len());
    }

    // 6. Kernel Tests
    // ----------------
    #[test]
    fn test_uor_kernel() {
        let mut kernel = UorKernel::new();
        let manifold_in = Manifold::new();
        let result = kernel.process_manifold(&manifold_in);
        assert!(result.is_ok());
        let manifold_out = result.unwrap();
        // By default, UorKernel also returns the input unchanged
        assert_eq!(manifold_in.nodes.len(), manifold_out.nodes.len());
        assert_eq!(manifold_in.edges.len(), manifold_out.edges.len());
    }

    // 7. Embedding Tests
    // -------------------
    #[test]
    fn test_default_quaternion_embedding() {
        let embedding = DefaultQuaternionEmbedding::default();
        let mut cortex = MemoryCortex::default();
        let manifold = Manifold::new();
        let quaternions = embedding.embed_manifold(&manifold, &mut cortex)
            .expect("Embedding should succeed");
        // The default stub returns a single quaternion
        assert_eq!(quaternions.len(), 1);
        let q = &quaternions[0];
        assert_eq!(q.w, 1.0);
        assert_eq!(q.x, 0.0);
        assert_eq!(q.y, 0.0);
        assert_eq!(q.z, 0.0);
    }

    // 8. Memory Cortex Tests
    // -----------------------
    #[test]
    fn test_memory_cortex_initialization() {
        let cortex = MemoryCortex::default();
        // By default we populate 144 prime references
        assert_eq!(cortex.references.len(), 144);
        // All references are None initially
        assert!(cortex.references.iter().all(|r| r.data.is_none()));
    }

    #[test]
    fn test_memory_cortex_link_manifold() {
        let mut cortex = MemoryCortex::default();
        let manifold = Manifold::new();
        let result = cortex.link_manifold(&manifold);
        assert!(result.is_ok(), "Linking should succeed in the stub");
        // Real test would examine changes to `cortex.references` if implemented
    }

    // 9. Cognitive Stack Tests
    // -------------------------
    #[test]
    fn test_cognitive_stack_default() {
        // We'll rely on NullFoundationModel for default
        let mut stack = CognitiveStack::<NullFoundationModel>::default();
        // By default, it contains 1 model (the default NullFoundationModel)
        assert_eq!(stack.models.len(), 1);

        // MemoryCortex should be defaulted to 144 references
        assert_eq!(stack.cortex.references.len(), 144);
    }

    #[test]
    fn test_cognitive_stack_process_single_model() {
        let mut stack = CognitiveStack::new_default(vec![NullFoundationModel::default()]);
        let manifold = Manifold::new();
        let result = stack.process(manifold);
        assert!(result.is_ok(), "Processing with a single NullFoundationModel should succeed");
    }

    #[test]
    fn test_cognitive_stack_process_kernel() {
        // Use the kernel as the model
        let mut stack = CognitiveStack::new_default(vec![UorKernel::default()]);
        let manifold = Manifold::new();
        let result = stack.process(manifold);
        assert!(result.is_ok(), "Processing with UorKernel should succeed");
    }

    #[test]
    fn test_cognitive_stack_process_multiple_models() {
        // Stack that has both NullFoundationModel and the kernel
        let mut stack = CognitiveStack::new_default(vec![
            NullFoundationModel::default(),
            UorKernel::default(),
        ]);

        // Prepare a small manifold
        let mut manifold = Manifold::new();
        manifold.add_node(ManifoldNode { id: "N1".into(), data: "D1".into() });
        manifold.add_node(ManifoldNode { id: "N2".into(), data: "D2".into() });
        manifold.add_edge("N1", "N2").unwrap();

        let result = stack.process(manifold);
        assert!(result.is_ok(), "Processing with multiple models should succeed");
    }
}
