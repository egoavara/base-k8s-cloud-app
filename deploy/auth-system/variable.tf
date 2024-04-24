variable "dex-static-clients" {
  type = map(object({
    name          = string
    public        = bool
    redirect_uris = list(string)
  }))

  default = {
    "egoavara-net" = {
      redirect_uris = ["https://www.egoavara.net/callback"]
      name          = "www.egoavara.net"
      public        = true
    }
    "jaeger" = {
      redirect_uris = ["https://jaeger.egoavara.net/oauth2/callback"]
      name          = "jaeger"
      public        = true
    }
    "prometheus" = {
      redirect_uris = ["https://prometheus.egoavara.net/oauth2/callback"]
      name          = "prometheus"
      public        = true
    }
  }
}