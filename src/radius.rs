#[allow(dead_code)]
pub struct Radius {
    rem_value: String
}

#[allow(dead_code)]
impl Radius {
    pub fn none() -> Self {
        Self { rem_value: "none".to_string() }
    }

    pub fn extra_small() -> Self {
        Self { rem_value: "0.125rem".to_string() }
    }

    pub fn small() -> Self {
        Self { rem_value: "0.25rem".to_string() }
    }

    pub fn medium() -> Self {
        Self { rem_value: "0.375rem".to_string() }
    }

    pub fn large() -> Self {
        Self { rem_value: "0.5rem".to_string() }
    }

    pub fn extra_large() -> Self {
        Self { rem_value: "0.75rem".to_string() }
    }

    pub fn circle() -> Self {
        Self { rem_value: "1rem".to_string() }
    }
}