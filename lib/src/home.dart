import 'dart:async';

import 'package:dropdown_button2/dropdown_button2.dart';
import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:flutter_styled_toast/flutter_styled_toast.dart';
import 'package:get/get.dart';
import 'package:hugeicons/hugeicons.dart';
import 'package:responsive_grid_list/responsive_grid_list.dart';
import 'package:rinf/rinf.dart';
import 'package:unit_converter/messages/convert.pb.dart';

import 'components/selector.dart';
import 'models/quantity.dart';

class Home extends StatelessWidget {
  const Home({super.key});

  @override
  Widget build(BuildContext context) {
    HomeController controller = Get.put(HomeController(context));

    List<Quantity> quantities = [
      Quantity(
        id: 1,
        title: 'Length',
        icon: HugeIcons.strokeRoundedRuler,
        type: QuantityType.length,
      ),
      Quantity(id: 2, title: 'Area', icon: HugeIcons.strokeRoundedBoundingBox, type: QuantityType.area),
      Quantity(
        id: 3,
        title: 'Temperature',
        icon: HugeIcons.strokeRoundedTemperature,
        type: QuantityType.temperature,
        inputFormatters: [
          FilteringTextInputFormatter.allow(
            RegExp(r'^-?(\d+(\.(\d+)?)?)?')
          )
        ]
      ),
      Quantity(id: 4, title: 'Volume', icon: HugeIcons.strokeRoundedDroplet, type: QuantityType.volume),
      Quantity(id: 5, title: 'Mass', icon: HugeIcons.strokeRoundedWeightScale01, type: QuantityType.mass),
      Quantity(id: 6, title: 'Data', icon: HugeIcons.strokeRoundedDatabase, type: QuantityType.data),
      Quantity(id: 7, title: 'Speed', icon: HugeIcons.strokeRoundedDashboardSpeed02, type: QuantityType.speed),
      Quantity(id: 8, title: 'Time', icon: HugeIcons.strokeRoundedClock02, type: QuantityType.time),
      // Quantity(id: 9, title: 'Tip', icon: HugeIcons.strokeRoundedTips, type: QuantityType.tip)
    ];

    return Scaffold(
      appBar: AppBar(
        backgroundColor: Colors.grey[100],
        elevation: 0,
        shadowColor: Colors.transparent,
        title: const Text('Converter', style: TextStyle(color: Colors.black),),
        centerTitle: false,
      ),
      body: Obx(() => Column(
        mainAxisAlignment: MainAxisAlignment.spaceBetween,
        children: [
          Container(
            height: 60,
            padding: const EdgeInsets.symmetric(horizontal: 16),
            margin: const EdgeInsets.only(top: 5, bottom: 10),
            // color: Colors.orange,
            child: SelectorList(
              value: controller.quantityType.value,
              onChanged: (v) {
                controller.quantityType.value = v;
                controller.init();
              },
              children: quantities,
            )
          ),

          Expanded(
            child: Container(
              padding: const EdgeInsets.symmetric(horizontal: 16),
              child: LayoutBuilder(
                builder: (context, constraints) {
                  return Column(
                    children: [
                      Container(
                        height: constraints.maxHeight / 2,
                        padding: const EdgeInsets.symmetric(vertical: 8),
                        decoration: BoxDecoration(
                          //color: Colors.red,
                          border: Border(
                            top: BorderSide(color: Colors.grey[300]!),
                            bottom: BorderSide(color: Colors.grey[300]!)
                          )
                        ),
                        child: Obx(() => Column(
                          mainAxisAlignment: MainAxisAlignment.spaceBetween,
                          children: [
                            Row(
                              children: [
                                DropdownButtonHideUnderline(
                                  child: DropdownButton2<QuantityUnit>(
                                    value: controller.fromUnit.value,
                                    customButton: Row(
                                      children: [
                                        Text(controller.fromUnit.value.title),
                                        Icon(Icons.arrow_drop_down)
                                      ],
                                    ),
                                    items: controller.quantityType.value.units.map((u) {
                                      return DropdownMenuItem(
                                        value: u,
                                        child: RichText(
                                          text: TextSpan(
                                            style: TextStyle(
                                              color: Colors.black
                                            ),
                                            children: [
                                              TextSpan(text: u.title),
                                              TextSpan(text: ' (${u.symbol}'),
                                              TextSpan(text: u.sup ?? '', style: const TextStyle(
                                                fontFeatures: [FontFeature.superscripts()]
                                              )),
                                              TextSpan(text: ')'),
                                            ]
                                          )
                                        )
                                      );
                                    }).toList(),
                                    onChanged: (v) {
                                      controller.fromUnit.value = v!;
                                      controller.activeField.$2.requestFocus();
                                      final unitVal = controller.activeField.$1.text;
                                      controller.handleConversion(unitVal);
                                    },
                                    isDense: true,
                                    dropdownStyleData: DropdownStyleData(
                                      width: MediaQuery.of(context).size.width * .7,
                                      decoration: BoxDecoration(
                                        borderRadius: BorderRadius.circular(8)
                                      )
                                    ),
                                  )
                                ),
                              ]
                            ),
                            Expanded(
                              child: Row(
                                crossAxisAlignment: CrossAxisAlignment.end,
                                children: [
                                  Expanded(
                                    child: Container(
                                      // height: 60,
                                      child: _buildInputField(
                                        inputController: controller.fromCtrl, 
                                        focusNode: controller.fromFocus,
                                        inputFormatters: quantities.firstWhere((q) => q.type == controller.quantityType.value).inputFormatters
                                      )
                                    ),
                                  ),
                                  const SizedBox(width: 8),
                                  RichText(
                                    text: TextSpan(
                                      style: TextStyle(
                                        fontSize: 16,
                                        fontWeight: FontWeight.w600,
                                        color: Colors.black
                                      ),
                                      children: [
                                        TextSpan(text: controller.fromUnit.value.symbol), 
                                        TextSpan(text: controller.fromUnit.value.sup ?? '', style: TextStyle(
                                          fontFeatures: [FontFeature.superscripts()]
                                        ))
                                      ]
                                    )
                                  )
                                ],
                              ),
                            )
                          ],
                        ),
                      )),
                      Container(
                        height: constraints.maxHeight / 2,
                        padding: const EdgeInsets.symmetric(vertical: 8),
                        decoration: BoxDecoration(
                          // color: Colors.red,
                          border: Border(
                            bottom: BorderSide(color: Colors.grey[300]!)
                          )
                        ),
                        child: Obx(() => Column(
                          mainAxisAlignment: MainAxisAlignment.spaceBetween,
                          children: [
                            Row(
                              children: [
                                DropdownButtonHideUnderline(
                                  child: DropdownButton2<QuantityUnit>(
                                    value: controller.toUnit.value,
                                    customButton: Row(
                                      children: [
                                        Text(controller.toUnit.value.title),
                                        Icon(Icons.arrow_drop_down)
                                      ],
                                    ),
                                    items: controller.quantityType.value.units.map((u) {
                                      return DropdownMenuItem(
                                        value: u,
                                        child: RichText(
                                          text: TextSpan(
                                            style: TextStyle(color: Colors.black),
                                            children: [
                                              TextSpan(text: u.title),
                                              TextSpan(text: ' (${u.symbol}'),
                                              TextSpan(text: u.sup ?? '', style: TextStyle(
                                                fontFeatures: [FontFeature.superscripts()]
                                              )),
                                              TextSpan(text: ')')
                                            ]
                                          )
                                        )
                                      );
                                    }).toList(),
                                    onChanged: (v) {
                                      controller.toUnit.value = v!;
                                      controller.activeField.$2.requestFocus();
                                      final unitVal = controller.activeField.$1.text;
                                      controller.handleConversion(unitVal);
                                    },
                                    isDense: true,
                                    dropdownStyleData: DropdownStyleData(
                                      width: MediaQuery.of(context).size.width * .7,
                                      decoration: BoxDecoration(
                                        borderRadius: BorderRadius.circular(8)
                                      )
                                    ),
                                  )
                                ),
                              ]
                            ),
                            Expanded(
                              child: Row(
                                crossAxisAlignment: CrossAxisAlignment.end,
                                children: [
                                  Expanded(
                                    child: Container(
                                      child: _buildInputField(
                                        inputController: controller.toCtrl, 
                                        focusNode: controller.toFocus,
                                        inputFormatters: quantities.firstWhere((q) => q.type == controller.quantityType.value).inputFormatters
                                      ),
                                    )
                                  ),
                                  const SizedBox(width: 8),
                                  RichText(
                                    text: TextSpan(
                                      style: TextStyle(
                                        fontSize: 16,
                                        fontWeight: FontWeight.w600,
                                        color: Colors.black
                                      ),
                                      children: [
                                        TextSpan(text: controller.toUnit.value.symbol), 
                                        TextSpan(text: controller.toUnit.value.sup ?? '', style: TextStyle(
                                          fontFeatures: [FontFeature.superscripts()]
                                        ))
                                      ]
                                    )
                                  )
                                ],
                              )
                            )
                          ],
                        )),
                      )
                    ],
                  );
                }
              )
            )
          ),

          Container(
            constraints: BoxConstraints(
              minHeight: MediaQuery.of(context).size.height * .1,
            ),
            // color: Colors.white,
            padding: EdgeInsets.zero,
            child: ResponsiveGridList(
              listViewBuilderOptions: ListViewBuilderOptions(
                physics: const NeverScrollableScrollPhysics(),
                shrinkWrap: true,
                padding: EdgeInsets.zero
              ),
              minItemWidth: 30,
              minItemsPerRow: 4,
              maxItemsPerRow: 4,
              horizontalGridSpacing: 8,
              verticalGridSpacing: 8,
              verticalGridMargin: 16,
              horizontalGridMargin: 16,
              children: [
                _buildInputBtn(text: '7'),
                _buildInputBtn(text: '8'),
                _buildInputBtn(text: '9'),
                _buildInputBtn(text: '⌫', color: Colors.indigo[900], onPressed: () => controller.backspace()),

                _buildInputBtn(text: '4'),
                _buildInputBtn(text: '5'),
                _buildInputBtn(text: '6'),
                _buildInputBtn(text: 'C', color: Colors.red, onPressed: () => controller.clear()),

                _buildInputBtn(text: '1'),
                _buildInputBtn(text: '2'),
                _buildInputBtn(text: '3'),
                _buildInputBtn(
                  child: Icon(
                    HugeIcons.strokeRoundedArrowUp02, 
                    color: Colors.teal[900],
                    size: 30,
                  ),
                  onPressed: () => controller.moveFocusUp()
                ),

                _buildInputBtn(
                  text: '+/-', 
                  onPressed: () => controller.toggleNegative(),
                  disabled: controller.quantityType.value != QuantityType.temperature
                ),
                _buildInputBtn(text: '0'),
                _buildInputBtn(text: '.', onPressed: () => controller.dot()),
                _buildInputBtn(
                  child: Icon(
                    HugeIcons.strokeRoundedArrowDown02,
                    color: Colors.teal[900],
                    size: 30,
                  ),
                  onPressed: () => controller.moveFocusDown()
                ),

              ],
            ),
          )
        ],
      ))
    );
  }

  Widget _buildInputBtn({String? text, Color? color,  Widget? child, void Function()? onPressed, bool disabled = false}) {
    HomeController controller = Get.find();

    return Material(
      color: Colors.grey[200],
      borderRadius: BorderRadius.circular(8),
      child: InkWell(
        onTap: disabled
          ? null
          : onPressed ?? () => controller.input(text!),
        borderRadius: BorderRadius.circular(8),
        // splashColor: Colors.teal[50],
        child: Container(
          width: 80,
          height: 80,
          decoration: BoxDecoration(
            //border: Border.all(color: Colors.grey[300]!),
            // shape: BoxShape.rectangle,
            borderRadius: BorderRadius.circular(8),
            color: Colors.transparent
            // color: Colors.grey[200]
          ),
          child: child ?? Center(child: Text(text ?? '', style: TextStyle(
            fontWeight: FontWeight.bold,
            fontSize: 18,
            color: disabled
              ? Colors.grey[400]
              : color ?? Colors.grey[700]
          ))),
        )
      )
    );
  }

  Widget _buildInputField({
    required TextEditingController inputController, 
    required FocusNode focusNode,
    List<TextInputFormatter>? inputFormatters}) {
    return TextField(
      controller: inputController,
      inputFormatters: inputFormatters,
      focusNode: focusNode,
      style: const TextStyle(
        fontSize: 22,
        fontWeight: FontWeight.w600
      ),
      textAlign: TextAlign.end,
      keyboardType: TextInputType.none,
      enableInteractiveSelection: false,
      decoration: const InputDecoration(
        isDense: true,
        contentPadding: EdgeInsets.zero,
        focusedBorder: InputBorder.none,
        border: InputBorder.none
      ),
    );
  }
}

class HomeController extends GetxController {
  Rx<QuantityType> quantityType = QuantityType.length.obs;

  late Rx<QuantityUnit> fromUnit;
  late TextEditingController fromCtrl;
  String prevInputVal = '';
  FocusNode fromFocus = FocusNode();

  late Rx<QuantityUnit> toUnit;
  late TextEditingController toCtrl;
  FocusNode toFocus = FocusNode();

  late (TextEditingController, FocusNode) activeField;
  int convOrder = 1; // 1: top to bottom, -1: reverse

  HomeController(this.context);
  BuildContext context;

  @override
  void onInit() {
    super.onInit();
    fromCtrl = TextEditingController();
    fromCtrl.addListener(() {
      if (fromFocus.hasFocus && prevInputVal != fromCtrl.text) {
        if (fromCtrl.text.isNotEmpty) {
          if (fromCtrl.text == '-') return;
          handleConversion(fromCtrl.text);
        } else {
          toCtrl.text = '';
        }
      }
    });

    fromFocus.addListener(() {
      if (fromFocus.hasFocus) {
        convOrder = 1;
        activeField = (fromCtrl, fromFocus);
      }
    });
    fromFocus.requestFocus();

    toCtrl = TextEditingController();
    toCtrl.addListener(() {
      if (toFocus.hasFocus && prevInputVal != toCtrl.text) {
        if (toCtrl.text.isNotEmpty) {
          if (toCtrl.text == '-') return;
          handleConversion(toCtrl.text);
        } else {
          fromCtrl.text = '';
        }
      }
    });

    toFocus.addListener(() {
      if (toFocus.hasFocus) {
        convOrder = -1;
        activeField = (toCtrl, toFocus);
      }
    });

    ConvertResult.rustSignalStream.listen(handleResult);

    init();
  }

  void init() {
    fromUnit = quantityType.value.units.first.obs;
    toUnit = quantityType.value.units[1].obs;
    // activeField = (fromCtrl, fromFocus);
    clear();
  }

  @override
  void onClose() {
    fromCtrl.dispose();
    fromFocus.dispose();
    toCtrl.dispose();
    toFocus.dispose();
    super.onClose();
  }

  void input(String char) {
    final inputCtrl = activeField.$1;
    final position = inputCtrl.selection.base.offset;
    var value = inputCtrl.text;
    int dotPos = value.indexOf('.');

    final count = RegExp(r'\d').allMatches(value).length;
    if (count >= 15) {
      showToast(
        'Max. of 15 digits allowed.', 
        context: context,
        animDuration: Duration.zero
      );
      return;
    }

    if (value.contains('.')) {
      final fracMatch = RegExp(r'\.(\d+)').firstMatch(value);
      if (fracMatch != null) {
        final fracLen = fracMatch[1]!.length;

        if (dotPos > 0 && position > dotPos && fracLen >= 10) {
          showToast(
            'Up to 10 digits allowed after decimal point.',
            context: context,
            animDuration: Duration.zero
          );
          return;
        }
      }
    }

    var suffix = value.substring(position, value.length);
    value = value.substring(0, position) + char + suffix;

    inputCtrl.text = value;
    prevInputVal = value;
    inputCtrl.selection = TextSelection.fromPosition(TextPosition(offset: position + 1));
  }

  void backspace() {
    final inputCtrl = activeField.$1;
    final position = inputCtrl.selection.base.offset;
    final value = inputCtrl.text; 

    if (value.isNotEmpty && position != 0) {
      final suffix = value.substring(position, value.length);
      inputCtrl.text = value.substring(0, position - 1) + suffix;
      prevInputVal = inputCtrl.text;
      inputCtrl.selection = TextSelection.fromPosition(TextPosition(offset: position - 1));
    }
  }

  void toggleNegative() {
    final inputCtrl = activeField.$1;
    final value = inputCtrl.text;

    if (value.contains('-')) {
      final suffix = value.substring(1, value.length);
      inputCtrl.text = suffix;
    } else {
      const prefix = '-';
      inputCtrl.text = prefix + value;
    }

    prevInputVal = inputCtrl.text;
  }

  void dot() {
    final inputCtrl = activeField.$1;
    var value = inputCtrl.text;
    final position = inputCtrl.selection.base.offset;

    if (!value.contains('.')) {
      var suffix = value.substring(position, value.length);
      String dot = '.';
      int offset = position + 1;
      if (position == 0) {
        dot = '0.';
        offset += 1;
      }

      // final fracMatch = RegExp(r'\.(\d+)').firstMatch(value);
      if (suffix.length > 10) {
        showToast(
          'Up to 10 digits allowed after decimal point.',
          context: context,
          animDuration: Duration.zero
        );
        return;
      }

      value = value.substring(0, position) + dot + suffix;

      inputCtrl.text = value;
      prevInputVal = value;
      inputCtrl.selection = TextSelection.fromPosition(TextPosition(offset: offset));
    }

  }

  void clear() {
    fromCtrl.text = '';
    toCtrl.text = '';
    prevInputVal = '';
  }

  void moveFocusUp() {
    prevInputVal = fromCtrl.text;
    fromFocus.requestFocus();
  }

  void moveFocusDown() {
    prevInputVal = toCtrl.text;
    toFocus.requestFocus();
  }

  void handleConversion(String value) {
    if (value.isNotEmpty) {
      HomeController controller = Get.find();
      String from = fromUnit.value.symbolStr;
      String to = toUnit.value.symbolStr;
      if (convOrder == -1) {
        final temp = from;
        from = to;
        to = temp;
      }

      print('Converting from: $from to $to');
      convert(
        quantityType.value,
        num.parse(value),
        from,
        to
      );
    }
  }

  convert(QuantityType quantityType, num value, String from, String to) {
    print('Converting...');
    Convert(
      quantity: quantityType.toString(),
      from: from,
      to: to,
      value: value.toDouble(),
    ).sendSignalToRust();

    return value * 2;
  }

  handleResult(RustSignal<ConvertResult> sig) {
    final result = sig.message.value;
    print('Dart Result: $result');
    if (convOrder == 1) {
      toCtrl.text = _handleValue(result);
    } else {
      fromCtrl.text = _handleValue(result);
    }
  }

  String _handleValue(double x) {
    if (x.toString().contains('e')) {
      return x.toString();
    }

    final decLen = x.toString().split('.')[1].length;
    final result = x.toStringAsFixed(
      x.truncateToDouble() == x ? 0 : (decLen > 10 ? 10 : decLen)
    );

    return result;
  }

}