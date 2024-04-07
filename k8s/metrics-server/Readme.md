To install metrics-server
```
helm repo add metrics-server https://kubernetes-sigs.github.io/metrics-server/
helm upgrade --install metrics-server metrics-server/metrics-server -f k8s/metrics-server/values.yml
```