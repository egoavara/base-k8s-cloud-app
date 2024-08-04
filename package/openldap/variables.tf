variable "ldap-users" {
  type    = set(string)
  default = ["svc"]
}

variable "namespace" {
  description = "Namespace to deploy the OpenLDAP"
  default     = "default"
}