apiVersion: "v1"
kind:       "ConfigMap"
metadata: name: "proxy-envs"
data: {
	upstream_host: "node-nest-app-service"
	upstream_port: "8080"
	upstream_name: "backend"
}
