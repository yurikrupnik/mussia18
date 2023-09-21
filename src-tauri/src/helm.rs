
extern crate kube;
use kube::api::Api;
use kube::Client;
use kube::helm::HelmRelease;
use kube::helm::ReleaseSpec;
use kube::ResourceExt;
// use <kube::metadata::>Metadata::</kube::metadata::>
use serde_json::json;
use serde_yaml::Value;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;


async fn helm_install() -> Result<(), kube::Error> {
    // Initialize the Kubernetes client
    let client = Client::try_default().await?;

    // Specify the Helm release name and namespace
    let release_name = "my-release";
    let namespace = "default";

    // Load the Helm chart values from a values.yaml file
    let values_file_path = "path/to/values.yaml";
    let mut values_file = File::open(values_file_path)?;
    let mut values_str = String::new();
    values_file.read_to_string(&mut values_str)?;

    // Parse the values.yaml content into a serde_yaml::Value
    let values: Value = serde_yaml::from_str(&values_str)?;

    // Create a HelmRelease resource
    let helm_release = HelmRelease {
        metadata: Default::default(),
        spec: ReleaseSpec {
            chart: "stable/nginx-ingress".to_string(),
            releaseName: release_name.to_string(),
            namespace: namespace.to_string(),
            values: Some(values),
            ..Default::default()
        },
    };

    // Create a Kubernetes API client for HelmRelease resources
    let api: Api<HelmRelease> = Api::namespaced(client.clone(), namespace);

    // Apply the HelmRelease to the cluster
    let applied_release = api.create(&helm_release).await?;
    println!("HelmRelease applied: {:?}", applied_release);

    Ok(())
}


