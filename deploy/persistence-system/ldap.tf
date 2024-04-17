locals {
  openldap_labels = {
    "app.kubernetes.io/name"      = "openldap"
    "app.kubernetes.io/instance"  = "openldap"
    "app.kubernetes.io/version"   = "2.6.7"
    "app.kubernetes.io/component" = "openldap"
  }
  openldap_match_labels = {
    "app.kubernetes.io/name"     = "openldap"
    "app.kubernetes.io/instance" = "openldap"
    "app.kubernetes.io/version"  = "2.6.7"
  }

  openldap_root_dn        = "dc=egoavara,dc=net"
  openldap_admin_username = "admin"
}

resource "kubernetes_stateful_set" "openldap" {
  metadata {
    name      = "openldap"
    namespace = kubernetes_namespace.persistence-system.metadata[0].name
    labels    = local.openldap_labels
  }
  spec {
    selector {
      match_labels = local.openldap_match_labels
    }
    replicas = 1
    template {
      metadata {
        labels = local.openldap_match_labels
      }
      spec {
        container {
          name  = "openldap"
          image = "docker.io/bitnami/openldap:2.6.7"
          env {
            name  = "LDAP_PORT_NUMBER"
            value = "389"
          }
          env {
            name  = "LDAP_ROOT"
            value = local.openldap_root_dn
          }
          env {
            name  = "LDAP_ADMIN_USERNAME"
            value = local.openldap_admin_username
          }
          env {
            name = "LDAP_ADMIN_PASSWORD"
            value_from {
              secret_key_ref {
                key  = "admin-password"
                name = "ldap"
              }
            }
          }
          env {
            name = "LDAP_USERS"
            value_from {
              secret_key_ref {
                key  = "usernames"
                name = "ldap"
              }
            }
          }
          env {
            name = "LDAP_PASSWORDS"
            value_from {
              secret_key_ref {
                key  = "passwords"
                name = "ldap"
              }
            }
          }
          port {
            name           = "tcp-ldap"
            container_port = 389
          }
        }
      }
    }
    service_name = "openldap"
  }
}

resource "kubernetes_service" "openldap" {
  metadata {
    name      = "openldap"
    namespace = kubernetes_namespace.persistence-system.metadata[0].name
    labels    = local.openldap_labels
  }
  spec {
    selector = local.openldap_match_labels
    port {
      name        = "tcp-ldap"
      port        = 389
      target_port = "tcp-ldap"
    }
  }
}