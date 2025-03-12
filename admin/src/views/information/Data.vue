<template>
  <n-card :bordered="false" class="proCard">
    <BasicTable :columns="columns" :request="loadDataTable" :row-key="(row) => row.id" ref="actionRef" :scroll-x="1090">
    </BasicTable>
  </n-card>
</template>

<script setup lang="ts">
import { reactive, ref } from 'vue';
import { BasicTable } from '@/components/Table';
import { getInformationDataList } from '@/api/system/information';

const columns = [
  {
    title: 'id',
    key: 'id',
    width: 50,
  },
  {
    title: '所属信息',
    key: 'information_name',
  },
  {
    title: '小区名称',
    key: 'community_name',
  },
  {
    title: '小区地址',
    key: 'address',
  },
  {
    title: '用户名',
    key: 'username',
  },
  {
    title: '手机号',
    key: 'mobile',
  },
]

const actionRef = ref();

const informationData = reactive({
  page: 1,
  pageSize: 10,
})

const loadDataTable = async (res) => {
  return await getInformationDataList({ ...informationData, ...res });
};
const reloadTable = () => {
  actionRef.value.reload();
}
</script>

<style scoped></style>
