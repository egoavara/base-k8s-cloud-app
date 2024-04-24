resource "kubernetes_namespace" "event-system" {
  metadata {
    name   = "event-system"
    labels = merge(
      {
      },
      local.labels
    )
  }
}