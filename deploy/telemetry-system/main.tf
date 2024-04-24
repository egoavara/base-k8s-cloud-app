resource "kubernetes_namespace" "telemetry-system" {
  metadata {
    name   = "telemetry-system"
    labels = merge(
      {
      },
      local.labels
    )
  }
}