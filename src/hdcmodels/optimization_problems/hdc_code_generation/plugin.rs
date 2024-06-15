pub trait Plugin {
    fn initialize(&self);
    fn execute(&self, input: &str) -> String;
    fn finalize(&self);
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

    pub fn execute_all(&self, input: &str) -> Vec<String> {
        self.plugins.iter().map(|plugin| plugin.execute(input)).collect()
    }
}
