package com.example.flutter_safety

import io.flutter.embedding.android.FlutterActivity
import io.flutter.embedding.engine.FlutterEngine
import io.flutter.plugin.common.MethodCall
import io.flutter.plugin.common.MethodChannel
import kaler.accelerometrhash.AccelerometerHash
import java.util.logging.Logger

class MainActivity : FlutterActivity() {

    private lateinit var accelerometerHash : AccelerometerHash

    override fun configureFlutterEngine(flutterEngine : FlutterEngine) {
        super.configureFlutterEngine(flutterEngine)
        MethodChannel(flutterEngine.dartExecutor.binaryMessenger , CHANNEL)
            .setMethodCallHandler { call : MethodCall , result : MethodChannel.Result ->
                if (call.method == "getAccelerometerData") {
                    if (accelerometerHash.isDataSizeReached()) {
                        // If 300 values have been recorded, process and send the data
                        val data = accelerometerHash.sensorDataToBytes()
                        Logger.getLogger(MainActivity::class.java.name).info("Data ready to send.")
                        result.success(data)
                        accelerometerHash.stopReading() // Stop reading to prevent additional data accumulation
                        accelerometerHash.startReading() // Restart reading for new data collection
                    } else {
                        // If 300 values have not been recorded, log and return an appropriate response
                        Logger.getLogger(MainActivity::class.java.name)
                            .info("Data not ready. Less than 300 values recorded.")
                        result.error(
                            "DATA_NOT_READY" ,
                            "Data not ready. Less than 300 values recorded." ,
                            null
                        )
                    }
                } else if (call.method == "getString") {
                    val data=accelerometerHash.getCpp()
                    result.success(data)
                } else {
                    result.notImplemented()
                }
            }
        accelerometerHash = AccelerometerHash(context)
        accelerometerHash.startReading() // Begin collecting data as soon as the app starts
    }

    companion object {
        private const val CHANNEL = "com.belstu/accelerometerAAR"
    }
}
