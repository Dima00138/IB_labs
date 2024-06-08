import 'package:flutter/services.dart';

class AccelerometerService {
  static const String serviceName = 'com.belstu/accelerometerAAR';
  static const platform = MethodChannel(serviceName);

  static Future<Uint8List> getAccelerometerData() async {
    final Uint8List data = await platform.invokeMethod('getAccelerometerData');
    return data;
  }
  static Future<String> getString() async{
    final String data = await platform.invokeMethod('getString');
    return data;
}
}
