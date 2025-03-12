import { http } from '@/utils/http/axios';

/**
 * 获取列表
 * @param params
 */
export function getStoreList(params?) {
  return http.request({
    url: '/store',
    method: 'GET',
    params,
  });
}

/**
 * 获取Account列表
 * @param params
 */
export function getStoreAccountType() {
  return http.request({
    url: '/store/store-account-type',
    method: 'GET',
  });
}

/**
 * 获取基础列表
 * @param params
 */
export function getStoreBase() {
  return http.request({
    url: '/store/base',
    method: 'GET',
  });
}

/**
 * 添加或者更新store
 * @param params
 */
export function postStore(data) {
  return http.request({
    url: '/store',
    method: 'POST',
    data
  });
}

/**
 * 删除store
 * @param params
 */
export function deleteStore(id) {
  return http.request({
    url: '/store/' + id,
    method: 'DELETE',
  });
}

/**
 * 添加或者更新store account type
 * @param params
 */
export function postStoreAccountType(data) {
  return http.request({
    url: '/store/store-account-type',
    method: 'POST',
    data
  });
}

/**
 * 删除store
 * @param params
 */
export function deleteStoreAccountType(id) {
  return http.request({
    url: '/store/store-account-type/' + id,
    method: 'DELETE',
  });
}

/**
 * 添加或者更新store type
 * @param params
 */
export function postStoreType(data) {
  return http.request({
    url: '/store/store-type',
    method: 'POST',
    data
  });
}

/**
 * 删除store
 * @param params
 */
export function deleteStoreType(id) {
  return http.request({
    url: '/store/store-type/' + id,
    method: 'DELETE',
  });
}


/**
 * 删除store
 * @param params
 */
export function getStoreDetail(id) {
  return http.request({
    url: '/store/' + id,
    method: 'GET',
  });
}

/**
 * 案例列表
 */
export function getCaseList(params?) {
  return http.request({
    url: '/case',
    method: 'GET',
    params,
  });
}

/**
 * 案例
 */
export function getCaseDetail(id) {
  return http.request({
    url: `/case/${id}`,
    method: 'GET'
  });
}
