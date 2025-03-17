import { http } from '@/utils/http/axios';

// 获取当前服务是否开启
export function getDockerStatus() {
  return http.request({
    url: '/docker/status',
    method: 'get',
  });
}
// 开启服务
export function startDocker() {
  return http.request({
    url: '/docker/run',
    method: 'post',
  });
}
// 关闭服务
export function stopDocker() {
  return http.request({
    url: '/docker/stop',
    method: 'post',
  });
}

// 获取当前任务
export function getCurrentTask() {
  return http.request({
    url: '/task/current-task',
    method: 'get',
  });
}

export function setEpisodeResult(data) {
  return http.request({
    url: '/episode/set-result',
    method: 'post',
    data
  });
}

// robot
export function getRobotList() {
  return http.request({
    url: '/robot',
    method: 'get',
  });
}
export function createRobot(data) {
  return http.request({
    url: '/robot',
    method: 'post',
    data
  });
}

// scene
export function getSceneList() {
  return http.request({
    url: '/scene',
    method: 'get',
  });
}
export function createScene(data) {
  return http.request({
    url: '/scene',
    method: 'post',
    data
  });
}

// task
export function getTaskList() {
  return http.request({
    url: '/task',
    method: 'get',
  });
}
export function createTask(data) {
  return http.request({
    url: '/task',
    method: 'post',
    data
  });
}

// 开始采集任务
export function postStartTask(data) {
  return http.request({
    url: '/task/start',
    method: 'post',
    data
  });
}


// 停止任务
export function postStopTask() {
  return http.request({
    url: '/task/stop',
    method: 'post'
  });
}
