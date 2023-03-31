#[derive(Debug)]
pub struct Config{
    p_size: u32,
    pr_m: f32,
    max_iter: u32
}

impl Config{
    pub fn new(p_size: u32, pr_m: f32, max_iter:u32) -> Self{
        Config{
            p_size,
            pr_m,
            max_iter
        }
    }

    pub fn get_p_size(&self) -> u32{
        self.p_size
    }

    pub fn get_pr_m(&self) -> f32{
        self.pr_m
    }
    
    pub fn get_max_iter(&self) -> u32{
        self.max_iter
    }
}

#[cfg(test)]
mod tests {
    use super::Config;

    #[test]
    fn config_creation() {
        let mut config = Config::new(10, 0.3, 10);

        println!("Configuration {:?}",config);
    }

    #[test]
    fn test_getters() {
        let mut config = Config::new(10, 0.3, 10);

        assert_eq!(config.get_p_size(), 10);
        assert_eq!(config.get_pr_m(), 0.3);
        assert_eq!(config.get_max_iter(), 10);
    }
}