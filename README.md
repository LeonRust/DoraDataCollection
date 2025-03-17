# `Dora` 机器人采集系统

`Dora` 机器人采集系统是基于 `Rust` 语言开发的端到端的采集系统. 当前版本主要针对睿尔曼 `gen72` 双臂机器人`aDora`开发.

![`aDora`](./doc/adora.jpg)


## 快速开始

### Ubuntu

```bash
sudo apt install libx11-dev curl
```


### docker

`daemon` 服务增加 `xhost + local:docker`

```bash
# 允许非 root 用户执行 docker 命令
sudo usermod -aG docker $USER

# 创建配置文件（文件名需无后缀）
sudo tee /etc/sudoers.d/${USER}-nopasswd <<EOF
# 安全警告：谨慎配置免密权限
${USER} ALL=(ALL:ALL) NOPASSWD:ALL
EOF

# 设置严格权限
sudo chmod 440 /etc/sudoers.d/${USER}-nopasswd
```

docker-compose.yaml
```yaml
services:
    daemon:
        image: daemon
        container_name: daemon
        stdin_open: true
        tty: true
        restart: always
        environment:
            - DISPLAY=${DISPLAY:-:0}
            - DAEMON_IP=0.0.0.0
            - DAEMON_TCP_PORT=1234
            - DAEMON_HTTP_PORT=8080
            - CLOCK_INTERVAL=100
        volumes:
            - /tmp/.X11-unix:/tmp/.X11-unix:rw
            - /tmp:/tmp:rw
            - ./datasets:/datasets
            - ./db:/db
        ports:
            - "8080:8080"
        network_mode: "host"

    left-arm:
        image: arm
        container_name: left-arm
        restart: always
        environment:
            - DAEMON_IP=127.0.0.1
            - DAEMON_TCP_PORT=1234
            - ARM_IP=192.168.1.20
            - ARM_DIRECTION=left
        volumes:
            - ./datasets:/datasets
        network_mode: "host"
        depends_on:
            - daemon

    right-arm:
        image: arm
        container_name: right-arm
        restart: always
        environment:
            - DAEMON_IP=127.0.0.1
            - DAEMON_TCP_PORT=1234
            - ARM_IP=192.168.1.19
            - ARM_DIRECTION=right
        volumes:
            - ./datasets:/datasets
        network_mode: "host"
        depends_on:
            - daemon

    head-camera:
        image: camera
        container_name: head-camera
        restart: always
        environment:
            - DAEMON_IP=127.0.0.1
            - DAEMON_TCP_PORT=1234
            - CAMERA_ID=0
            - CAMERA_NUMBER=1
        devices:
            - /dev/video0:/dev/video0
        volumes:
            - ./datasets:/datasets
        network_mode: "host"
        depends_on:
            - daemon

    left-camera:
        image: camera
        container_name: left-camera
        restart: always
        environment:
            - DAEMON_IP=127.0.0.1
            - DAEMON_TCP_PORT=1234
            - CAMERA_ID=1
            - CAMERA_NUMBER=2
        devices:
            - /dev/video2:/dev/video2
        volumes:
            - ./datasets:/datasets
        network_mode: "host"
        depends_on:
            - daemon

    right-camera:
        image: camera
        container_name: right-camera
        restart: always
        environment:
            - DAEMON_IP=127.0.0.1
            - DAEMON_TCP_PORT=1234
            - CAMERA_ID=2
            - CAMERA_NUMBER=3
        devices:
            - /dev/video2:/dev/video2
        volumes:
            - ./datasets:/datasets
        network_mode: "host"
        depends_on:
            - daemon

```