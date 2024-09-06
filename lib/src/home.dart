import 'package:dropdown_button2/dropdown_button2.dart';
import 'package:flutter/material.dart';
import 'package:get/get.dart';
import 'package:hugeicons/hugeicons.dart';
import 'package:responsive_grid_list/responsive_grid_list.dart';

import 'components/selector.dart';
import 'models/quantity.dart';

class Home extends StatelessWidget {
  const Home({super.key});

  @override
  Widget build(BuildContext context) {
    HomeController controller = Get.put(HomeController());

    List<Quantity> quantities = [
      Quantity(
        id: 1,
        title: 'Length',
        icon: HugeIcons.strokeRoundedRuler,
        type: QuantityType.length
      ),
      Quantity(id: 2, title: 'Area', icon: HugeIcons.strokeRoundedBoundingBox, type: QuantityType.area),
      Quantity(id: 3, title: 'Temperature', icon: HugeIcons.strokeRoundedTemperature, type: QuantityType.temperature),
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
                                      child: _buildInputField(inputController: controller.fromCtrl, focusNode: controller.fromFocus)
                                    ),
                                  ),
                                  const SizedBox(width: 8),
                                  Text(controller.fromUnit.value.symbol, style: TextStyle(
                                    fontSize: 16,
                                    fontWeight: FontWeight.w600
                                  ))
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
                                        child: Text('${u.title} (${u.symbol})'), 
                                      );
                                    }).toList(),
                                    onChanged: (v) {
                                      controller.toUnit.value = v!;
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
                                      child: _buildInputField(inputController: controller.toCtrl, focusNode: controller.toFocus),
                                    )
                                  ),
                                  const SizedBox(width: 8),
                                  Text(controller.toUnit.value.symbol, style: TextStyle(
                                    fontSize: 16,
                                    fontWeight: FontWeight.w600
                                  ))
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

                _buildInputBtn(text: '+/-'),
                _buildInputBtn(text: '0'),
                _buildInputBtn(text: '.'),
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

  Widget _buildInputBtn({String? text, Color? color,  Widget? child, void Function()? onPressed}) {
    HomeController controller = Get.find();

    return Material(
      color: Colors.grey[200],
      borderRadius: BorderRadius.circular(8),
      child: InkWell(
        onTap: onPressed ?? () => controller.input(text!),
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
            color: color ?? Colors.grey[700]
          ))),
        )
      )
    );
  }

  Widget _buildInputField({required TextEditingController inputController, required FocusNode focusNode}) {
    return TextField(
      controller: inputController,
      focusNode: focusNode,
      style: const TextStyle(
        fontSize: 22,
        fontWeight: FontWeight.w600
      ),
      textAlign: TextAlign.end,
      keyboardType: TextInputType.none,
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
  // late TextSelection fromSelection;
  FocusNode fromFocus = FocusNode();

  late Rx<QuantityUnit> toUnit;
  late TextEditingController toCtrl;
  FocusNode toFocus = FocusNode();

  late (TextEditingController, FocusNode) activeField;
  int convOrder = 1; // 1: top to bottom, -1: reverse

  @override
  void onInit() {
    super.onInit();
    fromCtrl = TextEditingController();
    fromCtrl.addListener(() {
      handleConversion(fromCtrl.text);
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
      handleConversion(toCtrl.text);
    });

    toFocus.addListener(() {
      if (toFocus.hasFocus) {
        convOrder = -1;
        activeField = (toCtrl, toFocus);
      }
    });
    init();
  }

  void init() {
    fromUnit = quantityType.value.units.first.obs;
    toUnit = quantityType.value.units[1].obs;
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

    var suffix = value.substring(position, value.length);
    value = value.substring(0, position) + char + suffix;

    inputCtrl.text = value;
    inputCtrl.selection = TextSelection.fromPosition(TextPosition(offset: position + 1));
  }

  void backspace() {
    final inputCtrl = activeField.$1;
    final position = inputCtrl.selection.base.offset;
    final value = inputCtrl.text; 

    if (value.isNotEmpty && position != 0) {
      final suffix = value.substring(position, value.length);
      inputCtrl.text = value.substring(0, position - 1) + suffix;
      inputCtrl.selection = TextSelection.fromPosition(TextPosition(offset: position - 1));
    }
  }

  void clear() {
    fromCtrl.text = '';
    toCtrl.text = '';
  }

  void moveFocusUp() {
    fromFocus.requestFocus();
  }

  void moveFocusDown() {
    toFocus.requestFocus();
  }

  void handleConversion(String value) {
    HomeController controller = Get.find();
    String from = fromUnit.value.uSym ?? fromUnit.value.symbol;
    String to = toUnit.value.uSym ?? toUnit.value.symbol;
    if (convOrder == -1) {
      final temp = from;
      from = to;
      to = temp;
    }

    print('Converting from: $from to $to');
  }

  void Function() c2f = () {
    print('Celcius to Fahrenheit');
  };

  void fahrenheitToCelcius() {
    print('Fahrenheit to Celcius');
  }

}