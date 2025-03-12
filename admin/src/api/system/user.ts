import { http } from '@/utils/http/axios';

export interface BasicResponseModel<T = any> {
  code: number;
  message: string;
  result: T;
}

export interface BasicPageParams {
  pageNumber: number;
  pageSize: number;
  total: number;
}

/**
 * @description: 获取用户信息
 */
export function getUserInfo() {
  return http.request({
    url: '/admin_info',
    method: 'get',
  });
}

export interface LoginToken {
  token: string;
  username: string;
  timestamp: number;
}

/**
 * @description: 用户登录
 */
export function login(data) {
  return http.request<LoginToken>({
    url: '/login',
    method: 'POST',
    data,
  });
  // return http.request<BasicResponseModel>(
  //   {
  //     url: '/login',
  //     method: 'POST',
  //     params,
  //   },
  //   {
  //     isTransformResponse: false,
  //   }
  // );
}

/**
 * @description: 用户修改密码
 */
export function changePassword(params, uid) {
  return http.request(
    {
      url: `/user/u${uid}/changepw`,
      method: 'POST',
      params,
    },
    {
      isTransformResponse: false,
    }
  );
}

/**
 * @description: 用户登出
 */
export function logout(params) {
  return http.request({
    url: '/login/logout',
    method: 'POST',
    params,
  });
}

/**
 * @description: 修改密码
 */
export function changePwd(data) {
  return http.request({
    url: '/admin',
    method: 'POST',
    data,
  });
}


