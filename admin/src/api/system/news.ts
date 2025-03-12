import { http } from '@/utils/http/axios';

/**
 * 获取News列表
 * @param params
 */
export function getNewsList(params) {
  return http.request({
    url: '/news',
    method: 'GET',
    params
  });
}

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
