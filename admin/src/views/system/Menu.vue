<template>
  <n-card :bordered="false" class="proCard">
    <n-button type="primary" @click="add">
      <n-icon>
        <PlusOutlined />
      </n-icon>
      新建
    </n-button>

    <n-data-table :columns="columns" :data="data" :bordered="false" />

    <n-drawer v-model:show="form.activeDrawer" width="50%" :close-on-esc="false" :mask-closable="false">
      <n-drawer-content :title="form.title">
        <n-form ref="formRef" :model="form" label-placement="left" label-width="auto"
          require-mark-placement="right-hanging" :rules="rules">
          <n-form-item label="名称" path="name">
            <n-input v-model:value="form.name" placeholder="请输入名称" />
          </n-form-item>
          <n-form-item label="是否隐藏">
            <n-radio-group v-model:value="form.is_hide">
              <n-radio-button :value="0">显示</n-radio-button>
              <n-radio-button :value="1">隐藏</n-radio-button>
            </n-radio-group>
          </n-form-item>
          <n-form-item label="顺序">
            <n-input-number v-model:value="form.type_order" :min="1" :max="65535" />
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
import { h, reactive, ref, onMounted } from 'vue';
import { useMessage, useDialog, NTag, NButton } from 'naive-ui';
import { TableAction } from '@/components/Table';
import { PlusOutlined } from '@vicons/antd';
import { getMenuList, postMenu, deleteMenu } from '@/api/system/system';

const columns = [
  {
    title: 'id',
    key: 'id',
    width: 50,
  },
  {
    title: '名称',
    key: 'name',
    width: 150,
  },
  {
    title: '是否隐藏',
    key: 'is_hide',
    width: 80,
    render(row) {
      return h(
        NTag,
        {
          type: !row.is_hide ? 'success' : 'error',
        },
        {
          default: () => (!row.is_hide ? '显示' : '隐藏'),
        }
      );
    },
  },
  {
    title: '顺序',
    key: 'type_order',
    width: 80,
  },
  {
    title: '操作',
    key: 'actions',
    width: 100,
    render(record) {
      return h(TableAction as any, {
        style: 'button',
        actions: [
          {
            label: '删除',
            icon: 'ic:outline-delete-outline',
            onClick: handleDelete.bind(null, record),
          },
          {
            label: '编辑',
            onClick: handleEdit.bind(null, record),
          },
        ],
      });
    },
  }
]

const dialog = useDialog()
const message = useMessage()

const formRef: any = ref(null);
const data = ref([])
const form = reactive({
  activeDrawer: false,
  loading: false,
  title: '首页菜单管理',

  id: 0,
  name: null,
  is_hide: 0,
  type_order: 255,
})
const rules = {
  name: {
    required: true,
    trigger: ['blur', 'input'],
    message: '请输入名称'
  },
}

const init = async () => {
  getMenuList().then(res => {
    data.value = res
  })
}

const add = () => {
  resetData()
  form.activeDrawer = true
}
const resetData = () => {
  form.activeDrawer = false
  form.loading = false

  form.activeDrawer = false
  form.loading = false
  form.id = 0
  form.name = null
  form.is_hide = 0
  form.type_order = 255
}
const closeDrawer = () => {
  resetData()
}
const handleEdit = (record: Recordable) => {
  // console.log(record);
  resetData()

  form.id = record.id
  form.name = record.name
  form.is_hide = record.is_hide ? 1 : 0
  form.type_order = record.type_order

  form.activeDrawer = true
}

const handleDelete = (record: Recordable) => {
  // console.log('点击了删除', record);
  dialog.warning({
    title: '警告',
    content: '确定要删除当前数据，该操作无法撤销？',
    positiveText: '确定',
    negativeText: '取消',
    onPositiveClick: () => {
      deleteMenu(record.id).then(() => init())
    },
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
        name: form.name,
        is_hide: form.is_hide,
        type_order: form.type_order,
      }

      form.loading = true

      postMenu(formData).then(() => {
        resetData()
        init();
      }).finally(() => {
        form.loading = false
      })

    } else {
      message.error('验证失败')
    }
  })
}

onMounted(async () => {
  init()
})
</script>

<style scoped></style>
