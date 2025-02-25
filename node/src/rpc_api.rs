use jsonrpsee::{core::RpcResult, proc_macros::rpc};
use kylix_runtime::{AggregatedTotals, LendingPoolInfo};

/// Defines the custom RPC interface for the lending pools.
/// This trait specifies the structure and methods for the RPC interface that clients
/// can use to interact with lending pools on the blockchain.
///
/// The trait is marked with the `#[rpc(client, server)]` attribute, which automatically
/// generates both the client and server implementations for this RPC interface. This
/// attribute simplifies the creation of RPC methods that can be called by clients and
/// implemented by servers.
#[rpc(client, server)]
pub trait LendingPoolApi {
    /// Fetches the list of lending pools and their aggregated totals.
    ///
    /// This method is invoked by clients to retrieve information about all available lending pools,
    /// including details such as pool identifiers, associated assets, and aggregated totals.
    ///
    /// # Returns
    ///
    /// * `RpcResult<(Vec<LendingPoolInfo>, AggregatedTotals)>` - A result containing a tuple:
    ///   - A vector of `LendingPoolInfo` structs, each representing the details of a lending pool.
    ///   - An `AggregatedTotals` struct containing aggregated data for all lending pools.
    ///
    /// # Errors
    ///
    /// This method returns an `RpcResult` that may contain an error if the data cannot be retrieved
    /// due to issues such as blockchain state access failures or other runtime errors.
    #[method(name = "getLendingPools")]
    fn get_lending_pools(&self) -> RpcResult<(Vec<LendingPoolInfo>, AggregatedTotals)>;
}
