 apiVersion: "apiextensions.crossplane.io/v1"
kind:       "CompositeResourceDefinition"
metadata: name: "compositestorages.multicloud.platformref.yurikrupnik.com"
spec: {
	claimNames: {
		kind:   "Storage"
		plural: "storages"
	}
	group: "multicloud.platformref.yurikrupnik.com"
	names: {
		kind:   "CompositeStorage"
		plural: "compositestorages"
	}
	versions: [{
		name:          "v1alpha1"
		served:        true
		referenceable: true
		schema: openAPIV3Schema: {
			type: "object"
			properties: spec: {
				type: "object"
				properties: location: {
					type:        "string"
					description: "Location where the bucket should be created."
				}
				required: ["location"]
			}
		}
	}]
}
