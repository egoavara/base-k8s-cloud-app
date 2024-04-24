locals {

  postgres-username = "postgres-user"
  etcd-username     = "etcd-user"
  redis-username    = "redis-user"
  neo4j-username    = "neo4j"

  internal-ldap-users = toset([local.postgres-username, local.etcd-username, local.neo4j-username, local.redis-username])
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
    usernames      = join(",", [for k in local.total-ldap-users : k])
    passwords      = join(",", [for k in local.total-ldap-users : random_password.ldap-user-password[k].result])
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
    ldap-password     = random_password.ldap-admin-password.result
    default-username  = local.postgres-username
    default-password  = random_password.ldap-user-password[local.postgres-username].result
  }
}

########################################################
# redis
resource "kubernetes_secret" "redis" {
  metadata {
    name      = "redis"
    namespace = kubernetes_namespace.persistence-system.metadata[0].name
    labels    = merge(
      {},
      local.labels,
    )
  }
  data = {
    redis-password = random_password.ldap-user-password[local.redis-username].result
  }
}

########################################################
# neo4j
resource "kubernetes_secret" "neo4j" {
  metadata {
    name      = "neo4j"
    namespace = kubernetes_namespace.persistence-system.metadata[0].name
    labels    = merge(
      {},
      local.labels,
    )
  }
  data = {
    NEO4J_AUTH = "${local.neo4j-username}/${random_password.ldap-user-password[local.neo4j-username].result}"
  }
}
