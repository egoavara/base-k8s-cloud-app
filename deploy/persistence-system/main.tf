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
        binddn = "cn=${local.openldap_admin_username},${local.openldap_root_dn}"
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

resource "helm_release" "redis" {
  depends_on = [kubernetes_namespace.persistence-system]

  repository = "https://charts.bitnami.com/bitnami"
  chart      = "redis-cluster"
  name       = "redis"
  version    = "10.0.2"

  namespace        = kubernetes_namespace.persistence-system.metadata[0].name
  create_namespace = false
  values           = [
    yamlencode({
      cluster = {
        nodes    = 6
        replicas = 1
      }
      usePassword     = true
      usePasswordFile = true
      existingSecret  = kubernetes_secret.redis.metadata[0].name

      metrics = {
        enabled = true
      }
    })
  ]
}

resource "helm_release" "neo4j" {
  depends_on = [kubernetes_namespace.persistence-system]

  repository = "https://helm.neo4j.com/neo4j"
  chart      = "neo4j"
  name       = "neo4j"
  version    = "5.14.0"

  namespace        = kubernetes_namespace.persistence-system.metadata[0].name
  create_namespace = false
  values           = [
    yamlencode({
      disableLookups = true
      neo4j          = {
        name               = "neo4j"
        passwordFromSecret = "neo4j"
      }
      volumes = {
        data = {
          mode = "defaultStorageClass"
        }
      }
    })
  ]
}