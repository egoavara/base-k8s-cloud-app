
resource "random_password" "ldap-admin-password" {
  length           = 32
  special          = true
  override_special = "-_"
}
resource "random_password" "ldap-user-password" {
  for_each = var.ldap-users

  length           = 32
  special          = true
  override_special = "-_"
}


resource "kubernetes_secret" "ldap" {
  metadata {
    name      = "ldap"
    namespace = var.namespace
    labels = merge(
      {},
      local.labels,
    )
  }
  data = {
    admin-password = random_password.ldap-admin-password.result
    usernames      = join(",", [for k in var.ldap-users : k])
    passwords      = join(",", [for k in var.ldap-users : random_password.ldap-user-password[k].result])
  }
}
