kubectl kustomize --enable-helm . -o build.yaml

kubectl apply -f build.yaml --server-side
