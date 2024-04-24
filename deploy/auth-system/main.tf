resource "kubernetes_namespace" "auth-system" {
  metadata {
    name   = "auth-system"
    labels = merge(
      {
      },
      local.labels
    )
  }
}