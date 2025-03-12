<template>
  <div>
    <div class="n-layout-page-header">
      <n-card :bordered="false" title="工作台">
        <n-grid cols="2 s:1 m:1 l:2 xl:2 2xl:2" responsive="screen">
          <n-gi>
            <div class="flex items-center">
              <div>
                <n-avatar circle :size="64" :src="schoolboy" />
              </div>
              <div>
                <p class="px-4 text-xl">欢迎登录数据采集系统</p>
              </div>
            </div>
          </n-gi>
          <n-gi>
            <div class="flex justify-end w-full">
              <div class="flex flex-1 flex-col justify-center text-right">
                <span class="text-secondary">当前项目</span>
                <span class="text-2xl">{{ isRun ? '进行中' : '无' }} {{ taskName }}</span>
              </div>
              <!-- <div class="flex flex-1 flex-col justify-center text-right">
                <span class="text-secondary"></span>
                <span class="text-2xl"></span>
              </div> -->
              <!-- <div class="flex flex-1 flex-col justify-center text-right">
                <span class="text-secondary">消息</span>
                <span class="text-2xl">35</span>
              </div> -->
            </div>
          </n-gi>
        </n-grid>
        <n-card :bordered="false">

          <n-space>
            <n-button v-if="!hasCollectTask" type="success" @click="settingTask">
              设置采集任务
            </n-button>
            <template v-else>
              <n-button type="warning" @click="stopTask">
                结束采集任务
              </n-button>
            </template>
          </n-space>
        </n-card>
      </n-card>
    </div>

    <n-drawer v-model:show="form.activeDrawer" width="50%" :close-on-esc="false" :mask-closable="false">
      <n-drawer-content title="设置任务">
        <n-form ref="formRef" :model="form" label-placement="left" label-width="auto"
          require-mark-placement="right-hanging" :rules="rules">
          <n-form-item label="robot" path="robot">
            <n-select
            v-model:value="form.robot"
            placeholder="请选择robot"
            :options="robotOptions"
            />
          </n-form-item>
          <n-form-item label="scene" path="scene">
            <n-select
            v-model:value="form.scene"
            placeholder="请选择scene"
            :options="sceneOptions"
            />
          </n-form-item>
          <n-form-item label="task" path="task">
            <n-select
            v-model:value="form.task"
            placeholder="请选择task"
            :options="taskOptions"
            />
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

    <n-modal
      v-model:show="showModal"
      :mask-closable="false"
      :z-index="100000"
      preset="dialog"
      title="设置数据结果"
      content="请选择当前的采集组的成功状态"
      positive-text="成功"
      negative-text="失败"
      @positive-click="setSuccess"
      @negative-click="setFail"
    />
  </div>
</template>

<script lang="ts" setup>
import { getCurrentTask, getRobotList, getSceneList, getTaskList, postStartTask, postStopTask, setEpisodeResult } from '@/api/dashboard/workplace';
import schoolboy from '@/assets/images/schoolboy.png';
import { useMessage } from 'naive-ui';
import { onMounted, onUnmounted, reactive, ref } from 'vue';

// const userStore = useUserStore();
const message = useMessage()

const isRun = ref(false); // 采集数据是否在进行中
const taskName = ref('');
const showModal = ref(false); // 是否显示弹窗
const robotOptions = ref([]);
const sceneOptions = ref([]);
const taskOptions = ref([]);
const formRef: any = ref(null);

const hasCollectTask = ref(false);

const form = reactive({
  hasTask: false, // 是否有任务
  activeDrawer: false,
  loading: false,
  robot: null,
  scene: null,
  task: null
})
const rules = {
  robot: {
    required: true,
    message: '请选择robot',
    trigger: 'change',
  },
  scene: {
    required: true,
    message: '请选择scene',
    trigger: 'change',
  },
  task: {
    required: true,
    message: '请选择task',
    trigger: 'change',
  },
}


let timer;
const timekeeping = () => {
  timer = setInterval(() => {
    getCurrentTask().then((res) => {
      isRun.value = res.is_running;
      taskName.value = res.task_name;

      if (showModal.value != res.result) {
        showModal.value = res.result;
      }

      if (res.robot_id > 0 && res.scene_id > 0 && res.task_id > 0) {
        hasCollectTask.value = true;
      } else {
        hasCollectTask.value = false;
      }
    });
  }, 1000);
};

const settingTask = () => {
  // 获取robot list
  getRobotList().then((res) => {
    robotOptions.value = res.data.map(item  => {
      return {
        label: item.name,
        value: item.robot_id + '',
      }
    });
  });
  // 获取scene list
  getSceneList().then((res) => {
    sceneOptions.value = res.data.map(item  => {
      return {
        label: item.name,
        value: item.id + '',
      }
    });
  });
  // 获取robot list
  getTaskList().then((res) => {
    taskOptions.value = res.data.map(item  => {
      return {
        label: item.name,
        value: item.id + '',
      }
    });
  });
  resetData()
  form.activeDrawer = true
}
const resetData = () => {
  form.activeDrawer = false
  form.loading = false
  form.robot = null
  form.scene = null
  form.task = null
}
const closeDrawer = () => {
  resetData()
}
const submitForm = (e: MouseEvent) => {
  e.preventDefault();
  formRef.value?.validate((errors) => {
    if (!errors) {
      form.loading = true

      postStartTask({robot_id: ~~form.robot, scene_id: ~~form.robot, task_id: ~~form.task }).then(() => {
        resetData();
        hasCollectTask.value = true;
      })
    } else {
      message.error('验证失败')
    }
  })
}
const startTask = async () => {}

const stopTask = async () => {
  postStopTask().then(() => {
    hasCollectTask.value = false;
  })
}

const setSuccess = () => {
  setEpisodeResult({result: true})
}
const setFail = () => {
  setEpisodeResult({result: false})
}

onMounted(() => {
  timekeeping()
});

onUnmounted(() => {
  clearInterval(timer);
})

</script>

<style lang="less" scoped>
.project-card {
  margin-right: -6px;

  &-item {
    margin: -1px;
    width: 33.333333%;
  }
}
</style>
