resource "helm_release" "openfga" {
  depends_on = [kubernetes_namespace.auth-system]

  repository = "https://openfga.github.io/helm-charts"
  chart      = "openfga"
  name       = "openfga"
  version    = "0.2.3"

  namespace        = kubernetes_namespace.auth-system.metadata.0.name
  create_namespace = false
  values           = [
    yamlencode({
      replicaCount = 1,
      datastore    = {
        engine    = "postgres",
        uriSecret = "openfga-datastore"
      },
      
      serviceAccount = {
        create = true,
      }

      postgresql = {
        enabled = false,
      },

      playground = {
        enabled = true,
        port    = 3000
      },

      ingress = {
        enabled = false
      },

      telemetry = {
        trace = {
          enabled = true,
          otlp    = {
            endpoint = "jaeger-collector.telemetry-system.svc:4317",
            tls      = {
              enabled = false
            }
          },
          sampleRatio = 100
        }
      },
      autoscaling = {
        enabled = false,
      },
    })
  ]
}

data "kubernetes_secret" "postgres" {
  metadata {
    name      = "postgres"
    namespace = "persistence-system"
  }
}

locals {
  postgres_username = data.kubernetes_secret.postgres.data.default-username
  postgres_password = data.kubernetes_secret.postgres.data.default-password
}

resource "kubernetes_secret" "openfga-datastore" {
  metadata {
    name      = "openfga-datastore"
    namespace = kubernetes_namespace.auth-system.metadata.0.name
  }

  data = {
    uri = "postgres://${local.postgres_username}:${local.postgres_password}@postgresql.persistence-system.svc:5432/openfga?sslmode=disable",
  }
}