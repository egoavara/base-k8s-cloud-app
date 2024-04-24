resource "helm_release" "dex" {
  depends_on = [kubernetes_namespace.auth-system]

  repository = "https://charts.dexidp.io"
  chart      = "dex"
  name       = "dex"
  version    = "0.15.3"

  namespace        = kubernetes_namespace.auth-system.metadata.0.name
  create_namespace = false
  values           = [
    yamlencode({
      replicaCount = 1,
      configSecret = {
        create = false
        name   = "dex"
      }
    })
  ]
}

data "kubernetes_secret" "etcd" {
  metadata {
    name      = "etcd"
    namespace = "persistence-system"
  }
}

data "kubernetes_secret" "ldap" {
  metadata {
    name      = "ldap"
    namespace = "persistence-system"
  }
}


locals {
  etcd_username     = "root"
  etcd_password     = data.kubernetes_secret.etcd.data["root-password"]
  etcd_url          = "http://etcd.persistence-system.svc:2379"
  dex_build_clients = [
    for id, client in var.dex-static-clients : {
      id           = id
      secret       = random_password.dex-client[id].result
      redirectURIs = client.redirect_uris
      name         = client.name
      public       = client.public
    }
  ]
}

resource "random_password" "dex-client" {
  for_each         = toset([for id, _ in var.dex-static-clients : id])
  length           = 32
  special          = true
  override_special = "-_"
}

resource "kubernetes_secret" "dex-client" {
  for_each = {for _, value in local.dex_build_clients : value.id => value}

  metadata {
    name      = "dex-client-${each.key}"
    namespace = kubernetes_namespace.auth-system.metadata.0.name
  }
  data = {
    secret = each.value.secret
  }
}

resource "kubernetes_secret" "dex" {
  metadata {
    name      = "dex"
    namespace = kubernetes_namespace.auth-system.metadata.0.name
  }

  data = {
    "config.yaml" = yamlencode({
      issuer           = "https://dex.egoavara.net/"
      enablePasswordDB = false
      storage          = {
        type   = "etcd"
        config = {
          endpoints = [local.etcd_url]
          username  = local.etcd_username
          password  = local.etcd_password
          namespace = "dex/"
        }
      }
      staticClients = local.dex_build_clients
      connectors    = [
        {
          type   = "ldap"
          id     = "ldap"
          name   = "LDAP"
          config = {
            host               = "openldap.persistence-system.svc:389"
            insecureNoSSL      = true
            insecureSkipVerify = true
            bindDN             = "cn=admin,dc=egoavara,dc=net"
            bindPW             = data.kubernetes_secret.ldap.data["admin-password"]
            usernamePrompt     = "SSO Username"
            userSearch         = {
              baseDN                = "ou=users,dc=egoavara,dc=net"
              username              = "uid"
              idAttr                = "uid"
              emailAttr             = "uid"
              nameAttr              = "uid"
              preferredUsernameAttr = "uid"
            }
          }
        },
      ]
    })
  }
}