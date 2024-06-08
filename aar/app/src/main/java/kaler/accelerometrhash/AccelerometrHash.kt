package kaler.accelerometrhash

import android.content.Context
import android.hardware.Sensor
import android.hardware.SensorEvent
import android.hardware.SensorEventListener
import android.hardware.SensorManager
import java.nio.ByteBuffer

public class AccelerometerHash(context: Context) : SensorEventListener {
    private val sensorManager: SensorManager
    private val accelerometer: Sensor?
    private val sensorData = ArrayList<Byte>()
    private val DATA_SIZE = 300

    init {
        sensorManager = context.getSystemService(Context.SENSOR_SERVICE) as SensorManager
        accelerometer = sensorManager.getDefaultSensor(Sensor.TYPE_ACCELEROMETER)
        System.loadLibrary("app")
    }

    fun startReading() {
        sensorData.clear() // Clears previous data
        sensorManager.registerListener(this, accelerometer, SensorManager.SENSOR_DELAY_GAME)
    }

    fun stopReading() {
        sensorManager.unregisterListener(this)
    }

    override fun onSensorChanged(event: SensorEvent) {
        if (sensorData.size < DATA_SIZE) {
            sensorData.add((event.values[0].toRawBits() and 0xFF).toByte())
            sensorData.add((event.values[1].toRawBits() and 0xFF).toByte())
            sensorData.add((event.values[2].toRawBits() and 0xFF).toByte())
        } else {
            stopReading()
        }
    }

    override fun onAccuracyChanged(sensor: Sensor, accuracy: Int) {
        // Implementation not required for this use case
    }

    fun sensorDataToBytes(): ByteArray {
        val byteBuffer = ByteBuffer.allocate(sensorData.size)
        for (value in sensorData) {
            byteBuffer.put(value)
        }
        return byteBuffer.array()
    }

    fun isDataSizeReached(): Boolean {
        return sensorData.size >= DATA_SIZE
    }

    fun reset() {
        sensorData.clear()
    }

    external fun processSensorData(sensorData : ByteArray?) : ByteArray?

    fun getCpp(): String {
        val processedData = processSensorData(sensorDataToBytes())
        return processedData?.let { String(it) } ?: ""
    }
}