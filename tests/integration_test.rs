//! Integration tests for SigmaOS-data-quality-improvement.

use sigmaos_data_quality_improvement_lib::Component;

#[test]
fn component_initialises_and_runs() {
    let mut c = Component::new();
    c.init().expect("init should succeed");
    c.run().expect("run should succeed after init");
}
