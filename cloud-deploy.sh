#! /bin/bash

set -e

remote_host="3.139.84.148"
remote_port=22
remote_user="admin"
local_path=$(pwd)/
remote_path="/home/${remote_user}/todo_rust/"
ssh_key="/home/vktornaj/aws/key_001.pem"
tag='0.0.2'


# Upadte source code
ssh -i ${ssh_key} ${remote_user}@${remote_host} sudo rm -rf todo_rust/
ssh -i ${ssh_key} ${remote_user}@${remote_host} aws ecr get-login-password --region us-east-2 | sudo docker login --username AWS --password-stdin 569233066229.dkr.ecr.us-east-2.amazonaws.com
rsync -avzr --exclude='.git/' --exclude='target/' --delete -e "ssh -p ${remote_port} -i ${ssh_key} -o StrictHostKeyChecking=no" ${local_path} ${remote_user}@${remote_host}:${remote_path}

# Build, tag and push docker image to aws ecr
ssh -tt -i ${ssh_key} ${remote_user}@${remote_host} << EOF 
sudo docker build -t todo-rust:${tag} ${remote_path}
sudo docker tag todo-rust:${tag} 569233066229.dkr.ecr.us-east-2.amazonaws.com/todo-rust:${tag}
sudo docker push 569233066229.dkr.ecr.us-east-2.amazonaws.com/todo-rust:${tag}
exit
EOF

## only for Mac OS:
# say "image created"