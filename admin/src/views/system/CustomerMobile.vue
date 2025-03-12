<template>
  <n-card :bordered="false" class="proCard">
    <n-space vertical>
      <n-form ref="formRef" :model="formData" label-placement="left" label-width="auto"
        require-mark-placement="right-hanging">
        <n-form-item label="客服电话" path="mobile">
          <n-input v-model:value="formData.mobile" placeholder="请输入客服电话" />
        </n-form-item>
        <n-form-item label=" ">
          <n-space>
            <n-button type="info" :loading="formData.loading" @click="submit">保存</n-button>
          </n-space>
        </n-form-item>
      </n-form>
    </n-space>
  </n-card>
</template>

<script setup lang="ts">
import { onMounted, reactive } from 'vue';
import { setServicePhone } from '@/api/system/system';
import { useMessage } from 'naive-ui';

const message = useMessage()

const formData = reactive({
  loading: false,
  mobile: '',
})

const submit = () => {
  if (formData.mobile.length === 0) {
    message.error('请输入客服电话')
    return
  }
  setServicePhone({ mobile: formData.mobile }).then(() => message.success('已经成功保存'))
}

onMounted(() => {
  setServicePhone({}).then(res => {
    formData.mobile = res.mobile
  })
})
</script>

<style scoped></style>
