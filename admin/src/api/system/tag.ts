import { http } from '@/utils/http/axios';

/**
 * 获取tag
 * @param data
 */
export function postTag(data) {
  return http.request({
    url: '/tag',
    method: 'POST',
    data
  });
}

/**
 * 删除tag
 * @param params
 */
export function deleteTag(id) {
  return http.request({
    url: '/tag/' + id,
    method: 'DELETE',
  });
}
