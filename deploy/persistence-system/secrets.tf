locals {

  postgres-username = "postgres-user"
  etcd-username     = "etcd-user"
  neo4j-username    = "neo4j-user"

  internal-ldap-users = toset([local.postgres-username, local.etcd-username, local.neo4j-username])
  total-ldap-users    = setunion(var.ldap-users, local.internal-ldap-users)
}
resource "random_password" "ldap-admin-password" {
  length           = var.default-password-length
  special          = true
  override_special = "-_"
}

resource "random_password" "ldap-user-password" {
  for_each = toset(local.total-ldap-users)

  length           = var.default-password-length
  special          = true
  override_special = "-_"
}

########################################################
# ldap

resource "kubernetes_secret" "ldap" {
  metadata {
    name      = "ldap"
    namespace = kubernetes_namespace.persistence-system.metadata[0].name
    labels    = merge(
      {},
      local.labels,
    )
  }
  data = {
    admin-password = random_password.ldap-admin-password.result
    usernames      = join(",", [for k, v in random_password.ldap-user-password : k])
    passwords      = join(",", [for k, v in random_password.ldap-user-password : v.result])
  }
}
########################################################
# etcd

resource "kubernetes_secret" "etcd" {
  metadata {
    name      = "etcd"
    namespace = kubernetes_namespace.persistence-system.metadata[0].name
    labels    = merge(
      {},
      local.labels,
    )
  }
  data = {
    root-password = random_password.ldap-user-password[local.etcd-username].result
  }
}

########################################################
# neo4j
#resource "kubernetes_secret" "neo4j" {}

########################################################
# postgres
resource "kubernetes_secret" "postgres" {
  metadata {
    name      = "postgres"
    namespace = kubernetes_namespace.persistence-system.metadata[0].name
    labels    = merge(
      {},
      local.labels,
    )
  }
  data = {
    postgres-password = ""
    ldap-password     = random_password.ldap-user-password[local.postgres-username].result
  }
}
