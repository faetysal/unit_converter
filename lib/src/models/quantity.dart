import 'package:flutter/material.dart';

class Quantity {
  final int? _id;
  String title;
  IconData? icon;
  QuantityType type;

  Quantity({
    int? id,
    this.title = '',
    this.icon,
    this.type = QuantityType.none
  }): _id = id;

  int? get id => _id;
}

enum QuantityType {
  length,
  area,
  temperature,
  volume,
  mass,
  data,
  speed,
  time,
  tip,
  none
}