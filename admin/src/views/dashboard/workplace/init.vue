<template>
  <div>
    <n-card title="初始化摄像头和摇操臂">
      <n-form ref="formRef" :model="form" label-placement="left" :label-width="200" style="width: 600px;"
        require-mark-placement="right-hanging" :rules="rules" size="large">
        <n-form-item>
          <n-button type="warning" @click="refreshData">
            <template #icon>
              <n-icon>
                <Refresh />
              </n-icon>
            </template>
            刷新数据, 每次插入设备后需要点击该刷新按钮
          </n-button>
        </n-form-item>
        <n-form-item label="左摇操臂" path="u2d2_left">
          <n-select
          v-model:value="form.u2d2_left"
          placeholder="请选择左摇操臂"
          :options="u2d2Options"
          />
        </n-form-item>
        <n-form-item label="右摇操臂" path="u2d2_right">
          <n-select
          v-model:value="form.u2d2_right"
          placeholder="请选择右摇操臂"
          :options="u2d2Options"
          />
        </n-form-item>
        <n-form-item label="头部摄像头" path="orbbec_head">
          <n-select
          v-model:value="form.orbbec_head"
          placeholder="请选择头部摄像头"
          :options="orbbecOptions"
          />
        </n-form-item>
        <n-form-item label="左摄像头" path="orbbec_left">
          <n-select
          v-model:value="form.orbbec_left"
          placeholder="请选择左摄像头"
          :options="orbbecOptions"
          />
        </n-form-item>
        <n-form-item label="右摄像头" path="orbbec_right">
          <n-select
          v-model:value="form.orbbec_right"
          placeholder="请选择右摄像头"
          :options="orbbecOptions"
          />
        </n-form-item>
        <n-form-item>
          <n-button type="info" :loading="form.loading" @click="submitForm">保存</n-button>
        </n-form-item>
      </n-form>
    </n-card>
  </div>
</template>

<script setup lang="ts">
import { Refresh } from '@vicons/ionicons5'
import { getDeviceList, postRefreshDevice, postSaveDeviceList } from '@/api/dashboard/workplace';
import { reactive, ref } from 'vue';
import { useMessage } from 'naive-ui';

const message = useMessage()

const formRef: any = ref(null);
const u2d2Options = ref([]);
const orbbecOptions = ref([]);

const form = reactive({
  loading: false,
  u2d2_left: null,
  u2d2_right: null,
  orbbec_head: null,
  orbbec_left: null,
  orbbec_right: null,
})
const rules = {
  u2d2_left: {
    required: true,
    message: '请选择robot',
    trigger: 'change',
  },
  u2d2_right: {
    required: true,
    message: '请选择scene',
    trigger: 'change',
  },
  orbbec_head: {
    required: true,
    message: '请选择task',
    trigger: 'change',
  },
  orbbec_left: {
    required: true,
    message: '请选择task',
    trigger: 'change',
  },
  orbbec_right: {
    required: true,
    message: '请选择task',
    trigger: 'change',
  },
}

// 刷新数据
const refreshData = async () => {
  try {
    await postRefreshDevice({"usb_type": 1})
    const u2d2 = await getDeviceList()
    if (u2d2.data.length > 0) {
      u2d2Options.value = u2d2.data.map((item) => {
        return {
          label: item.usb_serial,
          value: item.usb_serial,
        }
      })
    }

    await postRefreshDevice({"usb_type": 2})
    const orbbec = await getDeviceList()
    if (orbbec.data.length > 0) {
      orbbecOptions.value = orbbec.data.map((item) => {
        return {
          label: item.usb_serial,
          value: item.usb_serial,
        }
      })
    }

  } catch (error) {
    console.log(error)
  }
}

const submitForm = (e: MouseEvent) => {
  e.preventDefault();
  formRef.value?.validate((errors) => {
    if (!errors) {
      if (form.u2d2_left === form.u2d2_right) {
        message.error('左右摇操臂设备不能相同')
        return
      }
      if ((form.orbbec_head === form.orbbec_left)
        || (form.orbbec_head === form.orbbec_right)
        || (form.orbbec_left === form.orbbec_right))
      {
        message.error('摄像头设备不能重复相同')
        return
      }


      form.loading = true

      postSaveDeviceList({
        u2d2_left: form.u2d2_left,
        u2d2_right: form.u2d2_right,
        orbbec_head: form.orbbec_head,
        orbbec_left: form.orbbec_left,
        orbbec_right: form.orbbec_right
      }).then(() => {
        form.loading = false;
      })
    } else {
      message.error('保存失败, 稍后重试')
    }
  })
}
</script>

<style lang="scss" scoped>

</style>
