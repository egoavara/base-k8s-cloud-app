locals {
  grafana_labels = merge({
    grafana_dashboard = "1"
  }, local.labels)
}

resource "kubernetes_config_map" "istio-services-grafana-dashboards" {
  metadata {
    name      = "istio-services-grafana-dashboards"
    namespace = kubernetes_namespace.telemetry-system.metadata[0].name
    labels    = merge({}, local.grafana_labels)
  }

  data = {
    "istio-extension-dashboard.json" = file("${path.module}/configs/istio-services-grafana-dashboards/istio-extension-dashboard.json")
    "istio-mesh-dashboard.json"      = file("${path.module}/configs/istio-services-grafana-dashboards/istio-mesh-dashboard.json")
    "istio-service-dashboard.json"   = file("${path.module}/configs/istio-services-grafana-dashboards/istio-service-dashboard.json")
    "istio-workload-dashboard.json"  = file("${path.module}/configs/istio-services-grafana-dashboards/istio-workload-dashboard.json")
  }
}

resource "kubernetes_config_map" "istio-grafana-dashboards" {
  metadata {
    name      = "istio-grafana-dashboards"
    namespace = kubernetes_namespace.telemetry-system.metadata[0].name
    labels    = merge({}, local.grafana_labels)
  }

  data = {
    "istio-performance-dashboard.json" = file("${path.module}/configs/istio-grafana-dashboards/istio-performance-dashboard.json")
    "pilot-dashboard.json"             = file("${path.module}/configs/istio-grafana-dashboards/pilot-dashboard.json")
  }
}