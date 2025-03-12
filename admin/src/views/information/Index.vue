<template>
  <n-card :bordered="false" class="proCard">
    <BasicTable :columns="columns" :request="loadDataTable" :row-key="(row) => row.id" ref="actionRef"
      :actionColumn="actionColumn" :scroll-x="1090">
      <template #tableTitle>
        <n-button type="primary" @click="add">
          <template #icon>
            <n-icon>
              <PlusOutlined />
            </n-icon>
          </template>
          新建
        </n-button>
      </template>

    </BasicTable>

    <n-drawer v-model:show="form.activeDrawer" width="50%" :close-on-esc="false" :mask-closable="false" :z-index="1000">
      <n-drawer-content :title="form.drawerTitle">
        <n-form ref="formRef" :model="form" label-placement="left" label-width="auto"
          require-mark-placement="right-hanging" :rules="rules">
          <n-form-item label="名称" path="name">
            <n-input v-model:value="form.name" placeholder="请输入名称" />
          </n-form-item>
          <n-form-item label="缩略图" path="image">
            <n-space v-if="form.image && form.image_path" vertical>
              <n-image :src="form.image" width="100" />
              <n-popconfirm :show-icon="false" @positive-click="removeImage">
                <template #trigger>
                  <n-button>删除</n-button>
                </template>
                确认要删除当前的图像吗?
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
          <n-form-item label="顺序">
            <n-input-number v-model:value="form.order" :min="1" :max="65535" />
          </n-form-item>
          <n-form-item label="正文图片">
            <n-space>
              <n-space v-for="(item, index) of form.images" key="formImage" vertical>
                <n-image :src="item.image" width="100" />
                <n-popconfirm :show-icon="false" @positive-click="removeInformationImage(index)">
                  <template #trigger>
                    <n-button>删除</n-button>
                  </template>
                  确认要删除当前的图像吗?
                </n-popconfirm>
              </n-space>
              <n-upload v-if="form.images.length < 9" directory-dnd action="/api/admin/upload"
                accept="image/png, image/jpeg" name="image" @finish="uploadInformationImageFinish">
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
import { useMessage, useDialog, NAvatar, NButton } from 'naive-ui';
import { BasicTable, TableAction } from '@/components/Table';
import { PlusOutlined } from '@vicons/antd';
import { ArchiveOutline as ArchiveIcon } from '@vicons/ionicons5'
import { getInformationList, postInformation, deleteInformation, getInformationDetail } from '@/api/system/information';

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
    title: '图像',
    key: 'image',
    width: 100,
    render(row: { image: string; }) {
      if (row.image.length > 0) {
        return h(NAvatar, {
          size: 48,
          src: row.image,
        });
      } else {
        return '无';
      }
    },
  },
  {
    title: '顺序',
    key: 'order',
    width: 80,
  },
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
          label: '删除',
          icon: 'ic:outline-delete-outline',
          onClick: handleDelete.bind(null, record),
          // 根据业务控制是否显示 isShow 和 auth 是并且关系
          ifShow: () => {
            return true;
          },
          // 根据权限控制是否显示: 有权限，会显示，支持多个
          auth: ['root'],
        },
        {
          label: '编辑',
          onClick: handleEdit.bind(null, record),
          ifShow: () => {
            return true;
          },
          auth: ['root'],
        },
      ],
    });
  },
});

const dialog = useDialog()
const message = useMessage()
const actionRef = ref();
const editorRef: any = ref(null)
const formRef: any = ref(null);
const form = reactive({
  activeDrawer: false,
  loading: false,
  drawerTitle: '添加/编辑信息',

  id: 0,
  name: null,
  image: null,
  image_path: null,
  order: 255,
  images: [],
})
const rules = {
  name: {
    required: true,
    trigger: ['blur', 'input'],
    message: '请输入名称'
  },
  image: {
    required: true,
    trigger: 'change',
    message: '请上传图片'
  },
}

const params = ref({
  pageSize: 10,
});

const loadDataTable = async (res) => {
  return await getInformationList({ ...params.value, ...res });
};
const reloadTable = () => {
  actionRef.value.reload();
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
  form.image = null
  form.image_path = null
  form.order = 255
  form.images = []
}
const closeDrawer = () => {
  resetData()

  editorRef.value?.destroy()
  editorRef.value = null
}
const handleEdit = (record: Recordable) => {
  // console.log(record);
  resetData()

  form.id = record.id
  form.name = record.name
  form.image = record.image
  form.image_path = record.image_path
  form.order = record.order

  form.activeDrawer = true

  getInformationDetail(record.id).then(res => {
    form.images = res;
  })
}
const handleDelete = (record: Recordable) => {
  // console.log('点击了删除', record);
  dialog.warning({
    title: '警告',
    content: '确定要删除当前数据，该操作无法撤销？',
    positiveText: '确定',
    negativeText: '取消',
    onPositiveClick: () => {
      deleteInformation(record.id).then(() => reloadTable())
    },
  })
}

// 图片上传完成回调
const uploadImageFinish = ({ event }) => {
  // console.log(file, event)
  if (event) {
    if (event.target.status == 200) {
      const fileResp = JSON.parse(event.target.response)
      form.image = fileResp.url
      form.image_path = fileResp.file_path
    }
  }
}
const removeImage = () => {
  form.image = null
  form.image_path = null
}
// 商家正文图片上传回调
function uploadInformationImageFinish({ file, event }) {
  // console.log(file, event)
  if (event) {
    if (event.target.status == 200) {
      const fileResp = JSON.parse(event.target.response)
      form.images.push({
        image: fileResp.url,
        image_path: fileResp.file_path,
      })
    }
  }
}
function removeInformationImage(index: number) {
  form.images.splice(index, 1)
}
function handleBeforeLeave(tabId: string) {
  // console.log(tabId)
  storeForm.accountUploadImageIndex = ~~tabId
  return true
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
        image: form.image_path,
        order: form.order,
        images: form.images.map(item => item.image_path),
      }

      form.loading = true

      postInformation(formData).then(() => {
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
