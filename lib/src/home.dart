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
                                      child: _buildInputField(controller: controller.fromCtrl, focusNode: controller.fromFocus)
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
                                      child: _buildInputField(controller: controller.toCtrl, focusNode: controller.toFocus),
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
                _buildInputBtn(text: '⌫', color: Colors.indigo[900], onPressed: () => _backspace()),

                _buildInputBtn(text: '4'),
                _buildInputBtn(text: '5'),
                _buildInputBtn(text: '6'),
                _buildInputBtn(text: 'C', color: Colors.red),

                _buildInputBtn(text: '1'),
                _buildInputBtn(text: '2'),
                _buildInputBtn(text: '3'),
                _buildInputBtn(
                  child: Icon(
                    HugeIcons.strokeRoundedArrowUp02, 
                    color: Colors.teal[900],
                    size: 30,
                  ),
                  onPressed: () => _moveFocusUp()
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
                  onPressed: () => _moveFocusDown()
                ),

              ],
            ),
          )
        ],
      ))
    );
  }

  Widget _buildInputBtn({String? text, Color? color,  Widget? child, void Function()? onPressed}) {
    return Material(
      color: Colors.grey[200],
      borderRadius: BorderRadius.circular(8),
      child: InkWell(
        onTap: onPressed ?? () => _input(text!),
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

  Widget _buildInputField({required TextEditingController controller, required FocusNode focusNode}) {
    return TextField(
      controller: controller,
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

  void _input(String char) {
    HomeController controller = Get.find();
    final inputCtrl = controller.activeField.$1;
    final value = inputCtrl.text + char;
    inputCtrl.text = value;
  }

  void _backspace() {
    HomeController controller = Get.find();
    final inputCtrl = controller.activeField.$1;
    final value = inputCtrl.text; 
    if (value.isNotEmpty) {
      inputCtrl.text = value.substring(0, value.length - 1);
    }
  }

  void _moveFocusUp() {
    HomeController controller = Get.find();
    controller.fromFocus.requestFocus();
  }

  void _moveFocusDown() {
    HomeController controller = Get.find();
    controller.toFocus.requestFocus();
  }
}

class HomeController extends GetxController {
  Rx<QuantityType> quantityType = QuantityType.length.obs;

  late Rx<QuantityUnit> fromUnit;
  late TextEditingController fromCtrl;
  FocusNode fromFocus = FocusNode();

  late Rx<QuantityUnit> toUnit;
  late TextEditingController toCtrl;
  FocusNode toFocus = FocusNode();

  late (TextEditingController, FocusNode) activeField;

  @override
  void onInit() {
    super.onInit();
    fromCtrl = TextEditingController();
    fromCtrl.addListener(() {
      print('Cursor pos: ${fromCtrl.selection.base.offset}');
    });
    fromFocus.requestFocus();

    toCtrl = TextEditingController();
    activeField = (fromCtrl, fromFocus);
    init();
  }

  void init() {
    fromUnit = quantityType.value.units.first.obs;
    toUnit = quantityType.value.units.last.obs;
  }

  @override
  void onClose() {
    fromCtrl.dispose();
    toCtrl.dispose();
    super.onClose();
  }
}