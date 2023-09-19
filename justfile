
set export
set shell := ["sh", "-c"]
GCP_PROJECT := `gcloud config get-value project`
LOCAL_K8S_TYPE := 'kind'

default:
    @just --list --unsorted

list:
  task -a
ctlptl-create:
    ctlptl create cluster kind --registry=ctlptl-registry
start: ctlptl-create
    echo Finished setting cluster

stop:
  #tilt down
  ctlptl delete cluster kind
#  just install-linkerd
#  just run-titl-cluster

install-linkerd:
  linkerd install --crds | kubectl apply -f -
  linkerd install | kubectl apply -f -
  linkerd check

install-viz:
  linkerd viz install | kubectl apply -f -
  linkerd viz dashboard &

build-all:
  pnpm nx affected --target=build.yaml --parallel --max-parallel=10 --prod

# Check for unused packages.
cargo-unused-deps:
  cargo +nightly udeps --all-targets

kaniko:
  docker run \
  -v $HOME/.config/gcloud:/root/.config/gcloud \
  -v $HOME/.config/kaniko/.docker:/root/.config/kaniko.docker \
  -v $PWD/:/workspace \
  gcr.io/kaniko-project/executor:debug \
  --dockerfile /workspace/Dockerfile \
  --destination "yurikrupnik/node-api-rest" \
  --build.yaml-arg "DIST_PATH=dist/apps/node/api-res" \
  --target node

task-list-all:
   task -ap

task-list-k8s:
  task -a -p -t k8s/Taskfile.yaml up
task-run-k8s:
  task -a -p -t k8s/Taskfile.yaml up
task-json:
  task -a -p -j -t k8s/Taskfile.yaml up
