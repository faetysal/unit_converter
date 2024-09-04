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
  length([
    QuantityUnit(title: 'Milimetres', symbol: 'mm'),
    QuantityUnit(title: 'Centimetres', symbol: 'cm'),
    QuantityUnit(title: 'Metres', symbol: 'm')
  ]),
  area([]),
  temperature([]),
  volume([]),
  mass([]),
  data([]),
  speed([]),
  time([]),
  tip([]),
  none([]);

  const QuantityType(this.units);
  final List<QuantityUnit> units;
}

class QuantityUnit {
  final String title;
  final String symbol;

  const QuantityUnit({
    this.title = '',
    this.symbol = ''
  });
}