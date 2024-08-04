resource "helm_release" "airflow" {
  repository = "https://airflow-helm.github.io/charts"
  chart      = "airflow"
  name       = "airflow"
  version    = "8.9.0"

  namespace = var.namespace
  values = [
    yamlencode({
      airflow = {
        # users            = [] # For ldap, it must be empty
        extraPipPackages = var.pip
      }
      #       web = {
      #         webserverConfig = {
      #           stringOverride = <<EOF
      # from flask_appbuilder.security.manager import AUTH_LDAP
      # AUTH_TYPE = AUTH_LDAP
      # AUTH_LDAP_SERVER = "ldap://${var.ldap-host}:${var.ldap-port}"
      # AUTH_LDAP_USE_TLS = False

      # # registration configs
      # AUTH_USER_REGISTRATION = False

      # # search configs
      # AUTH_LDAP_SEARCH = "${var.ldap-search}"
      # AUTH_LDAP_UID_FIELD = "uid"
      # AUTH_LDAP_BIND_USER = "${var.ldap-user}"
      # AUTH_LDAP_BIND_PASSWORD = "${var.ldap-password}"
      # # a mapping from LDAP DN to a list of FAB roles
      # AUTH_ROLES_MAPPING = {
      #     "${var.ldap-search}": ["Admin"],
      # }

      # # if we should replace ALL the user's roles each login, or only on registration
      # AUTH_ROLES_SYNC_AT_LOGIN = True

      # # force users to re-auth after 30min of inactivity (to keep roles in sync)
      # PERMANENT_SESSION_LIFETIME = 1800
      # EOF
      #         }
      #       }
    })
  ]
}
