resource "kubernetes_namespace" "persistence-system" {
  metadata {
    name   = "persistence-system"
    labels = merge(
      {
        "istio-injection" = "disabled"
      },
      local.labels
    )
  }
}


resource "helm_release" "postgresql" {
  depends_on = [kubernetes_namespace.persistence-system]

  repository = "https://charts.bitnami.com/bitnami"
  chart      = "postgresql"
  name       = "postgresql"
  version    = "13.2.3"

  namespace        = kubernetes_namespace.persistence-system.metadata[0].name
  create_namespace = false
  values           = [
    file("${path.module}/files/postgres.yaml"),
    yamlencode({
      auth = {
        existingSecret = kubernetes_secret.postgres.metadata[0].name
      },
      ldap = {
        server = "${kubernetes_service.openldap.metadata[0].name}.${kubernetes_service.openldap.metadata[0].namespace}.svc.cluster.local"
        port   = 389
        basedn = local.openldap_root_dn
        binddn = "cn=${local.postgres-username},${local.openldap_root_dn}"
        bindpw = "auto"
      },
      primary = {
        initdb = {
          scripts = {
            "users.sql" = <<-EOT
              create user "admin" with superuser;
              create user "${local.postgres-username}" with superuser;
            EOT
          }
        }
      },
    }),
  ]
}


resource "helm_release" "etcd" {
  depends_on = [kubernetes_namespace.persistence-system]

  repository = "https://charts.bitnami.com/bitnami"
  chart      = "etcd"
  name       = "etcd"
  version    = "10.0.3"

  namespace        = kubernetes_namespace.persistence-system.metadata[0].name
  create_namespace = false
  values           = [
    file("${path.module}/files/etcd.yaml"),
    yamlencode({
      auth = {
        rbac : {
          allowNoneAuthentication   = false
          existingSecret            = kubernetes_secret.etcd.metadata[0].name
          existingSecretPasswordKey = "root-password"
        },
      },
    })
  ]
}

# postgres for svc
#- repo: https://charts.bitnami.com/bitnami
#name: postgresql
#namespace: persistence
#releaseName: postgres-svc
#version: 13.2.3
#valuesFile: ./values/postgres-svc.yaml
## etcd for dex
#- repo: https://charts.bitnami.com/bitnami
#name: etcd
#namespace: persistence
#releaseName: etcd-dex
#version: 9.8.0
#valuesFile: ./values/etcd-dex.yaml
## redis (key-value store)
#  - releaseName: redis
#    repo: https://charts.bitnami.com/bitnami
#    name: redis
#    version: 18.3.0
#    valuesFile: ./values/redis.yaml
# neo4j (graph database)
#  - releaseName: neo4j
#    repo: https://helm.neo4j.com/neo4j
#    name: neo4j
#    version: 5.14.0
#    valuesFile: ./values/neo4j.yaml