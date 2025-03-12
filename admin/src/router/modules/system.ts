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
    path: '/system',
    name: 'System',
    redirect: '/system/banner',
    component: Layout,
    meta: {
      title: '系统设置',
      icon: renderIcon(OptionsSharp),
      sort: 2,
      hidden: true
    },
    children: [
      {
        path: 'banner',
        name: 'system_banner',
        meta: {
          title: 'Banner管理',
        },
        component: () => import('@/views/system/Banner.vue'),
      },
      {
        path: 'news',
        name: 'system_news',
        meta: {
          title: '消息管理',
        },
        component: () => import('@/views/system/News.vue'),
      },
      {
        path: 'notice',
        name: 'system_notice',
        meta: {
          title: '系统公告',
        },
        component: () => import('@/views/system/Notice.vue'),
      },
      {
        path: 'menu',
        name: 'system_menu',
        meta: {
          title: '首页菜单管理',
        },
        component: () => import('@/views/system/Menu.vue'),
      },
      {
        path: 'customer_mobile',
        name: 'system_customer_mobile',
        meta: {
          title: '客服电话',
        },
        component: () => import('@/views/system/CustomerMobile.vue'),
      },
    ],
  },
];

export default routes;
