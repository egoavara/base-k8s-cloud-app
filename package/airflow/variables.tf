variable "namespace" {
  type        = string
  description = "Namespace to deploy the OpenLDAP"
  default     = "default"
}

variable "ldap-host" {
  type        = string
  description = "LDAP Host"
}

variable "ldap-port" {
  type        = number
  description = "LDAP Port"
}

variable "ldap-user" {
  type        = string
  description = "LDAP User"
}

variable "ldap-password" {
  type        = string
  description = "LDAP Password"
}
variable "ldap-search" {
  type        = string
  description = "LDAP Password"
}


variable "pip" {
  type        = list(string)
  description = "List of pip packages to install"
}

# variable "python" {
#   type        = string
#   description = "Python version"
#   default     = "3.9"
# }