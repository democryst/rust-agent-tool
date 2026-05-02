use crate::domain::VettingScore;

pub struct GrillService;

impl GrillService {
    pub fn generate_feedback(score: &VettingScore) -> String {
        let mut feedback = String::from("🚨 PROMPT REJECTED: Low Efficiency Score\n\n");
        feedback.push_str(&format!("Total Efficiency: {:.1}/100 (Required: 40.0)\n", score.total_efficiency));
        feedback.push_str("----------------------------------\n");
        feedback.push_str("TECHNICAL DEBT CHECKLIST:\n");
        
        if !score.anchor_status {
            feedback.push_str("- [ ] MISSING ANCHORS: No project-specific keywords found.\n");
        } else {
            feedback.push_str("- [x] ANCHORS DETECTED\n");
        }

        if score.noun_density < 15.0 {
            feedback.push_str(&format!("- [ ] LOW NOUN DENSITY: {:.1}% (Expected > 15%)\n", score.noun_density));
        } else {
            feedback.push_str("- [x] NOUN DENSITY SUFFICIENT\n");
        }

        if score.verb_density < 10.0 {
            feedback.push_str(&format!("- [ ] LOW VERB DENSITY: {:.1}% (Expected > 10%)\n", score.verb_density));
        } else {
            feedback.push_str("- [x] VERB DENSITY SUFFICIENT\n");
        }

        feedback.push_str("\n💡 TIP: Be more precise, use technical nouns, and anchor your request in known context.");
        feedback
    }

    pub fn security_failure(msg: &str) -> String {
        format!("🔒 SECURITY VIOLATION: {}\nAccess denied. Please provide a valid SHA-256 vetted signature.", msg)
    }
}
