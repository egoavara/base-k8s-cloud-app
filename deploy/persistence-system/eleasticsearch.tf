resource "kubernetes_manifest" "elasticsearch" {
  manifest = {
    apiVersion = "elasticsearch.k8s.elastic.co/v1"
    kind       = "Elasticsearch"
    metadata   = {
      name      = "elasticsearch"
      namespace = kubernetes_namespace.persistence-system.metadata[0].name
    }
    spec = {
      version  = "8.11.3"
      nodeSets = [
        {
          name   = "default"
          count  = 1
          config = {
            "xpack.security.authc" = {
              anonymous = {
                username        = "anonymous"
                roles           = "superuser"
                authz_exception = false
              }
            }
            "node.store.allow_mmap" = false
          }
          volumeClaimTemplates = [
            {
              metadata = {
                name = "elasticsearch-data"
              }
              spec = {
                accessModes = ["ReadWriteOnce"]
                resources   = {
                  requests = {
                    storage = "5Gi"
                  }
                }
                storageClassName = "hostpath"
              }
            }
          ]
        }
      ]
      http = {
        tls = {
          selfSignedCertificate = {
            disabled = true
          }
        }
      }
    }
  }
}

resource "kubernetes_manifest" "kibana" {
  manifest = {
    apiVersion = "kibana.k8s.elastic.co/v1"
    kind       = "Kibana"
    metadata   = {
      name      = "kibana"
      namespace = kubernetes_namespace.persistence-system.metadata[0].name
    }
    spec = {
      version          = "8.11.3"
      count            = 1
      elasticsearchRef = {
        name = kubernetes_manifest.elasticsearch.manifest.metadata.name
      }
    }
  }
}