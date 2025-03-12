import { h } from 'vue';
import { NAvatar } from 'naive-ui';

export const CaseColumns = [
  {
    title: 'id',
    key: 'id',
    width: 50,
  },
  {
    title: '商家名称',
    key: 'store_name',
    width: 180,
  },
  {
    title: '标题',
    key: 'title',
    width: 80,
  },
  {
    title: '板材',
    key: 'board',
    width: 100,
  },
  {
    title: '投影面积',
    key: 'area',
    width: 100,
  },
  {
    title: '总价',
    key: 'price',
    width: 240,
  },
  {
    title: '缩略图',
    key: 'thumb',
    width: 100,
    render(row) {
      return h(NAvatar, {
        size: 48,
        src: row.thumb.image,
      });
    },
  }
];
