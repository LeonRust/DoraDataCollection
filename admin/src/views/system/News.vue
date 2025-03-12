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

    <n-drawer v-model:show="form.activeDrawer" width="100%" :close-on-esc="false" :mask-closable="false" :z-index="1000">
      <n-drawer-content :title="form.drawerTitle">
        <n-form ref="formRef" :model="form" label-placement="left" label-width="auto"
          require-mark-placement="right-hanging" :rules="rules">
          <n-form-item label="名称" path="title">
            <n-input v-model:value="form.title" placeholder="请输入名称" />
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
          <n-form-item label="说明" path="desc">
            <n-input type="textarea" v-model:value="form.desc" placeholder="请输入说明" />
          </n-form-item>
          <n-form-item label="目标用户">
            <n-radio-group v-model:value="form.news_type">
              <n-radio-button :value="0">全平台</n-radio-button>
              <n-radio-button :value="1">普通用户</n-radio-button>
              <n-radio-button :value="2">商家用户</n-radio-button>
            </n-radio-group>
          </n-form-item>
          <n-form-item label="是否隐藏">
            <n-radio-group v-model:value="form.is_hide">
              <n-radio-button :value="0">显示</n-radio-button>
              <n-radio-button :value="1">隐藏</n-radio-button>
            </n-radio-group>
          </n-form-item>
          <n-form-item label="顺序">
            <n-input-number v-model:value="form.news_order" :min="1" :max="65535" />
          </n-form-item>
          <n-form-item label="内容" path="content">
            <!-- <n-input id="tinymceId" type="textarea" class="tinymce-textarea" /> -->
            <textarea :id="tinymceId">{{ form.content }}</textarea>
            <div class="editor-custom-btn-container">
              <!-- <a-button type="primary" @click="visible = true">
                <a-icon type="upload" />上传图片
              </a-button> -->
              <n-upload action="/api/admin/upload" accept="image/png, image/jpeg" name="image" :show-file-list="false"
                @finish="uploadEditorImageFinish">
                <n-button>上传文件</n-button>
              </n-upload>
              <!-- <a-modal :visible="visible" :destroyOnClose="true" @ok="visible = false" @cancel="visible = false">
                <upload label="上传图片" type="image" field="editor" :form="form" @upload-result="uploadImage($event)" />
              </a-modal> -->
            </div>
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
import { h, onMounted, reactive, ref } from 'vue';
import { useMessage, useDialog, NTag, NAvatar, NButton } from 'naive-ui';
import { BasicTable, TableAction } from '@/components/Table';
import { PlusOutlined } from '@vicons/antd';
import { ArchiveOutline as ArchiveIcon } from '@vicons/ionicons5'
import { getNewsList, postNews, deleteNews, getNewsDetail } from '@/api/system/news';
import { nextTick } from 'vue';

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
    key: 'news_order',
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
const tinymceId = ref<string>('tinymce-')
const visible = ref<boolean>(false)
const form = reactive({
  activeDrawer: false,
  loading: false,
  drawerTitle: 'News管理',

  id: 0,
  news_type: 0,
  title: null,
  desc: null,
  image: null,
  image_path: null,
  content: null,
  is_hide: 0,
  news_order: 255,
})
const rules = {
  title: {
    required: true,
    trigger: ['blur', 'input'],
    message: '请输入名称'
  },
  // image: {
  //   required: true,
  //   trigger: 'change',
  //   message: '请上传图片'
  // },
  // desc: {
  //   required: true,
  //   trigger: ['blur', 'input'],
  //   message: '请输入说明'
  // },
  content: {
    required: true,
    trigger: ['blur', 'input'],
    message: '请输入内容'
  },
}
const formParams = reactive({
  name: '',
  address: '',
  date: null,
});
const params = ref({
  pageSize: 10,
});

const loadDataTable = async (res) => {
  return await getNewsList({ ...formParams, ...params.value, ...res });
};
const reloadTable = () => {
  actionRef.value.reload();
}
const add = () => {
  resetData()
  form.activeDrawer = true
  initEditor()
}
const resetData = () => {
  form.activeDrawer = false
  form.loading = false

  form.activeDrawer = false
  form.loading = false
  form.id = 0
  form.news_type = 0
  form.title = null
  form.desc = null
  form.image = null
  form.image_path = null
  form.content = null
  form.is_hide = 0
  form.news_order = 255
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
  form.news_type = record.news_type
  form.title = record.title
  form.desc = record.desc
  form.image = record.image
  form.image_path = record.image_path
  form.content = null
  form.is_hide = record.is_hide
  form.news_order = record.news_order

  form.activeDrawer = true

  getNewsDetail(record.id).then(res => {
    form.content = res.content
    initEditor()
    // editorRef.value?.setContent(res.content)
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
      deleteNews(record.id).then(() => reloadTable())
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
const uploadEditorImageFinish = ({ event }) => {
  // console.log(file, event)
  if (event) {
    if (event.target.status == 200) {
      const fileResp = JSON.parse(event.target.response)
      // console.log(editorRef.value);
      editorRef.value?.insertContent(`<img src="${fileResp.url}">`)
    }
  }
}
// 初始化编辑器
const initEditor = () => {
  tinymceId.value = 'tinymce-' + (Math.random() * 10000).toFixed(0)
  nextTick(() => {
    tinymce.init({
      selector: `#${tinymceId.value}`,
      language: 'zh_CN',
      width: '100%',
      init_instance_callback: editor => {
        editorRef.value = editor
        editor.on('NodeChange Change KeyUp SetContent', () => {
          form.content = editor.getContent()
        })
      },
      // destroyTinymce() {
      //   const editor = tinymce.get('#mytextarea')
      //   if (editor) {
      //     editor.destroy()
      //   }
      // },
    });
  })
}
// const uploadImage = file => {
//   tinymce.get(`#${tinymceId.value}`).insertContent(`<img src="${file.path}">`)
// }
// 提交表单
const submitForm = (e: MouseEvent) => {
  e.preventDefault()
  formRef.value?.validate((errors) => {
    if (!errors) {
      // 组织数据
      const formData = {
        id: form.id,
        news_type: form.news_type,
        title: form.title,
        desc: form.desc,
        image: form.image_path,
        content: form.content,
        is_hide: form.is_hide,
        news_order: form.news_order,
      }

      form.loading = true

      postNews(formData).then(() => {
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

onMounted(() => {
})
</script>

<style scoped>
.tinymce-container {
  position: relative;
  /*line-height: normal;*/
}

.tinymce-textarea {
  visibility: hidden;
  z-index: -1;
}

.editor-custom-btn-container {
  position: absolute;
  right: 4px;
  top: 4px;
  z-index: 2005;
  padding: 10px;
  margin-top: 5px;
  margin-right: 5px;
  background-color: white;
}
</style>
