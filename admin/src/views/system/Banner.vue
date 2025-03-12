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
      <n-drawer-content :title="form.drawerTitle">
        <n-form ref="formRef" :model="form" label-placement="left" label-width="auto"
          require-mark-placement="right-hanging" :rules="rules">
          <n-form-item label="名称" path="title">
            <n-input v-model:value="form.title" placeholder="请输入名称" />
          </n-form-item>
          <n-form-item label="图标icon" path="image">
            <n-space v-if="form.image && form.image_path" vertical>
              <n-image :src="form.image" width="100" />
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
          <n-form-item label="是否隐藏">
            <n-radio-group v-model:value="form.is_hide">
              <n-radio-button :value="0">显示</n-radio-button>
              <n-radio-button :value="1">隐藏</n-radio-button>
            </n-radio-group>
          </n-form-item>
          <n-form-item label="顺序">
            <n-input-number v-model:value="form.banner_order" :min="1" :max="65535" />
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
import { useMessage, useDialog, NTag, NAvatar, NButton } from 'naive-ui';
import { TableAction } from '@/components/Table';
import { PlusOutlined } from '@vicons/antd';
import { getBannerList, postBanner, deleteBanner } from '@/api/system/banner';
import { ArchiveOutline as ArchiveIcon } from '@vicons/ionicons5'

const columns = [
  {
    title: 'id',
    key: 'id',
    width: 50,
  },
  {
    title: '名称',
    key: 'title',
    width: 150,
  },
  {
    title: '图像',
    key: 'image',
    width: 100,
    render(row: { image: any; }) {
      return h(NAvatar, {
        size: 48,
        src: row.image,
      });
    },
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
    key: 'banner_order',
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
  drawerTitle: 'Banner管理',

  id: 0,
  title: null,
  desc: null,
  image: null,
  image_path: null,
  link: null,
  is_hide: 0,
  banner_order: 255,
})
const rules = {
  title: {
    required: true,
    trigger: ['blur', 'input'],
    message: '请输入名称'
  },
}

const init = async () => {
  getBannerList().then(res => {
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

  form.activeDrawer = false
  form.loading = false
  form.id = 0
  form.title = null
  form.desc = null
  form.image = null
  form.image_path = null
  form.link = null
  form.is_hide = 0
  form.banner_order = 255
}
const closeDrawer = () => {
  resetData()
}
const handleEdit = (record: Recordable) => {
  // console.log(record);
  resetData()

  form.id = record.id
  form.title = record.title
  form.desc = record.desc
  form.image = record.image
  form.image_path = record.image_path
  form.link = record.link
  form.is_hide = record.is_hide
  form.banner_order = record.banner_order

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
      deleteBanner(record.id).then(() => init())
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

// 提交表单
const submitForm = (e: MouseEvent) => {
  e.preventDefault()
  formRef.value?.validate((errors) => {
    if (!errors) {
      // 组织数据
      const formData = {
        id: form.id,
        title: form.title,
        desc: '',
        image: form.image_path,
        link: '',
        is_hide: form.is_hide,
        banner_order: form.banner_order,
      }

      form.loading = true

      postBanner(formData).then(() => {
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
