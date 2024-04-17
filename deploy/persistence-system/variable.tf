variable "ldap-users" {
  type    = set(string)
  default = ["svc"]
}

variable "default-password-length" {
  type    = number
  default = 64
}