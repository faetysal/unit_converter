import 'package:flutter/material.dart';
import 'package:flutter/services.dart';

class Quantity {
  final int? _id;
  String title;
  IconData? icon;
  QuantityType type;
  List<TextInputFormatter>? inputFormatters;

  Quantity({
    int? id,
    this.title = '',
    this.icon,
    this.type = QuantityType.none,
    List<TextInputFormatter>? inputFormatters
  }):  _id = id,
      inputFormatters = inputFormatters ?? [
        FilteringTextInputFormatter.allow(
          RegExp(r'^\d+(\.\d*)?')
        ),
        LengthLimitingTextInputFormatter(15)
      ];

  int? get id => _id;
}

enum QuantityType {
  length([
    QuantityUnit(title: 'Millimetres', symbol: 'mm'),
    QuantityUnit(title: 'Centimetres', symbol: 'cm'),
    QuantityUnit(title: 'Metres', symbol: 'm'),
    QuantityUnit(title: 'Kilometres', symbol: 'km'),
    QuantityUnit(title: 'Inches', symbol: 'in'),
    QuantityUnit(title: 'Feet', symbol: 'ft'),
    QuantityUnit(title: 'Yards', symbol: 'yd'),
    QuantityUnit(title: 'Miles', symbol: 'mi'),
    QuantityUnit(title: 'Nautical miles', symbol: 'NM'),
    QuantityUnit(title: 'Mils', symbol: 'mil'),
  ]),
  area([
    QuantityUnit(title: 'Acres', symbol: 'ac'),
    QuantityUnit(title: 'Ares', symbol: 'a'),
    QuantityUnit(title: 'Hectares', symbol: 'ha'),
    QuantityUnit(title: 'Square centimetres', symbol: 'cm', sup: '2'),
    QuantityUnit(title: 'Square feet', symbol: 'ft', sup: '2'),
    QuantityUnit(title: 'Square inches', symbol: 'in', sup: '2'),
    QuantityUnit(title: 'Square metres', symbol: 'm', sup: '2')
  ]),
  temperature([
    QuantityUnit(title: 'Celcius', symbol: '°C'),
    QuantityUnit(title: 'Fahrenheit', symbol: '°F'),
    QuantityUnit(title: 'Kelvin', symbol: 'K'),
  ]),
  volume([
    QuantityUnit(title: 'UK gallons', symbol: 'gal', uSym: 'uk_gal'),
    QuantityUnit(title: 'US gallons', symbol: 'gal', uSym: 'us_gal'),
    QuantityUnit(title: 'Litres', symbol: 'l'),
    QuantityUnit(title: 'Millilitres', symbol: 'ml'),
    QuantityUnit(title: 'Cubic centimetres (cc)', symbol: 'cm', sup: '3'),
    QuantityUnit(title: 'Cubic metres', symbol: 'm', sup: '3'),
    QuantityUnit(title: 'Cubic inches', symbol: 'in', sup: '3'),
    QuantityUnit(title: 'Cubic feet', symbol: 'ft', sup: '3'),
  ]),
  mass([
    QuantityUnit(title: 'Tonnes', symbol: 't'),
    QuantityUnit(title: 'UK tons', symbol: 't', uSym: 'uk_t'),
    QuantityUnit(title: 'US tons', symbol: 't', uSym: 'us_t'),
    QuantityUnit(title: 'Pounds', symbol: 'lb'),
    QuantityUnit(title: 'Ounces', symbol: 'oz'),
    QuantityUnit(title: 'Kilogrammes', symbol: 'kg'),
    QuantityUnit(title: 'Grams', symbol: 'g'),
  ]),
  data([
    QuantityUnit(title: 'Bits', symbol: 'bit'),
    QuantityUnit(title: 'Bytes', symbol: 'B'),
    QuantityUnit(title: 'Kilobytes', symbol: 'KB'),
    QuantityUnit(title: 'Kibibytes', symbol: 'KiB'),
    QuantityUnit(title: 'Megabytes', symbol: 'MB'),
    QuantityUnit(title: 'Mebibytes', symbol: 'MiB'),
    QuantityUnit(title: 'Gigabytes', symbol: 'GB'),
    QuantityUnit(title: 'Gibibytes', symbol: 'GiB'),
    QuantityUnit(title: 'Terabytes', symbol: 'TB'),
    QuantityUnit(title: 'Tebibytes', symbol: 'TiB'),
  ]),
  speed([
    QuantityUnit(title: 'Metres per second', symbol: 'm/s'),
    QuantityUnit(title: 'Metres per hour', symbol: 'm/h'),
    QuantityUnit(title: 'Kilometres per second', symbol: 'km/s'),
    QuantityUnit(title: 'Kilometres per hour', symbol: 'km/h'),
    QuantityUnit(title: 'Inches per second', symbol: 'in/s'),
    QuantityUnit(title: 'Inches per hour', symbol: 'in/h'),
    QuantityUnit(title: 'Feet per second', symbol: 'ft/s'),
    QuantityUnit(title: 'Feet per hour', symbol: 'ft/h'),
    QuantityUnit(title: 'Miles per second', symbol: 'mi/s'),
    QuantityUnit(title: 'Miles per hour', symbol: 'mi/h'),
    QuantityUnit(title: 'Knots', symbol: 'kn')
  ]),
  time([
    QuantityUnit(title: 'Milliseconds', symbol: 'ms'),
    QuantityUnit(title: 'Seconds', symbol: 's'),
    QuantityUnit(title: 'Minutes', symbol: 'min'),
    QuantityUnit(title: 'Hours', symbol: 'h'),
    QuantityUnit(title: 'Days', symbol: 'd'),
    QuantityUnit(title: 'Weeks', symbol: 'wk')
  ]),
  // tip([]),
  none([]);

  const QuantityType(this.units);
  final List<QuantityUnit> units;

  @override
  String toString() {
    return name;
  }
}

class QuantityUnit {
  final String title;
  final String symbol;
  final String? uSym;
  final String? sup;

  const QuantityUnit({
    this.title = '',
    this.symbol = '',
    this.uSym,
    this.sup
  });

  factory QuantityUnit.withSuperScript(String title, String symbol) {
    final sup = symbol.split('^');
    return QuantityUnit(
      title: title,
      symbol: symbol
    );
  }

  String get symbolStr {
    String sym = uSym ?? symbol;
    if (sup != null) {
      sym += "^$sup";
    }

    return sym;
  }
}

/*class FunctionMap {
  Map<String, Function()> entries = {
    'celcius_to_fahrenheit': 
  };
}*/