use crate::config::CopperConfig;

pub trait CopperRuntimeTrait<Instances> {
    fn new(config: CopperConfig, instances: Instances) -> Self;
}

/// This is the core of copper, getting the generated tuple of tasks instances and copper list.
pub struct CopperRuntime<CT /*, CL*/> {
    tasks_instances: CT,
    // copper_lists: Vec<CL>,
}

impl<CT /*, CL*/> CopperRuntime<CT /*, CL*/> {
    /// Create a new CopperRuntime instance.
    pub fn new(copper_config: CopperConfig, tasks_instances: CT) -> Self {
        CopperRuntime {
            tasks_instances,
            //copper_lists: Vec::new(),
        }
    }

    pub fn start(&self) {
        for task in self.tasks_instances {
            task.start();
        }
    }
}
