
use ::{
    Direction,
    Incoming,
};

use visit::{
    GraphBase,
    GraphRef,
    IntoNodeIdentifiers,
    IntoNeighbors,
    IntoNeighborsDirected,
    IntoEdgeReferences,
    IntoExternals,
    NodeIndexable,
    Visitable,
    EdgeRef,
    Prop,
    Data,
};

/// Wrapper type for walking the graph as if all edges are reversed.
#[derive(Copy, Clone, Debug)]
pub struct Reversed<G>(pub G);

impl<G: GraphBase> GraphBase for Reversed<G> {
    type NodeId = G::NodeId;
    type EdgeId = G::EdgeId;
}

impl<G: GraphRef> GraphRef for Reversed<G> { }

impl<G: Data> Data for Reversed<G> {
    type NodeWeight = G::NodeWeight;
    type EdgeWeight = G::EdgeWeight;
}

impl<G> IntoNeighbors for Reversed<G>
    where G: IntoNeighborsDirected
{
    type Neighbors = G::NeighborsDirected;
    fn neighbors(self, n: G::NodeId) -> G::NeighborsDirected
    {
        self.0.neighbors_directed(n, Incoming)
    }
}

impl<G> IntoNeighborsDirected for Reversed<G>
    where G: IntoNeighborsDirected
{
    type NeighborsDirected = G::NeighborsDirected;
    fn neighbors_directed(self, n: G::NodeId, d: Direction)
        -> G::NeighborsDirected
    {
        self.0.neighbors_directed(n, d.opposite())
    }
}

impl<G> IntoExternals for Reversed<G>
    where G: IntoExternals,
{
    type Externals = G::Externals;
    fn externals(self, d: Direction) -> G::Externals {
        self.0.externals(d.opposite())
    }
}

impl<G: Visitable> Visitable for Reversed<G>
{
    type Map = G::Map;
    fn visit_map(&self) -> G::Map {
        self.0.visit_map()
    }
    fn reset_map(&self, map: &mut Self::Map) {
        self.0.reset_map(map);
    }
}


/// An edge reference for `Reversed`.
#[derive(Copy, Clone, Debug)]
pub struct ReversedEdgeRef<R>(R);

/// An edge reference
impl<R> EdgeRef for ReversedEdgeRef<R>
    where R: EdgeRef,
{
    type NodeId = R::NodeId;
    type EdgeId = R::EdgeId;
    type Weight = R::Weight;
    fn source(&self) -> Self::NodeId {
        self.0.target()
    }
    fn target(&self) -> Self::NodeId {
        self.0.source()
    }
    fn weight(&self) -> &Self::Weight {
        self.0.weight()
    }
    fn id(&self) -> Self::EdgeId {
        self.0.id()
    }
}

impl<G> Prop for Reversed<G>
    where G: Prop
{
    type EdgeType = G::EdgeType;
}

impl<G> IntoEdgeReferences for Reversed<G>
    where G: IntoEdgeReferences
{
    type EdgeRef = ReversedEdgeRef<G::EdgeRef>;
    type EdgeReferences = ReversedEdgeReferences<G::EdgeReferences>;
    fn edge_references(self) -> Self::EdgeReferences {
        ReversedEdgeReferences {
            iter: self.0.edge_references(),
        }
    }
}

/// An iterator of edge references for `Reversed`.
pub struct ReversedEdgeReferences<I> {
    iter: I,
}

impl<I> Iterator for ReversedEdgeReferences<I>
    where I: Iterator,
          I::Item: EdgeRef,
{
    type Item = ReversedEdgeRef<I::Item>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(ReversedEdgeRef)
    }
}

macro_rules! access0 {
    ($e:expr) => ($e.0)
}

NodeIndexable!{delegate_impl [[G], G, Reversed<G>, access0]}
IntoNodeIdentifiers!{delegate_impl [[G], G, Reversed<G>, access0]}

