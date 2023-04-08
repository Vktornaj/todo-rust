#! /bin/bash

set -e

remote_host="3.129.45.65"
remote_port=22
remote_user="admin"
dockerfile_local_path=$(pwd)/Dockerfile
local_path=$(pwd)/target/release/todo-rust
remote_path="/home/${remote_user}/todo_rust/"
ssh_key="/home/vktornaj/aws/key_001.pem"


ssh -i ${ssh_key} ${remote_user}@${remote_host} sudo rm -rf todo_rust/
cargo build --release 
rsync -avzr --delete -e "ssh -p ${remote_port} -i ${ssh_key} -o StrictHostKeyChecking=no" ${dockerfile_local_path} ${local_path} ${remote_user}@${remote_host}:${remote_path}

ssh -i ${ssh_key} ${remote_user}@${remote_host} sudo docker build --tag todo-rust ${remote_path}

## only for Mac OS:
# say "image created"