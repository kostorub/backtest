
To install the Ingress controller
```
helm repo add ingress-nginx https://kubernetes.github.io/ingress-nginx
helm repo update
helm upgrade --install nginx-ingress ingress-nginx --repo https://kubernetes.github.io/ingress-nginx -f k8s/ingress/values.yml
```

To install the nging ingress
```
kubectl apply -f k8s/ingress/ingress.yml
```

Check the tls secret
```
kubectl describe secret backtest-tls
```
