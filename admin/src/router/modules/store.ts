import { RouteRecordRaw } from 'vue-router';
import { Layout } from '@/router/constant';
import { OptionsSharp } from '@vicons/ionicons5';
import { renderIcon } from '@/utils/index';

/**
 * @param name 路由名称, 必须设置,且不能重名
 * @param meta 路由元信息（路由附带扩展信息）
 * @param redirect 重定向地址, 访问这个路由时,自定进行重定向
 * @param meta.disabled 禁用整个菜单
 * @param meta.title 菜单名称
 * @param meta.icon 菜单图标
 * @param meta.keepAlive 缓存该路由
 * @param meta.sort 排序越小越排前
 *
 * */
const routes: Array<RouteRecordRaw> = [
  {
    path: '/store',
    name: 'Store',
    redirect: '/store/list',
    component: Layout,
    meta: {
      title: '商家管理',
      icon: renderIcon(OptionsSharp),
      sort: 1,
      hidden: true
    },
    children: [
      {
        path: 'store',
        name: 'store_list',
        meta: {
          title: '商家管理',
        },
        component: () => import('@/views/store/List.vue'),
      },
      {
        path: 'account',
        name: 'store_account',
        meta: {
          title: '账号管理',
        },
        component: () => import('@/views/store/Account.vue'),
      },
      {
        path: 'tag',
        name: 'store_tag',
        meta: {
          title: '标签管理',
        },
        component: () => import('@/views/store/Tag.vue'),
      },
      {
        path: 'type',
        name: 'store_type',
        meta: {
          title: '商家类别管理',
        },
        component: () => import('@/views/store/Type.vue'),
      },
      {
        path: 'case',
        name: 'store_case',
        meta: {
          title: '商家案例',
        },
        component: () => import('@/views/store/Case.vue'),
      }
    ],
  },
];

export default routes;
