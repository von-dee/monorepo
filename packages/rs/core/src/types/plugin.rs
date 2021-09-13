use super::{Client, InvokableModules, Uri};
use std::collections::HashMap;

/// Invocable plugin "method"
/// @param input: Input arguments for the method, structured as
/// a map, removing the chance of incorrectly ordering arguments.
/// @param client: The client instance requesting this invocation.
/// This client will be used for any sub-queries that occur.
pub struct PluginMethod<T> {
    pub input: HashMap<String, T>,
    pub client: Client,
}

/// A plugin "module" is a named map of [[PluginMethod | invocable methods]].
/// The names of these methods map 1:1 with the schema's query methods.
pub type PluginModule<T> = HashMap<String, PluginMethod<T>>;

pub type PluginModulesType<InvokableModules> = PluginModule<InvokableModules>;

/// The plugin's query "modules"
pub type PluginModules<InvokableModules> = PluginModulesType<InvokableModules>;

/// The plugin instance
pub trait Plugin: Clone {
    /// Get an instance of this plugin's modules
    /// @param client: The client instance requesting the modules.
    /// This client will be used for any sub-queries that occur.
    fn get_modules(_client: Client) -> PluginModules<InvokableModules>
    where
        Self: Sized,
    {
        unimplemented!()
    }
}

/// The plugin package's manifest
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PluginPackageManifest {
    /// The API's schema
    pub schema: String,
    /// All interface schemas implemented by this plugin
    pub implements: Vec<Uri>,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct PluginPackage {
    pub factory: Client,
    pub manifest: PluginPackageManifest,
}

impl Plugin for PluginPackage {}

pub type PluginFactory = PluginPackage;
