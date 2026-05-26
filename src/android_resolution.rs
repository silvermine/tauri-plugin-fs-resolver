use fs_resolver::AndroidPath;
use fs_resolver::AndroidPathCollection;
use fs_resolver::PathResolver;
use fs_resolver::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tauri::Runtime;
use tauri::plugin::PluginApi;

#[derive(Serialize, Deserialize)]
struct ResolvePathRequest {
   path: AndroidPath,
}

#[derive(Serialize, Deserialize)]
struct ResolvePathResponse {
   path: String,
}

#[derive(Serialize, Deserialize)]
struct ResolvePathCollectionRequest {
   collection: AndroidPathCollection,
}

#[derive(Serialize, Deserialize)]
struct ResolvePathCollectionResponse {
   paths: Vec<String>,
}

pub(crate) fn configure_android_path_resolution<R: Runtime>(
   api: &PluginApi<R, ()>,
   resolver: &mut PathResolver,
) -> Result<()> {
   let handle = api
      .register_android_plugin("org.silvermine.plugin.fs_resolver", "FsResolverPlugin")
      .map_err(|e| fs_resolver::Error::Initialization(e.to_string()))?;

   let path_resolution_handle = handle.clone();
   let resolve_path = move |path: &AndroidPath| -> Result<PathBuf> {
      let request = ResolvePathRequest { path: path.clone() };
      let request_payload = serde_json::to_value(request)
         .map_err(|e| fs_resolver::Error::JsonSerialization(e.to_string()))?;
      let resp: ResolvePathResponse = path_resolution_handle
         .run_mobile_plugin("resolveDirectory", request_payload)
         .map_err(|e| fs_resolver::Error::PluginInvocation(e.to_string()))?;
      Ok(PathBuf::from(resp.path))
   };

   let path_collection_resolution_handle = handle.clone();
   let resolve_collection = move |collection: &AndroidPathCollection| -> Result<Vec<PathBuf>> {
      let request = ResolvePathCollectionRequest {
         collection: collection.clone(),
      };
      let request_payload = serde_json::to_value(request)
         .map_err(|e| fs_resolver::Error::JsonSerialization(e.to_string()))?;
      let resp: ResolvePathCollectionResponse = path_collection_resolution_handle
         .run_mobile_plugin("resolveDirectoryCollection", request_payload)
         .map_err(|e| fs_resolver::Error::PluginInvocation(e.to_string()))?;
      Ok(resp.paths.into_iter().map(PathBuf::from).collect())
   };

   resolver.configure_android_path_resolution(Box::new(resolve_path), Box::new(resolve_collection));

   Ok(())
}
