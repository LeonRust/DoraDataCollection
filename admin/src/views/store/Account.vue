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
          <n-form-item label="账号名称" path="name">
            <n-input v-model:value="form.name" placeholder="请输入账号名称" />
          </n-form-item>
          <n-form-item label="图标icon" path="icon">
            <n-space v-if="form.icon && form.icon_path" vertical>
              <n-image :src="form.icon" width="100" />
              <n-popconfirm :show-icon="false" @positive-click="removeImage">
                <template #trigger>
                  <n-button>删除</n-button>
                </template>
                确认要删除当前的图标icon吗?
              </n-popconfirm>
            </n-space>
            <n-upload v-else directory-dnd action="/api/admin/upload" accept="image/png, image/jpeg" name="image"
              @finish="uploadImageFinish">
              <n-upload-dragger>
                <div style="margin-bottom: 12px">
                  <n-icon size="48" :depth="3">
                    <archive-icon />
                  </n-icon>
                </div>
                <n-text style="font-size: 16px">
                  点击或者拖动文件到该区域来上传
                </n-text>
              </n-upload-dragger>
            </n-upload>
          </n-form-item>
          <n-form-item label="是否显示账号">
            <n-radio-group v-model:value="form.show_account">
              <n-radio-button :value="0">不显示</n-radio-button>
              <n-radio-button :value="1">显示</n-radio-button>
            </n-radio-group>
          </n-form-item>
          <n-form-item label="是否显示图片">
            <n-radio-group v-model:value="form.show_image">
              <n-radio-button :value="0">不显示</n-radio-button>
              <n-radio-button :value="1">显示</n-radio-button>
            </n-radio-group>
          </n-form-item>
          <n-form-item label="顺序">
            <n-input-number v-model:value="form.account_order" :min="1" :max="65535" />
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
import { useMessage, NAvatar, NTag, NButton, useDialog } from 'naive-ui';
import { getStoreAccountType, postStoreAccountType, deleteStoreAccountType } from '@/api/store';
import { TableAction } from '@/components/Table';
import { PlusOutlined } from '@vicons/antd';
import { ArchiveOutline as ArchiveIcon } from '@vicons/ionicons5'

const dialog = useDialog()
const message = useMessage()

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
    title: '图标',
    key: 'icon',
    width: 50,
    render(row) {
      return h(NAvatar, {
        size: 48,
        src: row.icon,
      });
    },
  },
  {
    title: '是否显示账号',
    key: 'show_account',
    width: 80,
    render(row) {
      return h(
        NTag,
        {
          type: row.show_account == 1 ? 'success' : 'error',
        },
        {
          default: () => (row.show_account == 1 ? '显示' : '隐藏'),
        }
      );
    },
  },
  {
    title: '是否显示图片',
    key: 'show_image',
    width: 80,
    render(row) {
      return h(
        NTag,
        {
          type: row.show_image == 1 ? 'success' : 'error',
        },
        {
          default: () => (row.show_image == 1 ? '显示' : '隐藏'),
        }
      );
    },
  },
  {
    title: '顺序',
    key: 'account_order',
    width: 80,
  },
  {
    title: '操作',
    key: 'actions',
    width: 80,
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

const formRef: any = ref(null);
const data = ref([])
const form = reactive({
  activeDrawer: false,
  loading: false,
  title: '账号类别管理',

  id: 0,
  name: null,
  icon: null,
  icon_path: null,
  show_account: 1,
  show_image: 1,
  account_order: 255,
})
const rules = {
  name: {
    required: true,
    trigger: ['blur', 'input'],
    message: '请输入名称'
  },
  icon: {
    required: true,
    trigger: 'change',
    message: '请上传账号icon'
  },
}

const init = async () => {
  getStoreAccountType().then(res => {
    data.value = res.data
  })
}

const add = () => {
  resetData()
  form.activeDrawer = true
}
const resetData = () => {
  form.activeDrawer = false
  form.loading = false
  form.id = 0
  form.name = null
  form.icon = null
  form.icon_path = null
  form.show_account = 1
  form.show_image = 1
  form.account_order = 255
}
const closeDrawer = () => {
  resetData()
}
// 图片上传完成回调
const uploadImageFinish = ({ event }) => {
  // console.log(file, event)
  if (event) {
    if (event.target.status == 200) {
      const fileResp = JSON.parse(event.target.response)
      form.icon = fileResp.url
      form.icon_path = fileResp.file_path
    }
  }
}
const removeImage = () => {
  form.icon = null
  form.icon_path = null
}
const handleEdit = (record: Recordable) => {
  console.log(record);
  form.id = record.id
  form.name = record.name
  form.icon = record.icon
  form.icon_path = record.icon_path
  form.show_account = record.show_account
  form.show_image = record.show_image
  form.account_order = record.account_order

  form.activeDrawer = true
}

const handleDelete = (record: Recordable) => {
  // console.log('点击了删除', record);
  dialog.warning({
    title: '警告',
    content: '确定要删除当前账号，该操作无法撤销？',
    positiveText: '确定',
    negativeText: '取消',
    onPositiveClick: () => {
      deleteStoreAccountType(record.id).then(() => init())
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
        icon: form.icon_path,
        show_account: form.show_account,
        show_image: form.show_image,
        account_order: form.account_order,
      }

      form.loading = true

      postStoreAccountType(formData).then(() => {
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
