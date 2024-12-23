It is not used at the moment.

To install mongodb
```bash
helm install mongo oci://registry-1.docker.io/bitnamicharts/mongodb --version 16.4.0 -f k8s/mongo/values.yml
```

The latest output
```
Pulled: registry-1.docker.io/bitnamicharts/mongodb:16.4.0
Digest: sha256:5e376c79044821a3398e79829e497af6828079285a108313e667bf5a98d00aa0
NAME: mongo
LAST DEPLOYED: Mon Dec 23 21:39:55 2024
NAMESPACE: default
STATUS: deployed
REVISION: 1
TEST SUITE: None
NOTES:
CHART NAME: mongodb
CHART VERSION: 16.4.0
APP VERSION: 8.0.4

Did you know there are enterprise versions of the Bitnami catalog? For enhanced secure software supply chain features, unlimited pulls from Docker, LTS support, or application customization, see Bitnami Premium or Tanzu Application Catalog. See https://www.arrow.com/globalecs/na/vendors/bitnami for more information.

** Please be patient while the chart is being deployed **

MongoDB&reg; can be accessed on the following DNS name(s) and ports from within your cluster:

    mongo-mongodb.default.svc.cluster.local

To get the root password run:

    export MONGODB_ROOT_PASSWORD=$(kubectl get secret --namespace default mongo-mongodb -o jsonpath="{.data.mongodb-root-password}" | base64 -d)

To connect to your database, create a MongoDB&reg; client container:

    kubectl run --namespace default mongo-mongodb-client --rm --tty -i --restart='Never' --env="MONGODB_ROOT_PASSWORD=$MONGODB_ROOT_PASSWORD" --image docker.io/bitnami/mongodb:8.0.4-debian-12-r0 --command -- bash

Then, run the following command:
    mongosh admin --host "mongo-mongodb" --authenticationDatabase admin -u $MONGODB_ROOT_USER -p $MONGODB_ROOT_PASSWORD

To connect to your database from outside the cluster execute the following commands:

    kubectl port-forward --namespace default svc/mongo-mongodb 27017:27017 &
    mongosh --host 127.0.0.1 --authenticationDatabase admin -p $MONGODB_ROOT_PASSWORD

WARNING: There are "resources" sections in the chart not set. Using "resourcesPreset" is not recommended for production. For production installations, please set the following values according to your workload needs:
  - arbiter.resources
+info https://kubernetes.io/docs/concepts/configuration/manage-resources-containers/
```