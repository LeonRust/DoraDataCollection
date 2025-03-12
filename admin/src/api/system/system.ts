import { http } from '@/utils/http/axios';

/**
 * 获取Notice
 */
export function getNotice() {
  return http.request({
    url: '/system/notice',
    method: 'GET',
  });
}


/**
 * 添加或者更新Notice
 */
export function postNotice(data) {
  return http.request({
    url: '/system/notice',
    method: 'POST',
    data
  });
}

// 设置客服电话
export function setServicePhone(data) {
  return http.request({
    url: '/system/customer',
    method: 'POST',
    data
  });
}


// 获取首页菜单列表
export function getMenuList() {
  return http.request({
    url: '/index_menu',
    method: 'GET',
  });
}
// 添加/更新菜单
export function postMenu(data) {
  return http.request({
    url: '/index_menu',
    method: 'POST',
    data
  });
}
// 删除菜单
export function deleteMenu(id) {
  return http.request({
    url: `/index_menu?id=${id}`,
    method: 'DELETE',
  });
}
