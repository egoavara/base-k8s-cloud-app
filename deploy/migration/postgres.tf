data "kubernetes_secret" "postgres" {
  metadata {
    name      = "postgres"
    namespace = "persistence-system"
  }
}

locals {
  databases = [
    "openfga",
    "temporal",
    "temporal_visibility",
  ]
  raw_postgres_fileset = fileset("${path.module}/files/postgres-migration", "*.sql")
  postgres_each        = {
    for file in local.raw_postgres_fileset : regex("^(v[0-9]+\\.[0-9]+\\.[0-9]+)(_.+)?\\.sql$", file)[0] => {
      norm_filename = replace(
        file,
        "/^v([0-9]+)\\.([0-9]+)\\.([0-9]+)(?:_.+)?\\.sql$/",
        "v$1-$2-$3.sql"
      ),
      file     = file("${path.module}/files/postgres-migration/${file}")
      filepath = "${path.module}/files/postgres-migration/${file}"
    }
  }
  psql_commands = [for k, v in local.postgres_each : "psql -h postgresql-hl -f /opt/migrations/${v.norm_filename}"]
  psql_script   = join("\n", concat([
    "#!/bin/bash",
    <<EOT
%{ for database in local.databases ~}
psql -h postgresql-hl -tc "SELECT 1 FROM pg_database WHERE datname = '${database}'" | grep -q 1 || psql -h postgresql-hl -c "CREATE DATABASE ${database}"
%{ endfor ~}
    EOT
  ], local.psql_commands))
}

resource "kubernetes_config_map" "configs" {

  metadata {
    name      = "postgres-migration"
    namespace = data.kubernetes_secret.postgres.metadata[0].namespace
    labels    = merge(
      {
      },
      local.labels
    )
  }

  data = merge({
    "run.sh" = local.psql_script
  }, {
    for k, v in local.postgres_each : v.norm_filename => v.file
  })
}

resource "kubernetes_job" "postgres" {

  metadata {
    name      = "postgres-migration"
    namespace = data.kubernetes_secret.postgres.metadata[0].namespace
  }

  spec {
    template {
      metadata {
        annotations = {
          "sidecar.istio.io/inject" = "false"
        }
      }

      spec {
        container {
          name    = "postgres-migration"
          image   = "bitnami/postgresql:16.0.0-debian-11-r15"
          command = ["/bin/bash", "-f", "/opt/migrations/run.sh"]

          env {
            name  = "PGDATABASE"
            value = "public"
          }

          env {
            name = "PGUSER"
            value_from {
              secret_key_ref {
                name = data.kubernetes_secret.postgres.metadata[0].name
                key  = "default-username"
              }
            }
          }

          env {
            name = "PGPASSWORD"
            value_from {
              secret_key_ref {
                name = data.kubernetes_secret.postgres.metadata[0].name
                key  = "default-password"
              }
            }
          }

          volume_mount {
            name       = "migration"
            mount_path = "/opt/migrations"
          }
        }

        restart_policy = "Never"

        volume {
          name = "migration"

          config_map {
            name = kubernetes_config_map.configs.metadata[0].name
          }
        }
      }
    }
    backoff_limit = 5
  }
}


// apiVersion: batch/v1
//kind: Job
//metadata:
//  name: postgres-migration
//spec:
//  template:
//    metadata:
//      annotations:
//        sidecar.istio.io/inject: "false"
//    spec:
//      containers:
//        - name: postgres-migration
//          image: bitnami/postgresql:16.0.0-debian-11-r15
//          command: ["/bin/bash", "-f", "/var/configs/migration/run.sh"]
//          env:
//            - name: PGPASSWORD
//              valueFrom:
//                secretKeyRef:
//                  name: secret-postgres-svc
//                  key: svc-password
//          volumeMounts:
//            - name: postgres-svc-migration
//              mountPath: /var/configs/migration
//      restartPolicy: Never
//      volumes:
//        - name: postgres-svc-migration
//          configMap:
//            name: postgres-svc-migration
//  backoffLimit: 4