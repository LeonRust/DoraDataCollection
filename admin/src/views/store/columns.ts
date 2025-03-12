import { h } from 'vue';
import { NAvatar } from 'naive-ui';

export const columns = [
  {
    title: 'id',
    key: 'id',
    width: 50,
  },
  {
    title: '名称',
    key: 'name',
    width: 180,
  },
  {
    title: '顺序',
    key: 'store_order',
    width: 80,
  },
  {
    title: '状态',
    key: 'is_normal',
    width: 100,
    render(row) {
      return row.is_normal == 1 ? '已审核'
        : (row.is_normal == 2 ? '已拒绝' : '待审核')
    },
  },
  {
    title: '图像',
    key: 'avatar',
    width: 100,
    render(row) {
      return h(NAvatar, {
        size: 48,
        src: row.image,
      });
    },
  },
  {
    title: '地址',
    key: 'address',
    width: 240,
  },
  {
    title: '创建时间',
    key: 'created_at',
    width: 160,
  },
];
