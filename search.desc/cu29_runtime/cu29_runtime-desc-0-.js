searchState.loadedDescShard("cu29_runtime", 0, "This module defines the configuration of the copper …\nCopperList is the main data structure used by Copper to …\nCuRuntime is the heart of what copper is running on the …\nThis module contains all the main definition of the traits …\nSome basic internal monitoring tooling Copper uses to …\n<code>cu29::simulation</code> Module\nThis represents a connection between 2 tasks (nodes) in …\nThis is the configuration of a component (like a task …\nCuConfig is the programmatic representation of the …\nA node in the configuration graph. A node represents a …\nNodeId is the unique identifier of a node in the …\nWrapper around the ron::Value to allow for custom …\nAdd a new node to the configuration graph.\nTells Copper to batch messages before sending the buffer …\nAdds an edge between two nodes/tasks in the configuration …\nAdds an edge between two nodes/tasks in the configuration …\nThis is a custom serialization to make this implementation …\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nConvenience method to get all nodes in the configuration …\nGet the list of edges that are connected to the given node …\nGet the node with the given id.\nthis is more like infer from the connections of this node.\nGet the node with the given id mutably.\nthis is more like infer from the connections of this node.\nGet the list of edges that are connected to the given node …\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nMessage type exchanged between src and dst.\nRead a copper configuration from a file.\nRead a copper configuration from a file.\nRender the configuration graph in the dot format.\nThis is a custom serialization to make this implementation …\nTells Copper if it needs to log the messages.\nNot implemented yet. This mask will be used to for example …\nThose are the possible states along the lifetime of a …\nThis structure maintains the entire memory needed by …\nReturns an ascending iterator over the queue’s contents.\nReturns a mutable ascending iterator over the queue’s …\nClears the queue.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nReturns <code>true</code> if the queue contains no elements.\nReturns <code>true</code> if the queue is full.\nReturns an iterator over the queue’s contents.\nReturns a mutable iterator over the queue’s contents.\nReturns the current number of elements in the queue.\nPeeks at the last element in the queue.\nJust a simple struct to hold the various bits needed to …\nThis structure represents a loop in the execution plan. It …\nThis structure represents a step in the execution plan.\nThis structure represents a step in the execution plan.\nThis is the main structure that will be injected as a …\nCopper tasks can be of 3 types:\nThe base clock the runtime will be using to record time.\nThis is the main heuristics to compute an execution plan …\nCopper lists hold in order all the input/output messages …\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nthe indices in the copper list of the input messages and …\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nNode: node instance\nNodeId: node id of the task to execute\nthe index in the copper list of the output message and its …\nCuTaskType: type of the task\nThe tuple of all the tasks in order of execution.\nCuMsg is the envelope holding the msg payload and the …\nCuMsgMetadata is a structure that contains metadata common …\nA Sink Task is a task that only consumes messages. For …\nA Src Task is a task that only produces messages. For …\nThis is the most generic Task of copper. It is a “…\nThe internal state of a task needs to be serializable so …\nThis method is called by the framework when it wants to …\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nThis metadata is the data that is common to all messages.\nHere you need to initialize everything your task will need …\nHere you need to initialize everything your task will need …\nHere you need to initialize everything your task will need …\nThis is a method called by the runtime after “process”…\nThis is a method called by the runtime after “process”…\nThis is a method called by the runtime after “process”…\nThis is a method called by the runtime before “process”…\nThis is a method called by the runtime before “process”…\nThis is a method called by the runtime before “process”…\nProcess is the most critical execution of the task. The …\nProcess is the most critical execution of the task. The …\nProcess is the most critical execution of the task. The …\nThe time range used for the processing of this message\nStart is called between the creation of the task and the …\nStart is called between the creation of the task and the …\nStart is called between the creation of the task and the …\nA small string for real time feedback purposes. This is …\nCalled to stop the task. It signals that the *process …\nCalled to stop the task. It signals that the *process …\nCalled to stop the task. It signals that the *process …\nThis method is called by the framework when it wants to …\nThe time of validity of the message. It can be undefined …\nA simple allocator that counts the number of bytes …\nA Specialized statistics object for CuDuration. It will …\nTrait to implement a monitoring task.\nThe state of a task.\nMonitor decision to be taken when a task errored out.\nAccumulative stat object that can give your some real time …\nA do nothing monitor if no monitor is provided. This is …\nA simple struct that counts the number of bytes allocated …\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCallback that will be trigger at the end of every …\nCallbacked when a Task errored out. The runtime requires …\nAdds a value to the statistics.\nCallbacked when copper is stopping.\nCopper friendly wrapper for a fixed size array.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nThis is a placeholder task for a sink task for the …\nThis is a placeholder task for a source task for the …\nThis is the state that will be passed to the simulation …\nEmulated the behavior of an erroring task (same as return …\nThe actual implementation needs to be executed.\nThe callback took care of the logic on the simulation side …\nCallbacked when a task is created. It gives you the …\nCallbacked when a task is getting called on post-process.\nCallbacked when a task is getting called on pre-process.\nCallbacked when a task is getting called on process. I and …\nThis is the answer the simulator can give to control the …\nCallbacked when a task is started.\nCallbacked when a task is stopped.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.")