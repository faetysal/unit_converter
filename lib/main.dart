import 'package:flutter/material.dart';
import 'package:unit_converter/src/app.dart';
import 'package:rinf/rinf.dart';
import './messages/generated.dart';

void main() async {
  await initializeRust(assignRustSignal);
  runApp(UnitConverter());
}
