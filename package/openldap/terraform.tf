terraform {
  required_providers {
    kubernetes = {
      source  = "hashicorp/kubernetes"
      version = "2.29.0"
    }
    helm = {
      source  = "hashicorp/helm"
      version = "2.13.1"
    }
    random = {
      source  = "hashicorp/random"
      version = "3.6.1"
    }
  }
  backend "kubernetes" {
    config_path    = "~/.kube/config"
    config_context = "k3d-egoavara-net"
    namespace      = "default"
    secret_suffix  = "openldap"
    labels = {
      "app.kubernetes.io/managed-by" = "terraform"
    }
  }
}

provider "kubernetes" {
  config_path    = "~/.kube/config"
  config_context = "k3d-egoavara-net"
}

provider "helm" {
  kubernetes {
    config_path    = "~/.kube/config"
    config_context = "k3d-egoavara-net"
  }
}

provider "random" {

}
