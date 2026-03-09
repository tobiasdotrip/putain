use putain::personality::Personality;

#[test]
fn test_first_offense() {
    let mut p = Personality::new();
    let msg = p.react("sudo");
    assert!(msg.contains("Putain") || msg.contains("putain") || msg.contains("Merde"));
}

#[test]
fn test_escalation() {
    let mut p = Personality::new();
    let _ = p.react("sudo");
    let msg = p.react("sudo");
    assert!(msg.contains("encore") || msg.contains("ENCORE"));
}

#[test]
fn test_max_escalation() {
    let mut p = Personality::new();
    let _ = p.react("sudo");
    let _ = p.react("sudo");
    let _ = p.react("sudo");
    let msg = p.react("sudo");
    // Level 4+: exasperated
    assert!(msg.contains("soupir") || msg.contains("..."));
}

#[test]
fn test_different_rules_independent() {
    let mut p = Personality::new();
    let _ = p.react("sudo");
    let _ = p.react("sudo");
    let msg = p.react("typo");
    // Different rule should start at level 1
    assert!(msg.contains("Putain") || msg.contains("putain") || msg.contains("Merde"));
}
