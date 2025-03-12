<template>
  <n-card :bordered="false" class="proCard">
    <BasicTable :columns="CaseColumns" :request="loadDataTable" :row-key="(row) => row.id" ref="actionRef"
      :actionColumn="actionColumn" :scroll-x="1090" />

    <!-- 新建和编辑的抽屉 -->
    <n-drawer v-model:show="activeDrawer" width="60%" :close-on-esc="false"
      :mask-closable="false">
      <n-drawer-content>
        <n-form label-placement="left" label-width="auto">
          <n-form-item label="正文图片">
            <n-space>
              <n-space v-for="(item, index) of caseImages" key="caseImage" vertical>
                <n-image :src="item.image" width="100" />
              </n-space>
            </n-space>
          </n-form-item>
        </n-form>
        <template #footer>
          <n-space>
            <n-button @click="closeDrawer">关闭</n-button>
          </n-space>
        </template>
      </n-drawer-content>
    </n-drawer>
  </n-card>
</template>

<script setup lang="ts">
import { h, reactive, ref } from 'vue';
import { useMessage, useDialog } from 'naive-ui';
import { CaseColumns } from './CaseColumns';
import {  getCaseList, getCaseDetail } from '@/api/store';
import { BasicTable, TableAction } from '@/components/Table';
import { PlusOutlined } from '@vicons/antd';
import { ArchiveOutline as ArchiveIcon } from '@vicons/ionicons5'

const formRef: any = ref(null);
const message = useMessage();
const dialog = useDialog()
const actionRef = ref();
const formParams = reactive({
  name: '',
  address: '',
  date: null,
});

const params = ref({
  page: 1,
  pageSize: 10,
});

const activeDrawer = ref(false)
const caseImages = ref([])

const actionColumn = reactive({
  width: 140,
  title: '操作',
  key: 'action',
  fixed: 'right',
  render(record) {
    return h(TableAction as any, {
      style: 'button',
      actions: [
        {
          label: '查看图片',
          onClick: handleDetail.bind(null, record),
          ifShow: () => {
            return true;
          },
          auth: ['root'],
        },
      ],
    });
  },
});

const loadDataTable = async (res) => {
  return await getCaseList({ ...formParams, ...params.value, ...res });
};

// 关闭抽屉
function closeDrawer() {
  // 初始化数据
  caseImages.value = [];
  activeDrawer.value = false;
}

function reloadTable() {
  actionRef.value.reload();
}

function handleDetail(record: Recordable) {
  activeDrawer.value = true
  getCaseDetail(record.id).then(res => {
    caseImages.value = res.data;
  })
}
</script>

<style scoped></style>
