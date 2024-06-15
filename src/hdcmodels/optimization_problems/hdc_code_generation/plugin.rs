pub trait Plugin {
    fn initialize(&mut self);
    fn execute(&self, input: &str) -> String;
    fn finalize(&mut self);
}

pub struct PluginManager {
    plugins: Vec<Box<dyn Plugin>>,
}

impl PluginManager {
    pub fn new() -> Self {
        PluginManager { plugins: Vec::new() }
    }

    pub fn register_plugin(&mut self, plugin: Box<dyn Plugin>) {
        self.plugins.push(plugin);
    }

    pub fn initialize_all(&mut self) {
        for plugin in &mut self.plugins {
            plugin.initialize();
        }
    }

    pub fn finalize_all(&mut self) {
        for plugin in &mut self.plugins {
            plugin.finalize();
        }
    }

    pub fn execute_all(&self, input: &str) -> Vec<String> {
        self.plugins.iter().map(|plugin| plugin.execute(input)).collect()
    }
}
