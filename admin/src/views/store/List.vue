<template>
  <n-card :bordered="false" class="proCard">
    <BasicTable :columns="columns" :request="loadDataTable" :row-key="(row) => row.id" ref="actionRef"
      :actionColumn="actionColumn" :scroll-x="1090">
      <template #tableTitle>
        <n-button type="primary" @click="addTable" v-if="storeBaseSuccess">
          <template #icon>
            <n-icon>
              <PlusOutlined />
            </n-icon>
          </template>
          新建
        </n-button>
      </template>

    </BasicTable>

    <!-- 新建和编辑的抽屉 -->
    <n-drawer v-if="storeBaseSuccess" v-model:show="storeForm.activeDrawer" width="60%" :close-on-esc="false"
      :mask-closable="false">
      <n-drawer-content :title="storeForm.title">
        <n-form ref="formRef" :model="storeForm" label-placement="left" label-width="auto"
          require-mark-placement="right-hanging" :rules="storeRules">
          <n-form-item label="商家名称" path="name">
            <n-input v-model:value="storeForm.name" placeholder="请输入商家名称" />
          </n-form-item>
          <n-form-item label="商家状态" path="is_normal">
            <n-radio-group v-model:value="storeForm.is_normal">
              <n-radio-button :value="0">待审核</n-radio-button>
              <n-radio-button :value="1">已审核</n-radio-button>
              <n-radio-button :value="2">已拒绝</n-radio-button>
            </n-radio-group>
          </n-form-item>
          <n-form-item label="商家图片" path="image">
            <n-space v-if="storeForm.image && storeForm.image_path" vertical>
              <n-image :src="storeForm.image" width="100" />
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
          <n-form-item label="商家地址" path="address">
            <n-input v-model:value="storeForm.address" placeholder="请输入商家地址" />
          </n-form-item>
          <n-form-item label="姓名" path="username">
            <n-input v-model:value="storeForm.username" placeholder="请输入姓名" />
          </n-form-item>
          <n-form-item label="手机号" path="mobile">
            <n-input v-model:value="storeForm.mobile" placeholder="请输入手机号" />
          </n-form-item>
          <n-form-item label="地址经度" path="longitude">
            <n-input v-model:value="storeForm.longitude" placeholder="请输入地址经度。新添加的商家必须输入，修改商家留空保持不变" />
          </n-form-item>
          <n-form-item label="地址纬度" path="latitude">
            <n-input v-model:value="storeForm.latitude" placeholder="请输入地址纬度。新添加的商家必须输入，修改商家留空保持不变" />
          </n-form-item>
          <n-form-item label="首页菜单" path="type_id">
            <n-select v-model:value="storeForm.type_id" placeholder="请选择首页菜单" :options="storeBase.system_types" />
          </n-form-item>
          <n-form-item label="商家分类" path="store_type_id">
            <n-select v-model:value="storeForm.store_type_id" placeholder="请选择商家所属分类" :options="storeBase.store_types" />
          </n-form-item>
          <n-form-item label="商家标签" path="tags">
            <n-checkbox-group v-model:value="storeForm.tags">
              <n-space>
                <n-checkbox v-for="item of storeBase.tags" key="tag" :value="item.value">{{ item.label }}</n-checkbox>
              </n-space>
            </n-checkbox-group>
          </n-form-item>
          <n-form-item label="顺序" path="store_order">
            <n-input-number v-model:value="storeForm.store_order" :min="1" :max="65535" />
          </n-form-item>
          <n-form-item label="商家正文图片">
            <n-space>
              <n-space v-for="(item, index) of storeForm.storeImages" key="storeImage" vertical>
                <n-image :src="item.image" width="100" />
                <n-popconfirm :show-icon="false" @positive-click="removeStoreImage(index)">
                  <template #trigger>
                    <n-button>删除</n-button>
                  </template>
                  确认要删除当前的图像吗?
                </n-popconfirm>
              </n-space>
              <n-upload v-if="storeForm.storeImages.length < 9" directory-dnd action="/api/admin/upload"
                accept="image/png, image/jpeg" name="image" @finish="uploadStoreImageFinish">
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
          <n-form-item label="账号">
            <n-tabs type="line" animated @before-leave="handleBeforeLeave">
              <n-tab-pane v-for="(item, index) of storeForm.accounts" key="account" :name="index + ''" :tab="item.name">
                <n-form-item label="账号" v-if="item.show_account">
                  <n-input v-model:value="item.account_value" placeholder="请输入账号" />
                </n-form-item>
                <n-form-item label="图像" v-if="item.show_image">
                  <n-space v-if="item.account_image && item.account_image_path" vertical>
                    <n-image :src="item.account_image" width="100" />
                    <n-popconfirm :show-icon="false" @positive-click="removeAccountImage(index)">
                      <template #trigger>
                        <n-button>删除</n-button>
                      </template>
                      确认要删除当前的图像吗?
                    </n-popconfirm>
                  </n-space>
                  <n-upload v-else directory-dnd action="/api/admin/upload" accept="image/png, image/jpeg" name="image"
                    @finish="uploadAccountImageFinish">
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
              </n-tab-pane>
            </n-tabs>
          </n-form-item>
        </n-form>

        <template #footer>
          <n-space>
            <n-button @click="closeDrawer">关闭</n-button>
            <n-button type="info" :loading="storeForm.loading" @click="submitStoreForm">保存</n-button>
          </n-space>
        </template>
      </n-drawer-content>
    </n-drawer>
  </n-card>
</template>

<script setup lang="ts">
import { h, onBeforeMount, reactive, ref } from 'vue';
import { useMessage, useDialog } from 'naive-ui';
import { columns } from './columns';
import { getStoreBase, getStoreList, postStore, deleteStore, getStoreDetail } from '@/api/store';
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

const storeBaseSuccess = ref(false)
const storeBase = reactive({
  store_types: [], // 商家类型
  system_types: [], // 首页菜单
  tags: [], // 标签
  accounts: [], // 账号列表
})
// 商家表单
const storeForm = reactive({
  activeDrawer: false, // 抽屉开关
  title: '添加商家', // 抽屉标题
  loading: false, // 保存加载中

  id: 0,
  name: null, // 商家名称
  is_normal: 1, // 商家状态 0待审核 1已审核 2已拒绝
  image: null,
  image_path: null,
  longitude: null, // 经度
  latitude: null, // 纬度
  address: null, // 地址
  type_id: null, // 首页菜单
  store_type_id: null, // 商家类型
  store_order: 255, // 排序
  tags: [], // 标签列表
  accountUploadImageIndex: 0, // 账号上传图片的索引
  accounts: [], // 商家账号
  username: '',
  mobile: '',
  content: '',
  // 商家添加的正文图片列表
  storeImages: [],
})
const storeRules = {
  name: {
    required: true,
    trigger: ['blur', 'input'],
    message: '请输入商家名称'
  },
  is_normal: {
    type: 'number',
    required: true,
    trigger: 'change',
    message: '请选择商家状态'
  },
  image: {
    required: true,
    trigger: 'change',
    message: '请上传商家图片'
  },
  address: {
    required: true,
    trigger: ['blur', 'input'],
    message: '请输入商家地址'
  },
  type_id: {
    type: 'number',
    required: true,
    trigger: 'change',
    message: '请选择首页菜单'
  },
  store_type_id: {
    type: 'number',
    required: true,
    trigger: 'change',
    message: '请选择商家所属分类'
  },
  tags: {
    type: 'array',
    required: true,
    trigger: 'change',
    message: '请选择商家标签'
  },
}

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


const loadDataTable = async (res) => {
  return await getStoreList({ ...formParams, ...params.value, ...res });
};

function addTable() {
  resetData()
  storeForm.activeDrawer = true
}
// 关闭抽屉
function closeDrawer() {
  // 初始化数据
  resetData()
}
// 重制表单
function resetData() {
  storeForm.activeDrawer = false // 抽屉开关
  storeForm.title = '添加商家' // 抽屉标题
  storeForm.loading = false // 保存加载中

  storeForm.id = 0
  storeForm.name = null // 商家名称
  storeForm.is_normal = 1 // 商家状态 0待审核 1已审核 2已拒绝
  storeForm.image = null
  storeForm.image_path = null
  storeForm.longitude = null // 经度
  storeForm.latitude = null // 纬度
  storeForm.address = null // 地址
  storeForm.type_id = null // shouy
  storeForm.store_type_id = null // 商家所属分类
  storeForm.store_order = 255 // 排序
  storeForm.tags = [] // 标签列表
  storeForm.accountUploadImageIndex = 0 // 账号上传图片的索引
  storeForm.accounts = storeBase.accounts.map(item => {
    return { ...item }
  }) // 商家账号
  storeForm.username = ''
  storeForm.mobile = ''
  storeForm.content = ''
  storeForm.storeImages = []
}
function reloadTable() {
  actionRef.value.reload();
}

function handleEdit(record: Recordable) {
  storeForm.title = '编辑: ' + record.name
  storeForm.id = record.id
  storeForm.name = record.name // 商家名称
  storeForm.is_normal = record.is_normal // 商家状态 0待审核 1已审核 2已拒绝
  storeForm.image = record.image
  storeForm.image_path = record.image_path
  storeForm.longitude = null // 经度
  storeForm.latitude = null // 纬度
  storeForm.address = record.address // 地址
  storeForm.type_id = (record.type_id == null || record.type_id === 0) ? null : record.type_id // 首页菜单
  storeForm.store_type_id = (record.store_type_id == null || record.store_type_id === 0) ? null : record.store_type_id // 商家所属分类
  storeForm.store_order = record.store_order // 排序
  storeForm.tags = record.tag_ids.concat(record.tag_sys_ids) // 标签列表
  storeForm.accountUploadImageIndex = 0 // 账号上传图片的索引
  storeForm.accounts = storeBase.accounts.map(item => {
    record.accounts.forEach(v => {
      if (v.account_type_id == item.id) {
        item.account_value = v.account_value
        item.account_image = v.account_image
        item.account_image_path = v.account_image_path
        item.account_type_id = v.account_type_id
      }
    })
    return item
  }) // 商家账号
  storeForm.username = record.username ?? '' // 商家账号
  storeForm.mobile = record.mobile ?? '' // 商家账号
  storeForm.content = record.content ?? '' // 商家账号

  storeForm.activeDrawer = true

  getStoreDetail(record.id).then(res => {
    storeForm.storeImages = res;
  })
}

function handleDelete(record: Recordable) {
  // console.log('点击了删除', record);
  dialog.warning({
    title: '警告',
    content: '确定要删除当前商家，该操作无法撤销？',
    positiveText: '确定',
    negativeText: '取消',
    onPositiveClick: () => {
      deleteStore(record.id).then(() => reloadTable())
    },
  })
}

// 图片上传完成回调
function uploadImageFinish({ file, event }) {
  // console.log(file, event)
  if (event) {
    if (event.target.status == 200) {
      const fileResp = JSON.parse(event.target.response)
      storeForm.image = fileResp.url
      storeForm.image_path = fileResp.file_path
    }
  }
}
function removeImage() {
  storeForm.image = null
  storeForm.image_path = null
}
// 商家正文图片上传回调
function uploadStoreImageFinish({ file, event }) {
  // console.log(file, event)
  if (event) {
    if (event.target.status == 200) {
      const fileResp = JSON.parse(event.target.response)
      storeForm.storeImages.push({
        image: fileResp.url,
        image_path: fileResp.file_path,
      })
    }
  }
}
function removeStoreImage(index: number) {
  storeForm.storeImages.splice(index, 1)
}
function handleBeforeLeave(tabId: string) {
  // console.log(tabId)
  storeForm.accountUploadImageIndex = ~~tabId
  return true
}
// 图片上传完成回调
function uploadAccountImageFinish({ file, event }) {
  if (event) {
    if (event.target.status == 200) {
      const fileResp = JSON.parse(event.target.response)
      storeForm.accounts[storeForm.accountUploadImageIndex].account_image = fileResp.url
      storeForm.accounts[storeForm.accountUploadImageIndex].account_image_path = fileResp.file_path
    }
  }
}
function removeAccountImage(index) {
  storeForm.accounts[index].account_image = null
  storeForm.accounts[index].account_image_path = null
}

function submitStoreForm(e: MouseEvent) {
  e.preventDefault()
  formRef.value?.validate((errors) => {
    if (!errors) {
      // 继续验证经纬度
      const longitude = parseFloat(storeForm.longitude ?? '')
      const latitude = parseFloat(storeForm.latitude ?? '')
      if (storeForm.id == 0) {
        if (storeForm.longitude == null || storeForm.latitude == null) {
          return message.error('请输入经纬度')
        }
        if (isNaN(longitude) || isNaN(latitude)) {
          return message.error('请输入正确的经纬度')
        }

      } else {
        if (!(storeForm.longitude == null && storeForm.latitude == null)) {
          if (isNaN(longitude) || isNaN(latitude)) {
            return message.error('请输入正确的经纬度')
          }

          return message.error('经纬度必须同时输入')
        }
      }

      // 组织数据
      const formData = {
        id: storeForm.id,
        is_normal: storeForm.is_normal,
        name: storeForm.name,
        image: storeForm.image_path,
        address: storeForm.address,
        longitude: isNaN(longitude) ? null : longitude,
        latitude: isNaN(latitude) ? null : latitude,
        type_id: storeForm.type_id,
        store_type_id: storeForm.store_type_id,
        store_order: storeForm.store_order,
        tags: storeForm.tags,
        account: storeForm.accounts.map(item => {
          return {
            account_type_id: item.account_type_id,
            account_value: item.account_value ?? '',
            account_image: item.account_image_path ?? ''
          }
        }),
        username: storeForm.username,
        mobile: storeForm.mobile,
        content: storeForm.content,
        images: storeForm.storeImages.map(item => item.image_path),
      }

      storeForm.loading = true

      postStore(formData).then(() => {
        resetData()
        reloadTable();
      }).finally(() => {
        storeForm.loading = false
      })

    } else {
      message.error('验证失败')
    }
  })
}

onBeforeMount(async () => {
  storeBaseSuccess.value = false
  const res = await getStoreBase()
  storeBase.store_types = res.store_types.filter(item => !item.is_hide).map(item => {
    return {
      label: item.is_only_sys ? item.name + ' (平台)' : item.name,
      value: item.id,
      ...item
    }
  })

  storeBase.system_types = res.system_type.map(item => {
    return {
      label: item.name,
      value: item.id,
      ...item
    }
  })

  storeBase.tags = res.tags.filter(item => !item.is_hide).map(item => {
    return {
      label: item.is_sys ? item.name + ' (平台)' : item.name,
      value: item.id,
      ...item
    }
  })
  storeBase.accounts = res.accounts.map(item => {
    return {
      account_type_id: item.id,
      account_value: null,
      account_image: null, // 图像url
      account_image_path: null, // 仅图像路径
      ...item
    }
  })
  storeForm.accounts = res.accounts.map(item => {
    return {
      account_type_id: item.id,
      account_value: null,
      account_image: null, // 图像url
      account_image_path: null, // 仅图像路径
      ...item
    }
  })
  storeBaseSuccess.value = true
})
</script>

<style scoped></style>
