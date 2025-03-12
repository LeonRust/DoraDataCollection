<template>
  <n-card :bordered="false" class="proCard">
    <n-space vertical>
      <n-alert type="info">
        <!-- 可以留空，留空代表关闭系统公告。 -->
        删除图片表示关闭系统公告
      </n-alert>
      <n-form ref="formRef" :model="notice" label-placement="left" label-width="auto"
        require-mark-placement="right-hanging">
        <n-form-item label="公告名称">
          <n-input v-model:value="notice.title" placeholder="请输入公告名称" />
        </n-form-item>
        <!-- <n-form-item label="公告内容">
          <n-input type="textarea" :rows="10" show-count :maxlength="255" v-model:value="notice.content"
            placeholder="请输入公告内容, 留空表示关闭公告" />
        </n-form-item> -->
        <n-form-item label="公告图片" path="icon">
          <n-space v-if="notice.content && notice.icon_path" vertical>
            <n-image :src="notice.content" width="100" />
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
        <n-form-item label=" ">
          <n-space>
            <n-button type="info" :loading="notice.loading" @click="submit">保存</n-button>
          </n-space>
        </n-form-item>
      </n-form>
    </n-space>
  </n-card>
</template>

<script setup lang="ts">
import { onMounted, reactive } from 'vue';
import { getNotice, postNotice } from '@/api/system/system';
import { useMessage } from 'naive-ui';
import { ArchiveOutline as ArchiveIcon } from '@vicons/ionicons5'

const message = useMessage()

const notice = reactive({
  loading: false,
  title: '',
  content: null,
  icon_path: null,
})

const submit = () => {
  console.log(notice);

  postNotice({ title: notice.title, content: notice.icon_path }).then(() => message.success('已经成功保存'))
}

const removeImage = () => {
  notice.content = null
  notice.icon_path = null
}

// 图片上传完成回调
const uploadImageFinish = ({ event }) => {
  // console.log(file, event)
  if (event) {
    if (event.target.status == 200) {
      const fileResp = JSON.parse(event.target.response)
      notice.content = fileResp.url
      notice.icon_path = fileResp.file_path
    }
  }
}

onMounted(() => {
  getNotice().then(res => {
    notice.title = res.system_notice_title
    notice.content = res.system_notice_content
    notice.icon_path = res.system_icon_path
  })
})
</script>

<style scoped></style>
