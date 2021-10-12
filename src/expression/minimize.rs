use crate::{LicenseReq, Licensee};
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum MinimizeError {
    TooManyRequirements(usize),
    RequirementsUnmet,
}

impl fmt::Display for MinimizeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TooManyRequirements(n) => write!(
                f,
                "the license expression required {} licensees which exceeds the limit of 64",
                n
            ),
            Self::RequirementsUnmet => {
                f.write_str("the expression was not satisfied by the provided list of licensees")
            }
        }
    }
}

impl std::error::Error for MinimizeError {
    fn description(&self) -> &str {
        match self {
            Self::TooManyRequirements(_) => "too many requirements in license expression",
            Self::RequirementsUnmet => {
                "the expression was not satisfied by the provided list of licensees"
            }
        }
    }
}

impl super::Expression {
    pub fn minimized_requirements<'lic>(
        &self,
        accepted: impl IntoIterator<Item = &'lic Licensee>,
    ) -> Result<Vec<LicenseReq>, MinimizeError> {
        let found_set = {
            let mut found_set = smallvec::SmallVec::<[Licensee; 5]>::new();

            for lic in accepted {
                if !found_set.contains(lic)
                    && self.requirements().any(|ereq| lic.satisfies(&ereq.req))
                {
                    found_set.push(lic.clone());
                }
            }

            if found_set.len() > 64 {
                return Err(MinimizeError::TooManyRequirements(found_set.len()));
            }

            // Ensure that the licensees provided actually _can_ be accepted by
            // this expression
            if !self.evaluate(|ereq| found_set.iter().any(|lic| lic.satisfies(ereq))) {
                return Err(MinimizeError::RequirementsUnmet);
            }

            found_set
        };

        let set_size = (1 << found_set.len()) as u64;

        for mask in 1..=set_size {
            let eval_res = self.evaluate(|req| {
                for (ind, lic) in found_set.iter().enumerate() {
                    if mask & (1 << ind) != 0 && lic.satisfies(req) {
                        return true;
                    }
                }

                false
            });

            if eval_res {
                return Ok(found_set
                    .into_iter()
                    .enumerate()
                    .filter_map(|(ind, lic)| {
                        if mask & (1 << ind) != 0 {
                            Some(lic.into_req())
                        } else {
                            None
                        }
                    })
                    .collect());
            }
        }

        // This should be impossible, but would rather not panic
        Ok(found_set.into_iter().map(|lic| lic.into_req()).collect())
    }
}
