<template>
  <div>
    <n-grid :x-gap="24">
      <!-- <n-grid-item span="6">
        <n-card :bordered="false" size="small" class="proCard">
          <n-thing
            class="thing-cell"
            v-for="item in typeTabList"
            :key="item.key"
            :class="{ 'thing-cell-on': type === item.key }"
            @click="switchType(item)"
          >
            <template #header>{{ item.name }}</template>
            <template #description>{{ item.desc }}</template>
          </n-thing>
        </n-card>
      </n-grid-item> -->
      <n-grid-item span="12" :offset="6">
        <n-card :bordered="false" size="small" title="修改密码" class="proCard">
          <!-- <BasicSetting v-if="type === 1" />
          <SafetySetting v-if="type === 2" /> -->
          <n-form ref="formRef" :model="form" label-placement="left" label-width="auto"
            require-mark-placement="right-hanging" :rules="rules">
            <n-form-item label="原密码" path="oldPassword">
              <n-input v-model:value="form.oldPassword" type="password" placeholder="请输入原密码" />
            </n-form-item>
            <n-form-item label="新密码" path="newPassword">
              <n-input v-model:value="form.newPassword" type="password" placeholder="请输入新密码" />
            </n-form-item>
            <n-form-item label="确认密码" path="newPasswordAgain">
              <n-input v-model:value="form.newPasswordAgain" type="password" placeholder="请输入确认密码" />
            </n-form-item>
            <n-form-item label=" ">
              <n-space>
                <n-button type="primary" @click="submit" :loading="form.loading">保存</n-button>
                <n-button @click="reset">重置</n-button>
              </n-space>
            </n-form-item>
          </n-form>

        </n-card>
      </n-grid-item>
    </n-grid>
  </div>
</template>
<script lang="ts" setup>
import { ref, reactive } from 'vue';
import BasicSetting from './BasicSetting.vue';
import SafetySetting from './SafetySetting.vue';
import { useMessage } from 'naive-ui';
import { changePwd } from '@/api/system/user';
import md5 from 'blueimp-md5';

const typeTabList = [
  {
    name: '基本设置',
    desc: '个人账户信息设置',
    key: 1,
  },
  {
    name: '安全设置',
    desc: '密码，邮箱等设置',
    key: 2,
  },
];

const type = ref(1);
const typeTitle = ref('基本设置');
const formRef: any = ref(null);
const message = useMessage()

function switchType(e) {
  type.value = e.key;
  typeTitle.value = e.name;
}

const form = reactive({
  loading: false,
  oldPassword: null,
  newPassword: null,
  newPasswordAgain: null,
})
const rules = {
  oldPassword: {
    required: true,
    trigger: ['blur', 'input'],
    message: '请输入原密码'
  },
  newPassword: {
    required: true,
    trigger: ['blur', 'input'],
    message: '请输入新密码'
  },
  newPasswordAgain: {
    required: true,
    trigger: ['blur', 'input'],
    message: '请再次输入新密码'
  },
}

const submit = (e: MouseEvent) => {
  e.preventDefault()
  formRef.value?.validate((errors) => {
    if (!errors) {
      if (form.newPasswordAgain != form.newPassword) {
        return message.error('新密码输入不一致')
      }
      if (form.newPasswordAgain == form.oldPassword) {
        return message.error('新密码不能和旧密码一样')
      }


      form.loading = true

      changePwd({ password: md5(form.newPassword) }).then(() => {
        message.success('密码已经更改')
        reset()
      }).finally(() => {
        form.loading = false
      })

    } else {
      message.error('验证失败')
    }
  })
}
const reset = () => {
  form.oldPassword = null
  form.newPassword = null
  form.newPasswordAgain = null
}
</script>
<style lang="less" scoped>
.thing-cell {
  margin: 0 -16px 10px;
  padding: 5px 16px;

  &:hover {
    background: #f3f3f3;
    cursor: pointer;
  }
}

.thing-cell-on {
  background: #f0faff;
  color: #2d8cf0;

  ::v-deep(.n-thing-main .n-thing-header .n-thing-header__title) {
    color: #2d8cf0;
  }

  &:hover {
    background: #f0faff;
  }
}
</style>
