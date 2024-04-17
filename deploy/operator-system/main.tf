resource "kubernetes_namespace" "operator-system" {
  metadata {
    name   = "operator-system"
    labels = merge(
      {
        istio-injection = "disabled"
      },
      local.labels
    )
  }
}

resource "helm_release" "cert-manager" {
  depends_on = [kubernetes_namespace.operator-system]

  repository = "https://charts.jetstack.io"
  chart      = "cert-manager"
  name       = "cert-manager"
  version    = "1.14.4"

  namespace        = kubernetes_namespace.operator-system.metadata[0].name
  create_namespace = false
  values           = [
    file("${path.module}/files/cert-manager.yaml")
  ]
}

resource "helm_release" "prometheus-operator" {
  depends_on = [kubernetes_namespace.operator-system]

  repository = "https://prometheus-community.github.io/helm-charts"
  chart      = "kube-prometheus-stack"
  name       = "prometheus-operator"
  version    = "55.5.1"

  namespace        = kubernetes_namespace.operator-system.metadata[0].name
  create_namespace = false
  values           = [
    file("${path.module}/files/prometheus-operator.yaml")
  ]
}

resource "helm_release" "eck-operator" {
  depends_on = [kubernetes_namespace.operator-system]

  repository = "https://helm.elastic.co"
  chart      = "eck-operator"
  name       = "eck-operator"
  version    = "2.10.0"

  namespace        = kubernetes_namespace.operator-system.metadata[0].name
  create_namespace = false
  values           = [
    file("${path.module}/files/eck-operator.yaml")
  ]
}

resource "helm_release" "confluent-operator" {
  depends_on = [kubernetes_namespace.operator-system]

  repository = "https://packages.confluent.io/helm"
  chart      = "confluent-for-kubernetes"
  name       = "confluent-operator"
  version    = "0.824.33"

  namespace        = kubernetes_namespace.operator-system.metadata[0].name
  create_namespace = false
  values           = [
    file("${path.module}/files/confluent-operator.yaml")
  ]
}

resource "helm_release" "jaeger-operator" {
  depends_on = [kubernetes_namespace.operator-system]

  repository = "https://jaegertracing.github.io/helm-charts"
  chart      = "jaeger-operator"
  name       = "jaeger-operator"
  version    = "2.49.0"

  namespace        = kubernetes_namespace.operator-system.metadata[0].name
  create_namespace = false
  values           = [
    file("${path.module}/files/jaeger-operator.yaml")
  ]
}

resource "helm_release" "kiali-operator" {
  depends_on = [kubernetes_namespace.operator-system]

  repository = "https://kiali.org/helm-charts"
  chart      = "kiali-operator"
  name       = "kiali-operator"
  version    = "1.78.0"

  namespace        = kubernetes_namespace.operator-system.metadata[0].name
  create_namespace = false
  values           = [
    file("${path.module}/files/kiali-operator.yaml")
  ]
}

resource "helm_release" "minio-operator" {
  depends_on = [kubernetes_namespace.operator-system]

  repository = "https://operator.min.io"
  chart      = "operator"
  name       = "minio-operator"
  version    = "5.0.11"

  namespace        = kubernetes_namespace.operator-system.metadata[0].name
  create_namespace = false
  values           = [
    file("${path.module}/files/minio-operator.yaml")
  ]
}