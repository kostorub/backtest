To install the cert manager service
```
helm repo add jetstack https://charts.jetstack.io --force-update
helm repo update
helm install cert-manager jetstack/cert-manager --namespace cert-manager --create-namespace --version v1.14.4 -f k8s/cert-manager/values.yml
```
To upgrade
```
helm upgrade cert-manager jetstack/cert-manager --namespace cert-manager --version v1.14.4  -f k8s/cert-manager/values.yml
```

To install the ClusterIssuer resource
```
kubectl apply -f k8s/cert-manager/issuer-lets-encrypt-staging.yml
```
