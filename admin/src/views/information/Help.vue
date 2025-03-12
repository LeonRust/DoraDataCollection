<template>
  <n-card :bordered="false" class="proCard">
    <BasicTable :columns="columns" :request="loadDataTable" :row-key="(row) => row.id" ref="actionRef"
      :actionColumn="actionColumn" :scroll-x="1090" />

    <n-drawer v-model:show="form.activeDrawer" width="50%" :close-on-esc="false" :mask-closable="false" :z-index="1000">
      <n-drawer-content :title="form.drawerTitle">
        <n-form ref="formRef" :model="form" label-placement="left" label-width="auto"
          require-mark-placement="right-hanging">
          <n-form-item label="类型"> {{ form.help_type === 1 ? '投诉' : '售后' }} </n-form-item>
          <n-form-item label="用户称呼"> {{ form.username }} </n-form-item>
          <n-form-item label="手机号码"> {{ form.mobile }} </n-form-item>
          <n-form-item label="小区名称"> {{ form.community }} </n-form-item>
          <n-form-item label="详细地址"> {{ form.address }} </n-form-item>
          <n-form-item label="内容描述"> {{ form.content }} </n-form-item>
          <n-form-item label="期望结果"> {{ form.expectation_result }} </n-form-item>
          <n-form-item label="状态">
            <n-radio-group v-model:value="form.progress">
              <n-radio-button :value="1">待处理</n-radio-button>
              <n-radio-button :value="9">已处理</n-radio-button>
            </n-radio-group>
          </n-form-item>
          <n-form-item label="处理结果">
            <n-input v-model:value="form.progressResult" type="textarea" placeholder="请输入处理结果" />
          </n-form-item>
          <n-form-item label="正文图片">
            <n-space>
              <n-space v-for="(item, index) of helpImages" key="helpImage" vertical>
                <n-image :src="item.image" width="100" />
              </n-space>
            </n-space>
          </n-form-item>
        </n-form>

        <template #footer>
          <n-space>
            <n-button @click="closeDrawer">关闭</n-button>
            <n-button type="info" :loading="form.loading" @click="submitForm">保存</n-button>
          </n-space>
        </template>
      </n-drawer-content>
    </n-drawer>

  </n-card>
</template>

<script setup lang="ts">
import { h, reactive, ref } from 'vue';
import { useMessage, NButton } from 'naive-ui';
import { BasicTable, TableAction } from '@/components/Table';
import {  postHelp, getHelpList, getHelpDetail } from '@/api/system/information';

const columns = [
  {
    title: 'id',
    key: 'id',
    width: 50,
  },
  {
    title: '类型',
    key: 'help_type',
    width: 80,
    render(row) {
      return row.help_type == 1 ? '投诉' : '售后'
    },
  },
  {
    title: '称呼',
    key: 'username',
    width: 100,
  },
  {
    title: '手机',
    key: 'mobile',
    width: 120,
  },
  {
    title: '小区',
    key: 'address',
    width: 300,
  },
  {
    title: '问题',
    key: 'content',
    width: 150,
  },
  {
    title: '期望结果',
    key: 'expectation_result',
    width: 150,
  },
  {
    title: '当前状态',
    key: 'progress',
    width: 150,
    render(row) {
      return row.progress == 9 ? '已处理' : '待处理'
    },
  },
  {
    title: '处理结果',
    key: 'progress_result',
    width: 150,
  }
]
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
          label: '查看',
          onClick: handleEdit.bind(null, record),
          ifShow: () => {
            return true;
          },
        },
      ],
    });
  },
});
const helpImages = ref([])
const message = useMessage()
const actionRef = ref();
const formRef: any = ref(null);
const form = reactive({
  activeDrawer: false,
  loading: false,
  drawerTitle: '查看信息',

  id: 0,
  help_type: 1,
  username: '',
  mobile: '',
  community: '',
  address: '',
  content: '',
  expectation_result: '',
  progress: 1,
  progressResult: '',
})

const params = ref({
  pageSize: 10,
});

const loadDataTable = async (res) => {
  return await getHelpList({ ...params.value, ...res });
};
const reloadTable = () => {
  actionRef.value.reload();
}
const resetData = () => {
  form.activeDrawer = false
  form.loading = false
  form.id = 0
  form.help_type = 1,
  form.username = ''
  form.mobile = ''
  form.community = ''
  form.address = ''
  form.content = ''
  form.expectation_result = ''
  form.progress = 1
  form.progressResult = ''

  helpImages.value = []
}
const closeDrawer = () => {
  resetData()
}
const handleEdit = (record: Recordable) => {
  // console.log(record);
  resetData()

  form.id = record.id
  form.help_type = record.help_type,
  form.username = record.username
  form.mobile = record.mobile
  form.community = record.community
  form.address = record.address
  form.content = record.content
  form.expectation_result = record.expectation_result
  form.progress = record.progress
  form.progressResult = record.progress_result
  form.activeDrawer = true

  getHelpDetail(record.id).then(res => {
    helpImages.value = res.data;
  })
}

// 提交表单
const submitForm = (e: MouseEvent) => {
  e.preventDefault()
  formRef.value?.validate((errors) => {
    if (!errors) {
      // 组织数据
      const formData = {
        id: form.id,
        progress: form.progress,
        progress_result: form.progressResult,
      }

      form.loading = true

      postHelp(formData).then(() => {
        resetData()
        reloadTable()
      }).finally(() => {
        form.loading = false
      })

    } else {
      message.error('验证失败')
    }
  })
}
</script>

<style scoped></style>
