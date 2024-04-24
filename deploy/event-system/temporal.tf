data "kubernetes_secret" "postgres" {
  metadata {
    name      = "postgres"
    namespace = "persistence-system"
  }
}

locals {
  postgres_host           = "postgresql-hl.persistence-system.svc"
  postgres_visiblity_host = "postgresql-hl.persistence-system.svc"

  postgres_username = data.kubernetes_secret.postgres.data.default-username
  postgres_password = data.kubernetes_secret.postgres.data.default-password
}

resource "kubernetes_secret" "temporal-postgres" {
  metadata {
    name      = "temporal-postgres"
    namespace = kubernetes_namespace.event-system.metadata.0.name
  }

  data = {
    password = local.postgres_password
  }
}

resource "kubernetes_secret" "temporal-visiblity-postgres" {
  metadata {
    name      = "temporal-visiblity-postgres"
    namespace = kubernetes_namespace.event-system.metadata.0.name
  }

  data = {
    password = local.postgres_password
  }
}


resource "helm_release" "temporal" {
  depends_on = [kubernetes_namespace.event-system]

  chart = "https://github.com/temporalio/helm-charts/releases/download/temporal-0.36.0/temporal-0.36.0.tgz"
  name  = "temporal"

  namespace        = kubernetes_namespace.event-system.metadata[0].name
  create_namespace = false
  values           = [
    # disable all batteries
    yamlencode({
      cassandra = {
        enabled = false
      },
      mysql = {
        enabled = false
      },
      postgresql = {
        enabled = false
      },
      prometheus = {
        enabled = false
      },
      grafana = {
        enabled = false
      },
      elasticsearch = {
        enabled = false
      },
    }),
    # schema setup
    yamlencode({
      schema = {
        setup = {
          enabled = true
        }
        update = {
          enabled = true
        }
      }
    }),
    # config
    yamlencode({
      server = {
        replicaCount = 1

        config = {
          persistence = {
            default = {
              driver = "sql"
              sql    = {
                driver          = "postgres12"
                host            = local.postgres_host
                port            = 5432
                database        = "temporal"
                user            = local.postgres_username
                existingSecret  = kubernetes_secret.temporal-postgres.metadata.0.name
                maxConns        = 20
                maxConnLifetime = "1h"
              }
            }
            visibility = {
              driver = "sql"
              sql    = {
                driver          = "postgres12"
                host            = local.postgres_visiblity_host
                port            = 5432
                database        = "temporal_visibility"
                user            = local.postgres_username
                existingSecret  = kubernetes_secret.temporal-visiblity-postgres.metadata.0.name
                maxConns        = 20
                maxConnLifetime = "1h"
              }
            }
          }
        }
      }
      elasticsearch = {
        enabled  = false,
        external = true,
        host     = "elasticsearch-es-default.persistence-system.svc"
        port     = "9200"
        version  = "v7"
        scheme   = "http"
        logLevel = "error"
      }
      prometheus = {
        alertmanager = {
          enabled = false
        }
        alertmanagerFiles = {
          "alertmanager.yml" = {}
        }
        kubeStateMetrics = {
          enabled = false
        }
        nodeExporter = {
          enabled = false
        }
        pushgateway = {
          enabled = false
        }
        server = {
          persistentVolume = {
            enabled = false
          }
          extraArgs = {
            "storage.tsdb.retention"          = "6h"
            "storage.tsdb.min-block-duration" = "2h"
            "storage.tsdb.max-block-duration" = "2h"
          }
          serverFiles = {
            alerts            = {}
            "prometheus.yaml" = {
              remote_write : [
                {
                  url : "http://prometheus-operated.telemetry-system.svc:9090/write"
                }
              ]
            }
            rules = {}
          }
        }
      }
    })
  ]
}