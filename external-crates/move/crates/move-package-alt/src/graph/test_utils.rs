use std::{collections::BTreeMap, convert::identity};

use move_core_types::account_address::AccountAddress;
use petgraph::graph::{DiGraph, NodeIndex};

use crate::{
    flavor::Vanilla,
    graph::PackageGraph,
    package::{EnvironmentID, EnvironmentName, PackageName},
    schema::{OriginalID, PublishedID},
};

pub const DEFAULT_ENV_NAME: &str = "_test_env";
pub const DEFAULT_ENV_ID: &str = "_test_env_id";

pub struct Scenario;

pub struct TestPackageGraph {
    // invariant: `nodes` contains exactly one entry for each node of `inner`
    inner: DiGraph<PackageSpec, DepSpec>,
    nodes: BTreeMap<PackageName, NodeIndex>,
}

/// Information used to build a node in the package graph
pub struct PackageSpec {
    /// The `package.name` field
    name: PackageName,

    /// The publications for each environment
    pubs: BTreeMap<EnvironmentName, PubSpec>,
}

/// Information used to build an edge in the package graph
#[derive(Default)]
pub struct DepSpec {
    /// The name that the containing package gives to the dependency (may be different from the
    /// dependency's name, but the default is to use the dependency's name)
    name: Option<PackageName>,

    /// whether to include `override = true`
    is_override: bool,

    /// the `rename-from` field for the dep
    rename_from: Option<PackageName>,

    /// the `[dep-replacements]` environment to include the dep in (or `None` for the default section)
    env: Option<EnvironmentName>,

    /// the `use-environment` field for the dep
    use_env: Option<EnvironmentName>,
}

/// Information about a publication
#[derive(Default)]
pub struct PubSpec {
    chain_id: Option<EnvironmentID>,
    original_id: Option<OriginalID>,
    published_at: Option<PublishedID>,
}

impl TestPackageGraph {
    /// Create a package graph containing nodes named `node_names`
    pub fn new(node_names: impl IntoIterator<Item = impl AsRef<str>>) -> Self {
        let mut inner = DiGraph::new();
        let mut nodes = BTreeMap::new();
        for node in node_names {
            let index = inner.add_node(PackageSpec::new(node.as_ref()));
            let old = nodes.insert(
                PackageName::new(node.as_ref()).expect("valid package name"),
                index,
            );
            assert!(old.is_none());
        }
        Self { inner, nodes }
    }

    /// Add a dependency to the graph from `a` to `b` for each pair `("a", "b")` in `edges`. The
    /// dependencies will be local dependencies in the `[dependencies]` sections.
    pub fn add_deps(
        mut self,
        edges: impl IntoIterator<Item = (impl AsRef<str>, impl AsRef<str>)>,
    ) -> Self {
        edges.into_iter().fold(self, |graph, (source, target)| {
            graph.add_dep(source, target, identity)
        })
    }

    pub fn add_package(
        mut self,
        node: impl AsRef<str>,
        build: impl FnOnce(PackageSpec) -> PackageSpec,
    ) -> Self {
        let builder = PackageSpec::new(&node);

        let index = self.inner.add_node(build(builder));
        let old = self.nodes.insert(
            PackageName::new(node.as_ref()).expect("valid package name"),
            index,
        );
        assert!(old.is_none());

        self
    }

    pub fn add_dep(
        mut self,
        source: impl AsRef<str>,
        target: impl AsRef<str>,
        build: impl FnOnce(DepSpec) -> DepSpec,
    ) -> Self {
        let source_idx = self.nodes[source.as_ref()];
        let target_idx = self.nodes[target.as_ref()];
        self.inner
            .add_edge(source_idx, target_idx, build(DepSpec::default()).into());
        self
    }

    pub fn build(self) -> Scenario {
        todo!()
    }
}

impl PackageSpec {
    /// Create a new empty package spec
    fn new(name: impl AsRef<str>) -> Self {
        Self {
            name: PackageName::new(name.as_ref()).expect("valid package name"),
            pubs: BTreeMap::new(),
        }
    }

    /// Set the published original_id for the dependency in the default environment
    pub fn original_id(mut self, addr: u16) -> Self {
        let oid = OriginalID(AccountAddress::from_suffix(addr));
        self.pubs
            .entry(DEFAULT_ENV_NAME.to_string())
            .or_default()
            .original_id = Some(oid);
        self
    }

    /// Set the published published_at field for the dependency in the default environment
    pub fn published_at(mut self, addr: u16) -> Self {
        let pid = PublishedID(AccountAddress::from_suffix(addr));
        self.pubs
            .entry(DEFAULT_ENV_NAME.to_string())
            .or_default()
            .published_at = Some(pid);
        self
    }
}

impl DepSpec {
    fn new() -> Self {
        Self::default()
    }

    /// Add `override = true` to the dependency
    pub fn set_override(mut self) -> Self {
        self.is_override = true;
        self
    }

    /// Set the name used for the dependency in the containing package
    pub fn name(mut self, name: impl AsRef<str>) -> Self {
        self.name = Some(PackageName::new(name.as_ref()).expect("valid package name"));
        self
    }

    /// Set the `rename-from` field of the dependency
    pub fn rename_from(mut self, original: impl AsRef<str>) -> Self {
        self.rename_from = Some(PackageName::new(original.as_ref()).expect("valid package name"));
        self
    }

    /// Only include the dependency in `env` (in the `dep-replacements` section)
    pub fn in_env(mut self, env: impl AsRef<str>) -> Self {
        self.env = Some(env.as_ref().to_string());
        self
    }

    /// Set the `use-environment` field of the dependency
    pub fn use_env(mut self, env: impl AsRef<str>) -> Self {
        self.use_env = Some(env.as_ref().to_string());
        self
    }
}

impl Scenario {
    pub fn graph_for(&self, root: impl AsRef<str>) -> PackageGraph<Vanilla> {
        todo!()
    }
}
