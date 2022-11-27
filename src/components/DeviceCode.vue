<script setup lang="ts">

import { invoke } from '@tauri-apps/api/tauri';
import { ref } from 'vue';

// 定义响应试数据
const device_number = ref("");
const error_message = ref("");
const result = ref("");


/**
 * input 验证：
 * 1. 最大长度 4
 * 3. 只能为可转为整数的字符串
 */

async function derive_number_check() {
    if (isNaN(Number(device_number.value))) {
        error_message.value = "不能输入非数字类型...";
    } else {
        error_message.value = ""; // 清除错误信息
        device_number.value = device_number.value.replace(/\s*/g,""); // 清除 device_number 中的所有空格
        if (device_number.value.length == 4) {
            result.value = await invoke("generate_activation_code", { deviceNumber: device_number.value });
        }

        if (device_number.value.length < 4) {
            result.value = "";
        }
    }
}

</script>

<template>
    <h2 class="text-center my-4"><i class="bi bi-activity mx-2"></i>设备激活码生成器</h2>
    <div class="input-group input-group-lg shadow-lg p-3 mb-5 rounded">
        <span class="input-group-text" id="inputGroup-sizing-lg">设备序列号 4 位</span>
        <input type="text" v-model="device_number" class="form-control text-center" aria-label="4位数字"
            aria-describedby="4位数字" placeholder="请输入4位有效数字" @input="derive_number_check" maxlength="4">
    </div>

    <div>
        <div v-if="error_message" class="alert alert-danger" role="alert">
            {{ error_message }}
        </div>

        <div class="alert alert-success" role="alert">
            <p class="fs-1 m-0">激活代码为：<strong v-if="result">{{ result }}</strong></p>
        </div>
    </div>

</template>

