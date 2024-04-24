resource "kubernetes_manifest" "jaeger" {
  manifest = {
    apiVersion = "jaegertracing.io/v1"
    kind       = "Jaeger"
    metadata   = {
      name      = "jaeger"
      namespace = kubernetes_namespace.telemetry-system.metadata.0.name
    }
    spec = {
      strategy = "production"
      ingress  = {
        enabled = false
      }
      agent = {
        strategy = "DaemonSet"
      }
      storage = {
        elasticsearch = {
          doNotProvision = true
        }
        type    = "elasticsearch"
        options = {
          es = {
            server-urls  = var.jaeger-es-url
            index-prefix = "jaeger"
          }
        }
      }
    }
  }
}