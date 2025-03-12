<template>
  <div>
    <n-button type="success" @click="form.activeDrawer = true">添加</n-button>
    <n-table :bordered="false" :single-line="false">
      <thead>
        <tr>
          <th>id</th>
          <th>name</th>
          <th>info</th>
          <th>perception</th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="item in dataList">
          <td>{{ item.id }}</td>
          <td>{{ item.name }}</td>
          <td>{{ item.info }}</td>
          <td>{{ item.perception }}</td>
        </tr>
      </tbody>
    </n-table>

    <n-drawer v-model:show="form.activeDrawer" width="50%" :close-on-esc="false" :mask-closable="false">
      <n-drawer-content title="添加robot">
        <n-form ref="formRef" :model="form" label-placement="left" label-width="auto"
          require-mark-placement="right-hanging" :rules="rules">
          <n-form-item label="robot_id" path="robot_id">
            <n-input-number v-model:value="form.robot_id" :min="1" :max="65535" />
          </n-form-item>
          <n-form-item label="名称" path="name">
            <n-input v-model:value="form.name" placeholder="请输入名称" />
          </n-form-item>
          <n-form-item label="信息">
            <n-input v-model:value="form.info" placeholder="请输入机器人信息" />
          </n-form-item>
          <n-form-item label="感知">
            <n-input v-model:value="form.perception" placeholder="请输入机器人感知信息" />
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
  </div>
</template>

<script lang="ts" setup>
import { createRobot, getRobotList } from '@/api/dashboard/workplace';
import { useMessage } from 'naive-ui';
import { onMounted, reactive, ref } from 'vue';

const message = useMessage()

const dataList = ref([])
const formRef: any = ref(null);

const form = reactive({
  activeDrawer: false,
  loading: false,
  robot_id: 0,
  name: null,
  info: null,
  perception: null
})
const rules = {
  robot_id: {
    type: 'number',
    required: true,
    message: '请输入robot_id',
    trigger: ['blur', 'input'],
  },
  name: {
    required: true,
    message: '请输入name',
    trigger: ['blur', 'input'],
  },
}

const resetData = () => {
  form.activeDrawer = false
  form.loading = false
  form.robot_id = 0
  form.name = null
  form.info = null
  form.perception = null
}
const closeDrawer = () => {
  resetData()
}

const submitForm = (e: MouseEvent) => {
  e.preventDefault();
  formRef.value?.validate((errors) => {
    if (!errors) {
      form.loading = true

      createRobot({robot_id: ~~form.robot_id, name: form.name, info: form.info, perception: form.perception }).then(() => {
        init();
        resetData();
      })
      .catch(() => {
        form.loading = false
      })
    } else {
      message.error('验证失败')
    }
  })
}

const init = () => {
  getRobotList().then(res => {
    dataList.value = res.data
  })
}

onMounted(() => {
  init();
})

</script>

<style scoped>

</style>
