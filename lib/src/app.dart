import 'package:flutter/material.dart';

import 'home.dart';

class UnitConverter extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      debugShowCheckedModeBanner: false,
      title: 'Unit Converter',
      theme: ThemeData(
        useMaterial3: false,
        colorScheme: ColorScheme.fromSeed(seedColor: Colors.deepPurple)
      ),
      home: Home()
    );
  }
}