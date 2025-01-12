use std::fmt::Display;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct Health {
    level: u32,
    max: u32,
}

#[derive(Debug, PartialEq, thiserror::Error)]
pub enum HealthError {
    #[error("The level of health cannot be higher than max level.")]
    LevelTooHigh,
}

#[derive(Debug, PartialEq)]
pub enum HealthLog {
    HpMaxedOut,
}

impl Display for HealthLog {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let log = match self {
            HealthLog::HpMaxedOut => "HP maxed out",
        };

        write!(f, "{}", log)
    }
}

impl Health {
    pub fn new(max: u32, level: Option<u32>) -> Result<Health, HealthError> {
        if let Some(level_value) = level {
            Health::validate(level_value, max)?;
        }

        Ok(Health {
            max,
            level: level.unwrap_or(max),
        })
    }

    fn validate(level: u32, max: u32) -> Result<(), HealthError> {
        if level > max {
            Err(HealthError::LevelTooHigh)
        } else {
            Ok(())
        }
    }

    pub fn increase(&mut self, amount: u32) -> Option<HealthLog> {
        let new_value = self.level.saturating_add(amount);

        if new_value >= self.max {
            self.level = self.max;
            Some(HealthLog::HpMaxedOut)
        } else {
            self.level = new_value;
            None
        }
    }

    pub fn decrease(&mut self, amount: u32) {
        self.level = self.level.saturating_sub(amount);
    }

    pub fn get_level(&self) -> &u32 {
        &self.level
    }

    pub fn get_max(&self) -> &u32 {
        &self.max
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_init() {
        assert_eq!(
            Health::new(1u32, Some(2u32)).unwrap_err(),
            HealthError::LevelTooHigh
        );

        // init with no level
        assert_eq!(Health::new(5u32, None).unwrap().get_level(), &5u32);

        assert_eq!(Health::new(5u32, Some(4u32)).unwrap().get_level(), &4u32);
    }

    #[test]
    fn test_increase() {
        let mut health = Health::new(10u32, Some(0u32)).unwrap();

        health.increase(2u32);

        assert_eq!(health.get_level(), &2u32);

        // Add more than max value capacity
        let log = health.increase(10u32);

        // return log and max out the health level
        assert_eq!(log, Some(HealthLog::HpMaxedOut));
        assert_eq!(health.get_level(), health.get_max());
    }

    #[test]
    fn test_decrease() {
        let mut health = Health::new(10u32, None).unwrap();

        health.decrease(2u32);

        assert_eq!(health.get_level(), &8u32);

        // decrease more than actual health level
        health.decrease(10u32);

        assert_eq!(health.get_level(), &0u32);
    }
}
