#include <jni.h>
#include <string>
#include <vector>
#include <cmath>

extern "C" {
JNIEXPORT jbyteArray JNICALL
Java_kaler_accelerometrhash_AccelerometerHash_processSensorData(
        JNIEnv* env,
        jobject /* this */,
        jbyteArray sensorData) {
    jsize len = env->GetArrayLength(sensorData);
    std::vector<jbyte> data(len);
    env->GetByteArrayRegion(sensorData, 0, len, data.data());

    // Modify the sensor data
    for (int i = 0; i < len; i += 3) {
        float x = static_cast<float>(data[i]);
        float y = static_cast<float>(data[i + 1]);
        float z = static_cast<float>(data[i + 2]);

        // Apply some transformation, e.g., scale the values
        x *= 2.0f;
        y *= 2.0f;
        z *= 2.0f;

        data[i] = static_cast<jbyte>(x);
        data[i + 1] = static_cast<jbyte>(y);
        data[i + 2] = static_cast<jbyte>(z);
    }

    jbyteArray result = env->NewByteArray(len);
    env->SetByteArrayRegion(result, 0, len, data.data());
    return result;
}
}