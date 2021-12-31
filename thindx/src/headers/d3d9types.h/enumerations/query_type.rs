#[allow(unused_imports)] use crate::*;

use winapi::shared::d3d9types::*;



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dquerytype)\]
/// D3DQUERYTYPE
///
/// Identifies the query type. For information about queries, see [Queries (Direct3D 9)]
///
/// [Queries (Direct3D 9)]:         https://docs.microsoft.com/en-us/windows/win32/direct3d9/queries
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)] pub struct QueryType(D3DQUERYTYPE);

enumish! {
    QueryType => D3DQUERYTYPE;
    VCache, ResourceManager, VertexStats, Event, Occlusion, TimeStamp, TimeStampDisjoint, TimeStampFreq,
    PipelineTimings, InterfaceTimings, VertexTimings, PixelTimings, BandwidthTimings, CacheUtilization,
    MemoryPressure,
}

#[allow(non_upper_case_globals)] impl QueryType { // These are enum-like
    /// Query for driver hints about data layout for vertex caching.
    pub const VCache            : QueryType = QueryType(D3DQUERYTYPE_VCACHE); // 4

    /// Query the resource manager. For this query, the device behavior flags must include D3DCREATE_DISABLE_DRIVER_MANAGEMENT.
    pub const ResourceManager   : QueryType = QueryType(D3DQUERYTYPE_RESOURCEMANAGER);

    /// Query vertex statistics.
    pub const VertexStats       : QueryType = QueryType(D3DQUERYTYPE_VERTEXSTATS);

    /// Query for any and all asynchronous events that have been issued from API calls.
    pub const Event             : QueryType = QueryType(D3DQUERYTYPE_EVENT);

    /// An occlusion query returns the number of pixels that pass z-testing.
    /// These pixels are for primitives drawn between the issue of D3DISSUE_BEGIN and D3DISSUE_END.
    /// This enables an application to check the occlusion result against 0.
    /// Zero is fully occluded, which means the pixels are not visible from the current camera position.
    pub const Occlusion         : QueryType = QueryType(D3DQUERYTYPE_OCCLUSION);

    /// Returns a 64-bit timestamp.
    pub const TimeStamp         : QueryType = QueryType(D3DQUERYTYPE_TIMESTAMP);

    /// Use this query to notify an application if the counter frequency has changed from the D3DQUERYTYPE_TIMESTAMP.
    pub const TimeStampDisjoint : QueryType = QueryType(D3DQUERYTYPE_TIMESTAMPDISJOINT);

    /// This query result is TRUE if the values from D3DQUERYTYPE_TIMESTAMP queries cannot be guaranteed to be continuous throughout the duration of the D3DQUERYTYPE_TIMESTAMPDISJOINT query.
    /// Otherwise, the query result is FALSE.
    pub const TimeStampFreq     : QueryType = QueryType(D3DQUERYTYPE_TIMESTAMPFREQ);

    /// Percent of time processing pipeline data.
    pub const PipelineTimings   : QueryType = QueryType(D3DQUERYTYPE_PIPELINETIMINGS);

    /// Percent of time processing data in the driver.
    pub const InterfaceTimings  : QueryType = QueryType(D3DQUERYTYPE_INTERFACETIMINGS);

    /// Percent of time processing vertex shader data.
    pub const VertexTimings     : QueryType = QueryType(D3DQUERYTYPE_VERTEXTIMINGS);

    /// Percent of time processing pixel shader data.
    pub const PixelTimings      : QueryType = QueryType(D3DQUERYTYPE_PIXELTIMINGS);

    /// Throughput measurement comparisons for help in understanding the performance of an application.
    pub const BandwidthTimings  : QueryType = QueryType(D3DQUERYTYPE_BANDWIDTHTIMINGS);

    /// Measure the cache hit-rate performance for textures and indexed vertices.
    pub const CacheUtilization  : QueryType = QueryType(D3DQUERYTYPE_CACHEUTILIZATION);

    /// Efficiency of memory allocation contained in a D3DMEMORYPRESSURE structure.
    ///
    /// **Differences between Direct3D 9 and Direct3D 9Ex:**
    /// D3DQUERYTYPE_MEMORYPRESSURE is only available in Direct3D9Ex running on Windows 7 (or more current operating system).
    #[cfg(feature = "9ex")]
    pub const MemoryPressure    : QueryType = QueryType(D3DQUERYTYPE_MEMORYPRESSURE);
    #[cfg(not(feature = "9ex"))]
    pub(crate) const MemoryPressure : QueryType = QueryType(D3DQUERYTYPE_MEMORYPRESSURE);
}

#[cfg(feature = "impl-poor-defaults")]
impl Default for QueryType {
    fn default() -> Self { QueryType(0) }
}
