mod lab_boosted_classifier;
mod surf_mlp_classifier;

pub use self::lab_boosted_classifier::LabBoostedClassifier;
pub use self::surf_mlp_classifier::SurfMlpClassifier;

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub enum ClassifierKind {
    LabBoosted,
    SurfMlp,
}

impl ClassifierKind {
    pub fn from(id: i32) -> Option<Self> {
        match id {
            0 => Some(ClassifierKind::LabBoosted),
            1 => Some(ClassifierKind::SurfMlp),
            _ => None,
        }
    }
}

pub struct Score {
    positive: bool,
    score: f32,
}

impl Score {
    pub fn positive(&self) -> bool {
        self.positive
    }

    pub fn score(&self) -> f32 {
        self.score
    }
}

pub trait Classifier {
    fn classify(&mut self, output: &mut Vec<f32>) -> Score;
}