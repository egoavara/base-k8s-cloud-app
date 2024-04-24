resource "helm_release" "prometheus-grafana" {
  depends_on = [kubernetes_namespace.telemetry-system]

  repository = "https://prometheus-community.github.io/helm-charts"
  chart      = "kube-prometheus-stack"
  name       = "prometheus-grafana"
  version    = "55.5.1"

  namespace        = kubernetes_namespace.telemetry-system.metadata[0].name
  create_namespace = false
  values           = [
    yamlencode({
      crds = {
        enabled = false
      }
      windowsMonitoring = {
        enabled = true
      }
      alertmanager = {
        enabled = true
      }
      grafana = {
        enabled = true

        "grafana.ini" = {
          paths = {
            data         = "/var/lib/grafana/"
            logs         = "/var/log/grafana"
            plugins      = "/var/lib/grafana/plugins"
            provisioning = "/etc/grafana/provisioning"
          }
          anlytics = {
            check_for_updates = true
          }
          log = {
            mode = "console"
          }
          grafana_net = {
            url = "https://grafana.net"
          }
          server = {
            domain = "{{ if (and .Values.ingress.enabled .Values.ingress.hosts) }}{{ .Values.ingress.hosts | first }}{{ else }}''{{ end }}"
          }
          "auth.ldap" = { enabled = true }
        }
        ldap = {
          enabled        = true
          existingSecret = "grafana"
        }
      }

      kubernetesServiceMonitor = {
        enabled = true
      }
      kubeApiServer = {
        enabled = true
      }
      kubelet = {
        enabled = true
      }
      kubeControllerManager = {
        enabled = true
      }
      coreDns = {
        enabled = true
      }
      kubeScheduler = {
        enabled = true
      }
      kubeProxy = {
        enabled = true
      }
      kubeStateMetrics = {
        enabled = true
      }
      nodeExporter = {
        enabled = true
      }
      prometheusOperator = {
        enabled = false
      }

      prometheus = {
        enabled                       = true
        additionalRulesForClusterRole = [
          {
            apiGroups = [""]
            resources = ["nodes/proxy", "ingresses", "configmaps"]
            verbs     = ["get", "list", "watch"]
          },
          {
            apiGroups = ["extensions", "networking.k8s.io"]
            resources = ["ingresses/status", "ingresses"]
            verbs     = ["get", "list", "watch"]
          },
          {
            apiGroups = ["discovery.k8s.io"]
            resources = ["endpointslices"]
            verbs     = ["get", "list", "watch"]
          }
        ]
        "prometheusSpec" = {
          "additionalScrapeConfigs" = [
            {
              "bearer_token_file"     = "/var/run/secrets/kubernetes.io/serviceaccount/token"
              "job_name"              = "kubernetes-apiservers"
              "kubernetes_sd_configs" = [
                {
                  "role" = "endpoints"
                },
              ]
              "relabel_configs" = [
                {
                  "action"        = "keep"
                  "regex"         = "default;kubernetes;https"
                  "source_labels" = [
                    "__meta_kubernetes_namespace",
                    "__meta_kubernetes_service_name",
                    "__meta_kubernetes_endpoint_port_name",
                  ]
                },
              ]
              "scheme"     = "https"
              "tls_config" = {
                "ca_file"              = "/var/run/secrets/kubernetes.io/serviceaccount/ca.crt"
                "insecure_skip_verify" = true
              }
            },
            {
              "bearer_token_file"     = "/var/run/secrets/kubernetes.io/serviceaccount/token"
              "job_name"              = "kubernetes-nodes"
              "kubernetes_sd_configs" = [
                {
                  "role" = "node"
                },
              ]
              "relabel_configs" = [
                {
                  "action" = "labelmap"
                  "regex"  = "__meta_kubernetes_node_label_(.+)"
                },
                {
                  "replacement"  = "kubernetes.default.svc:443"
                  "target_label" = "__address__"
                },
                {
                  "regex"         = "(.+)"
                  "replacement"   = "/api/v1/nodes/$1/proxy/metrics"
                  "source_labels" = [
                    "__meta_kubernetes_node_name",
                  ]
                  "target_label" = "__metrics_path__"
                },
              ]
              "scheme"     = "https"
              "tls_config" = {
                "ca_file"              = "/var/run/secrets/kubernetes.io/serviceaccount/ca.crt"
                "insecure_skip_verify" = true
              }
            },
            {
              "bearer_token_file"     = "/var/run/secrets/kubernetes.io/serviceaccount/token"
              "job_name"              = "kubernetes-nodes-cadvisor"
              "kubernetes_sd_configs" = [
                {
                  "role" = "node"
                },
              ]
              "relabel_configs" = [
                {
                  "action" = "labelmap"
                  "regex"  = "__meta_kubernetes_node_label_(.+)"
                },
                {
                  "replacement"  = "kubernetes.default.svc:443"
                  "target_label" = "__address__"
                },
                {
                  "regex"         = "(.+)"
                  "replacement"   = "/api/v1/nodes/$1/proxy/metrics/cadvisor"
                  "source_labels" = [
                    "__meta_kubernetes_node_name",
                  ]
                  "target_label" = "__metrics_path__"
                },
              ]
              "scheme"     = "https"
              "tls_config" = {
                "ca_file"              = "/var/run/secrets/kubernetes.io/serviceaccount/ca.crt"
                "insecure_skip_verify" = true
              }
            },
            {
              "honor_labels"          = true
              "job_name"              = "kubernetes-service-endpoints"
              "kubernetes_sd_configs" = [
                {
                  "role" = "endpoints"
                },
              ]
              "relabel_configs" = [
                {
                  "action"        = "keep"
                  "regex"         = true
                  "source_labels" = [
                    "__meta_kubernetes_service_annotation_prometheus_io_scrape",
                  ]
                },
                {
                  "action"        = "drop"
                  "regex"         = true
                  "source_labels" = [
                    "__meta_kubernetes_service_annotation_prometheus_io_scrape_slow",
                  ]
                },
                {
                  "action"        = "replace"
                  "regex"         = "(https?)"
                  "source_labels" = [
                    "__meta_kubernetes_service_annotation_prometheus_io_scheme",
                  ]
                  "target_label" = "__scheme__"
                },
                {
                  "action"        = "replace"
                  "regex"         = "(.+)"
                  "source_labels" = [
                    "__meta_kubernetes_service_annotation_prometheus_io_path",
                  ]
                  "target_label" = "__metrics_path__"
                },
                {
                  "action"        = "replace"
                  "regex"         = "(.+?)(?::\\d+)?;(\\d+)"
                  "replacement"   = "$1:$2"
                  "source_labels" = [
                    "__address__",
                    "__meta_kubernetes_service_annotation_prometheus_io_port",
                  ]
                  "target_label" = "__address__"
                },
                {
                  "action"      = "labelmap"
                  "regex"       = "__meta_kubernetes_service_annotation_prometheus_io_param_(.+)"
                  "replacement" = "__param_$1"
                },
                {
                  "action" = "labelmap"
                  "regex"  = "__meta_kubernetes_service_label_(.+)"
                },
                {
                  "action"        = "replace"
                  "source_labels" = [
                    "__meta_kubernetes_namespace",
                  ]
                  "target_label" = "namespace"
                },
                {
                  "action"        = "replace"
                  "source_labels" = [
                    "__meta_kubernetes_service_name",
                  ]
                  "target_label" = "service"
                },
                {
                  "action"        = "replace"
                  "source_labels" = [
                    "__meta_kubernetes_pod_node_name",
                  ]
                  "target_label" = "node"
                },
              ]
            },
            {
              "honor_labels"          = true
              "job_name"              = "kubernetes-service-endpoints-slow"
              "kubernetes_sd_configs" = [
                {
                  "role" = "endpoints"
                },
              ]
              "relabel_configs" = [
                {
                  "action"        = "keep"
                  "regex"         = true
                  "source_labels" = [
                    "__meta_kubernetes_service_annotation_prometheus_io_scrape_slow",
                  ]
                },
                {
                  "action"        = "replace"
                  "regex"         = "(https?)"
                  "source_labels" = [
                    "__meta_kubernetes_service_annotation_prometheus_io_scheme",
                  ]
                  "target_label" = "__scheme__"
                },
                {
                  "action"        = "replace"
                  "regex"         = "(.+)"
                  "source_labels" = [
                    "__meta_kubernetes_service_annotation_prometheus_io_path",
                  ]
                  "target_label" = "__metrics_path__"
                },
                {
                  "action"        = "replace"
                  "regex"         = "(.+?)(?::\\d+)?;(\\d+)"
                  "replacement"   = "$1:$2"
                  "source_labels" = [
                    "__address__",
                    "__meta_kubernetes_service_annotation_prometheus_io_port",
                  ]
                  "target_label" = "__address__"
                },
                {
                  "action"      = "labelmap"
                  "regex"       = "__meta_kubernetes_service_annotation_prometheus_io_param_(.+)"
                  "replacement" = "__param_$1"
                },
                {
                  "action" = "labelmap"
                  "regex"  = "__meta_kubernetes_service_label_(.+)"
                },
                {
                  "action"        = "replace"
                  "source_labels" = [
                    "__meta_kubernetes_namespace",
                  ]
                  "target_label" = "namespace"
                },
                {
                  "action"        = "replace"
                  "source_labels" = [
                    "__meta_kubernetes_service_name",
                  ]
                  "target_label" = "service"
                },
                {
                  "action"        = "replace"
                  "source_labels" = [
                    "__meta_kubernetes_pod_node_name",
                  ]
                  "target_label" = "node"
                },
              ]
              "scrape_interval" = "5m"
              "scrape_timeout"  = "30s"
            },
            {
              "honor_labels"          = true
              "job_name"              = "prometheus-pushgateway"
              "kubernetes_sd_configs" = [
                {
                  "role" = "service"
                },
              ]
              "relabel_configs" = [
                {
                  "action"        = "keep"
                  "regex"         = "pushgateway"
                  "source_labels" = [
                    "__meta_kubernetes_service_annotation_prometheus_io_probe",
                  ]
                },
              ]
            },
            {
              "honor_labels"          = true
              "job_name"              = "kubernetes-services"
              "kubernetes_sd_configs" = [
                {
                  "role" = "service"
                },
              ]
              "metrics_path" = "/probe"
              "params"       = {
                "module" = [
                  "http_2xx",
                ]
              }
              "relabel_configs" = [
                {
                  "action"        = "keep"
                  "regex"         = true
                  "source_labels" = [
                    "__meta_kubernetes_service_annotation_prometheus_io_probe",
                  ]
                },
                {
                  "source_labels" = [
                    "__address__",
                  ]
                  "target_label" = "__param_target"
                },
                {
                  "replacement"  = "blackbox"
                  "target_label" = "__address__"
                },
                {
                  "source_labels" = [
                    "__param_target",
                  ]
                  "target_label" = "instance"
                },
                {
                  "action" = "labelmap"
                  "regex"  = "__meta_kubernetes_service_label_(.+)"
                },
                {
                  "source_labels" = [
                    "__meta_kubernetes_namespace",
                  ]
                  "target_label" = "namespace"
                },
                {
                  "source_labels" = [
                    "__meta_kubernetes_service_name",
                  ]
                  "target_label" = "service"
                },
              ]
            },
            {
              "honor_labels"          = true
              "job_name"              = "kubernetes-pods"
              "kubernetes_sd_configs" = [
                {
                  "role" = "pod"
                },
              ]
              "relabel_configs" = [
                {
                  "action"        = "keep"
                  "regex"         = true
                  "source_labels" = [
                    "__meta_kubernetes_pod_annotation_prometheus_io_scrape",
                  ]
                },
                {
                  "action"        = "drop"
                  "regex"         = true
                  "source_labels" = [
                    "__meta_kubernetes_pod_annotation_prometheus_io_scrape_slow",
                  ]
                },
                {
                  "action"        = "replace"
                  "regex"         = "(https?)"
                  "source_labels" = [
                    "__meta_kubernetes_pod_annotation_prometheus_io_scheme",
                  ]
                  "target_label" = "__scheme__"
                },
                {
                  "action"        = "replace"
                  "regex"         = "(.+)"
                  "source_labels" = [
                    "__meta_kubernetes_pod_annotation_prometheus_io_path",
                  ]
                  "target_label" = "__metrics_path__"
                },
                {
                  "action"        = "replace"
                  "regex"         = "(\\d+);(([A-Fa-f0-9]{1,4}::?){1,7}[A-Fa-f0-9]{1,4})"
                  "replacement"   = "[$2]:$1"
                  "source_labels" = [
                    "__meta_kubernetes_pod_annotation_prometheus_io_port",
                    "__meta_kubernetes_pod_ip",
                  ]
                  "target_label" = "__address__"
                },
                {
                  "action"        = "replace"
                  "regex"         = "(\\d+);((([0-9]+?)(\\.|$)){4})"
                  "replacement"   = "$2:$1"
                  "source_labels" = [
                    "__meta_kubernetes_pod_annotation_prometheus_io_port",
                    "__meta_kubernetes_pod_ip",
                  ]
                  "target_label" = "__address__"
                },
                {
                  "action"      = "labelmap"
                  "regex"       = "__meta_kubernetes_pod_annotation_prometheus_io_param_(.+)"
                  "replacement" = "__param_$1"
                },
                {
                  "action" = "labelmap"
                  "regex"  = "__meta_kubernetes_pod_label_(.+)"
                },
                {
                  "action"        = "replace"
                  "source_labels" = [
                    "__meta_kubernetes_namespace",
                  ]
                  "target_label" = "namespace"
                },
                {
                  "action"        = "replace"
                  "source_labels" = [
                    "__meta_kubernetes_pod_name",
                  ]
                  "target_label" = "pod"
                },
                {
                  "action"        = "drop"
                  "regex"         = "Pending|Succeeded|Failed|Completed"
                  "source_labels" = [
                    "__meta_kubernetes_pod_phase",
                  ]
                },
                {
                  "action"        = "replace"
                  "source_labels" = [
                    "__meta_kubernetes_pod_node_name",
                  ]
                  "target_label" = "node"
                },
              ]
            },
            {
              "honor_labels"          = true
              "job_name"              = "kubernetes-pods-slow"
              "kubernetes_sd_configs" = [
                {
                  "role" = "pod"
                },
              ]
              "relabel_configs" = [
                {
                  "action"        = "keep"
                  "regex"         = true
                  "source_labels" = [
                    "__meta_kubernetes_pod_annotation_prometheus_io_scrape_slow",
                  ]
                },
                {
                  "action"        = "replace"
                  "regex"         = "(https?)"
                  "source_labels" = [
                    "__meta_kubernetes_pod_annotation_prometheus_io_scheme",
                  ]
                  "target_label" = "__scheme__"
                },
                {
                  "action"        = "replace"
                  "regex"         = "(.+)"
                  "source_labels" = [
                    "__meta_kubernetes_pod_annotation_prometheus_io_path",
                  ]
                  "target_label" = "__metrics_path__"
                },
                {
                  "action"        = "replace"
                  "regex"         = "(\\d+);(([A-Fa-f0-9]{1,4}::?){1,7}[A-Fa-f0-9]{1,4})"
                  "replacement"   = "[$2]:$1"
                  "source_labels" = [
                    "__meta_kubernetes_pod_annotation_prometheus_io_port",
                    "__meta_kubernetes_pod_ip",
                  ]
                  "target_label" = "__address__"
                },
                {
                  "action"        = "replace"
                  "regex"         = "(\\d+);((([0-9]+?)(\\.|$)){4})"
                  "replacement"   = "$2:$1"
                  "source_labels" = [
                    "__meta_kubernetes_pod_annotation_prometheus_io_port",
                    "__meta_kubernetes_pod_ip",
                  ]
                  "target_label" = "__address__"
                },
                {
                  "action"      = "labelmap"
                  "regex"       = "__meta_kubernetes_pod_annotation_prometheus_io_param_(.+)"
                  "replacement" = "__param_$1"
                },
                {
                  "action" = "labelmap"
                  "regex"  = "__meta_kubernetes_pod_label_(.+)"
                },
                {
                  "action"        = "replace"
                  "source_labels" = [
                    "__meta_kubernetes_namespace",
                  ]
                  "target_label" = "namespace"
                },
                {
                  "action"        = "replace"
                  "source_labels" = [
                    "__meta_kubernetes_pod_name",
                  ]
                  "target_label" = "pod"
                },
                {
                  "action"        = "drop"
                  "regex"         = "Pending|Succeeded|Failed|Completed"
                  "source_labels" = [
                    "__meta_kubernetes_pod_phase",
                  ]
                },
                {
                  "action"        = "replace"
                  "source_labels" = [
                    "__meta_kubernetes_pod_node_name",
                  ]
                  "target_label" = "node"
                },
              ]
              "scrape_interval" = "5m"
              "scrape_timeout"  = "30s"
            },
          ]
        }
      }

      thanosRuler = {
        enabled = false
      }
      prometheus-node-exporter = {
        hostRootFsMount = {
          enabled = false
        }
      }
    })
  ]
}

data "kubernetes_secret" "ldap" {
  metadata {
    name      = "ldap"
    namespace = "persistence-system"
  }
}

locals {
  ldap_admin_password = data.kubernetes_secret.ldap.data["admin-password"]
  ldap_admin_username = "admin"
  ldap_admin_binddn   = "cn=${local.ldap_admin_username},dc=egoavara,dc=net"
  ldap_search_basedn  = "ou=users,dc=egoavara,dc=net"
  ldap_host           = "openldap.persistence-system.svc"
  ldap_port           = 389
  ldap_base_dn        = "dc=egoavara,dc=net"
}

resource "kubernetes_secret" "grafana" {
  metadata {
    name      = "grafana"
    namespace = kubernetes_namespace.telemetry-system.metadata[0].name
  }
  data = {
    "ldap-toml" = <<EOF
verbose_logging = false

[[servers]]
host = "${local.ldap_host}"
port = ${local.ldap_port}
use_ssl = false
start_tls = false
ssl_skip_verify = false

bind_dn = "${local.ldap_admin_binddn}"
bind_password = "${local.ldap_admin_password}"

search_base_dns = ["${local.ldap_search_basedn}"]
search_filter = "(uid=%s)"

# group_search_filter = "(&(objectClass=posixGroup)(memberUid=%s))"
# group_search_filter_user_attribute = "distinguishedName"
# group_search_base_dns = ["ou=groups,dc=grafana,dc=org"]

[servers.attributes]
# member_of = "memberOf"
username = "uid"
email = "uid"
EOF
  }
}