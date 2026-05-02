use crate::domain::VettingScore;

pub struct HeuristicParser;

impl HeuristicParser {
    pub fn score_prompt(content: &str, anchors: &[String]) -> VettingScore {
        let words: Vec<&str> = content.split_whitespace().collect();
        let total_words = words.len() as f32;
        
        if total_words == 0.0 {
            return VettingScore {
                noun_density: 0.0,
                verb_density: 0.0,
                anchor_status: false,
                total_efficiency: 0.0,
            };
        }

        // Simple heuristic: 
        // Nouns: starting with caps (proper) or long words ending in common suffixes
        // Verbs: ending in 'ing', 'ed', 'ize', 'ate'
        let mut nouns = 0;
        let mut verbs = 0;
        
        for word in &words {
            let clean = word.to_lowercase();
            if clean.ends_with("tion") || clean.ends_with("ity") || clean.ends_with("ness") {
                nouns += 1;
            } else if clean.ends_with("ing") || clean.ends_with("ed") || clean.ends_with("ize") {
                verbs += 1;
            } else if word.chars().next().unwrap_or(' ').is_uppercase() {
                nouns += 1;
            }
        }

        let noun_density = (nouns as f32 / total_words) * 100.0;
        let verb_density = (verbs as f32 / total_words) * 100.0;
        
        let anchor_status = anchors.iter().any(|a| content.contains(a));
        
        // Total efficiency score (example formula)
        let mut total_efficiency = (noun_density + verb_density) / 2.0;
        if anchor_status {
            total_efficiency += 20.0;
        }
        total_efficiency = total_efficiency.min(100.0);

        VettingScore {
            noun_density,
            verb_density,
            anchor_status,
            total_efficiency,
        }
    }
}
