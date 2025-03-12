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
    path: '/information',
    name: 'Information',
    redirect: '/information/index',
    component: Layout,
    meta: {
      title: '信息收集管理',
      icon: renderIcon(OptionsSharp),
      sort: 3,
      hidden: true
    },
    children: [
      {
        path: 'index',
        name: 'information_index',
        meta: {
          title: '信息收集',
        },
        component: () => import('@/views/information/Index.vue'),
      },
      {
        path: 'data',
        name: 'information_data',
        meta: {
          title: '数据列表',
        },
        component: () => import('@/views/information/Data.vue'),
      },
      {
        path: 'help',
        name: 'information_help',
        meta: {
          title: '投诉与售后',
        },
        component: () => import('@/views/information/Help.vue'),
      }
    ],
  },
];

export default routes;
