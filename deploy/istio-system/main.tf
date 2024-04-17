resource "kubernetes_namespace" "istio-system" {
  metadata {
    name   = "istio-system"
    labels = merge(
      {
      },
      local.labels
    )
  }
}

resource "helm_release" "istio-base" {
  depends_on = [kubernetes_namespace.istio-system]

  repository = "https://istio-release.storage.googleapis.com/charts"
  chart      = "base"
  name       = "istio-base"
  version    = "1.20.1"

  namespace        = kubernetes_namespace.istio-system.metadata[0].name
  create_namespace = false
  values           = [
    file("${path.module}/files/istio-base.yaml")
  ]
}

resource "helm_release" "istio-istiod" {
  depends_on = [kubernetes_namespace.istio-system, helm_release.istio-base]

  repository = "https://istio-release.storage.googleapis.com/charts"
  chart      = "istiod"
  name       = "istio-istiod"
  version    = "1.20.1"

  namespace        = kubernetes_namespace.istio-system.metadata[0].name
  create_namespace = false
  values           = [
    file("${path.module}/files/istio-istiod.yaml")
  ]
}


resource "helm_release" "istio-ingressgateway" {
  depends_on = [kubernetes_namespace.istio-system, helm_release.istio-istiod]

  repository = "https://istio-release.storage.googleapis.com/charts"
  chart      = "gateway"
  name       = "istio-ingressgateway"
  version    = "1.20.1"

  namespace        = kubernetes_namespace.istio-system.metadata[0].name
  create_namespace = false
  values           = [
    file("${path.module}/files/istio-ingressgateway.yaml")
  ]
}


resource "helm_release" "istio-egressgateway" {
  depends_on = [kubernetes_namespace.istio-system, helm_release.istio-istiod]

  repository = "https://istio-release.storage.googleapis.com/charts"
  chart      = "gateway"
  name       = "istio-egressgateway"
  version    = "1.20.1"

  namespace        = kubernetes_namespace.istio-system.metadata[0].name
  create_namespace = false
  values           = [
    file("${path.module}/files/istio-egressgateway.yaml")
  ]
}
