import { http } from '@/utils/http/axios';

/**
 * 获取Banner列表
 * @param params
 */
export function getBannerList() {
  return http.request({
    url: '/banner',
    method: 'GET',
  });
}


/**
 * 添加或者更新store type
 * @param params
 */
export function postBanner(data) {
  return http.request({
    url: '/banner',
    method: 'POST',
    data
  });
}

/**
 * 删除store
 * @param params
 */
export function deleteBanner(id) {
  return http.request({
    url: '/banner/' + id,
    method: 'DELETE',
  });
}
