output "postgres-password" {
  sensitive = true
  value     = random_password.ldap-user-password[local.postgres-username].result
}

output "redis-password" {
  sensitive = true
  value     = random_password.ldap-user-password[local.redis-username].result
}

output "svc-password" {
  sensitive = true
  value     = random_password.ldap-user-password["svc"].result
}