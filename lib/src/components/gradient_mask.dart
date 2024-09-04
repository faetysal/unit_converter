import 'package:flutter/material.dart';
import 'package:unit_converter/src/constants.dart';

class RadiantGradientMask extends StatelessWidget {
  RadiantGradientMask({required this.child});
  final Widget child;

  @override
  Widget build(BuildContext context) {
    return ShaderMask(
      shaderCallback: (bounds) => RadialGradient(
        center: Alignment.center,
        radius: 0.5,
        colors: appGradient,
        tileMode: TileMode.mirror,
      ).createShader(bounds),
      child: child,
    );
  }
}