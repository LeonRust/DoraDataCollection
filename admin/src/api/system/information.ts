import { http } from '@/utils/http/axios';

/**
 * 删除
 */
export function getNewsDetail(id) {
  return http.request({
    url: '/news/' + id,
    method: 'GET',
  });
}

/**
 * 添加或者更新
 */
export function postNews(data) {
  return http.request({
    url: '/news',
    method: 'POST',
    data
  });
}

/**
 * 删除
 */
export function deleteNews(id) {
  return http.request({
    url: '/news/' + id,
    method: 'DELETE',
  });
}

/**
 * 获取Information列表
 * @param params
 */
export function getInformationList(params) {
  return http.request({
    url: '/information',
    method: 'GET',
    params
  });
}

// 添加或者修改
export function postInformation(data) {
  return http.request({
    url: '/information',
    method: 'POST',
    data
  });
}

// 删除
export function deleteInformation(id) {
  return http.request({
    url: '/information?id=' + id,
    method: 'DELETE',
  });
}

/**
 * 收集数据列表
 * @param params
 * @returns
*/
export function getInformationDataList(params) {
  return http.request({
    url: '/information/datas',
    method: 'GET',
    params
  });
}

/**
 * 获取信息详情
 * @param params
*/
export function getInformationDetail(id) {
  return http.request({
    url: '/information/' + id,
    method: 'GET',
  });
}

/**
 * 投诉与售后列表
*/
export function getHelpList(params) {
  return http.request({
    url: '/help',
    method: 'GET',
    params
  });
}
/**
 * 获取信息详情
 * @param params
*/
export function getHelpDetail(id) {
  return http.request({
    url: '/help/' + id,
    method: 'GET',
  });
}

// 添加或者修改
export function postHelp(data) {
  return http.request({
    url: '/help',
    method: 'POST',
    data
  });
}
