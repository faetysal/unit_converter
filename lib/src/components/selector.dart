import 'package:flutter/material.dart';
import 'package:gradient_borders/gradient_borders.dart';
import 'package:simple_gradient_text/simple_gradient_text.dart';

import '../models/quantity.dart';
import 'gradient_mask.dart';

class SelectorList extends StatelessWidget {
  const SelectorList({super.key,
    required this.value,
    required this.onChanged,
    this.children = const []
  });

  final QuantityType value;
  final void Function(QuantityType q) onChanged;
  final List children;

  @override
  Widget build(BuildContext context) {
    // return Row(children: children.map((c) => Expanded(child: c)).toList());
    return ListView.separated(
      scrollDirection: Axis.horizontal,
      itemCount: children.length,
      separatorBuilder: (context, index) => const SizedBox(width: 8),
      itemBuilder: (context, index) {
        final Quantity quantity = children[index];
        return Selector(
          value: quantity.type,
          label: quantity.title,
          icon: quantity.icon
        );
      },
    );
  }
}

class Selector extends StatelessWidget {
  const Selector({super.key, required this.value, this.label = '', this.icon});

  final String label;
  final QuantityType value;
  final IconData? icon;

  @override
  Widget build(BuildContext context) {
    final parent = context.findAncestorWidgetOfExactType<SelectorList>();

    return GestureDetector(
      onTap: () {
        parent.onChanged(value);
      },
      child: Container(
        padding: const EdgeInsets.symmetric(vertical: 4),
        width: 100,
        height: 40,
        decoration: BoxDecoration(
          color: Colors.white,
          border: parent!.value == value
            ? GradientBoxBorder(
                gradient: LinearGradient(colors: [Colors.pink, Colors.purple])
              )
            : null,
          borderRadius: BorderRadius.circular(8)
        ),
        child: Column(
          mainAxisAlignment: MainAxisAlignment.spaceBetween,
          children: [
            parent.value == value
              ? RadiantGradientMask(
                  child: Icon(icon, size: 30, color: Colors.white)
                )
              : Icon(icon, size: 30, color: Colors.grey[600]),
            const SizedBox(height: 4),
            Container(
              alignment: Alignment.center,
              width: double.infinity,
              child: parent.value == value
                ? GradientText(
                    label, 
                    colors: [Colors.pink, Colors.purple],
                    style: TextStyle(
                      fontSize: 12,
                      fontWeight: FontWeight.bold
                    ),
                  )
                : Text(label, style: TextStyle(
                  fontSize: 12,
                  fontWeight: FontWeight.bold,
                  color: Colors.grey[600]
                ))
            )
          ],
        )
      )
    );
  }
}