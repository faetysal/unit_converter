import 'package:dropdown_button2/dropdown_button2.dart';
import 'package:flutter/material.dart';
import 'package:hugeicons/hugeicons.dart';
import 'package:responsive_grid_list/responsive_grid_list.dart';

import 'components/selector.dart';
import 'models/quantity.dart';

class Home extends StatelessWidget {
  const Home({super.key});

  @override
  Widget build(BuildContext context) {
    List<Quantity> quantities = [
      Quantity(id: 1, title: 'Length', icon: HugeIcons.strokeRoundedRuler, type: QuantityType.length),
      Quantity(id: 2, title: 'Area', icon: HugeIcons.strokeRoundedBoundingBox, type: QuantityType.area),
      Quantity(id: 3, title: 'Temperature', icon: HugeIcons.strokeRoundedTemperature, type: QuantityType.temperature),
      Quantity(id: 4, title: 'Volume', icon: HugeIcons.strokeRoundedDroplet, type: QuantityType.volume),
      Quantity(id: 5, title: 'Mass', icon: HugeIcons.strokeRoundedWeightScale01, type: QuantityType.mass),
      Quantity(id: 6, title: 'Data', icon: HugeIcons.strokeRoundedDatabase, type: QuantityType.data),
      Quantity(id: 7, title: 'Speed', icon: HugeIcons.strokeRoundedDashboardSpeed02, type: QuantityType.speed),
      Quantity(id: 8, title: 'Time', icon: HugeIcons.strokeRoundedClock02, type: QuantityType.time),
      Quantity(id: 9, title: 'Tip', icon: HugeIcons.strokeRoundedTips, type: QuantityType.tip)
    ];

    return Scaffold(
      appBar: AppBar(
        backgroundColor: Colors.white,
        elevation: 0,
        shadowColor: Colors.transparent,
        title: const Text('Converter', style: TextStyle(color: Colors.black),),
        centerTitle: false,
      ),
      body: Column(
        mainAxisAlignment: MainAxisAlignment.spaceBetween,
        children: [
          Container(
            height: 60,
            padding: const EdgeInsets.symmetric(horizontal: 16),
            margin: const EdgeInsets.only(top: 5, bottom: 10),
            // color: Colors.orange,
            child: SelectorList(
              value: QuantityType.temperature, 
              onChanged: (v) {},
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
                        padding: const EdgeInsets.only(top: 8),
                        decoration: BoxDecoration(
                          border: Border(
                            top: BorderSide(color: Colors.grey[300]!),
                            bottom: BorderSide(color: Colors.grey[300]!)
                          )
                        ),
                        child: Column(
                          mainAxisAlignment: MainAxisAlignment.spaceBetween,
                          children: [
                            Container(
                              alignment: Alignment.centerLeft,
                              // color: Colors.yellow,
                              child: DropdownButtonHideUnderline(
                                child: DropdownButton2(
                                  customButton: Row(
                                    children: [
                                      Text('Input'),
                                      Icon(Icons.arrow_drop_down)
                                    ],
                                  ),
                                  items: [
                                    DropdownMenuItem(child: Text('Option 1'), value: '1'),
                                    DropdownMenuItem(child: Text('Option 2'), value: '2',),
                                  ],
                                  onChanged: (v) {},
                                  isDense: true,
                                  dropdownStyleData: DropdownStyleData(
                                    width: MediaQuery.of(context).size.width * .7,
                                    decoration: BoxDecoration(
                                      borderRadius: BorderRadius.circular(8)
                                    )
                                  ),
                                )
                              ),
                            ),
                            Container(
                              height: 60,
                              child: TextField(
                                textAlign: TextAlign.end,
                                keyboardType: TextInputType.none,
                                decoration: InputDecoration(
                                  focusedBorder: InputBorder.none,
                                  border: InputBorder.none
                                ),
                              ),
                            ),
                          ],
                        ),
                      ),
                      Container(
                        height: constraints.maxHeight / 2,
                        padding: const EdgeInsets.only(top: 8),
                        child: Column(
                          mainAxisAlignment: MainAxisAlignment.spaceBetween,
                          children: [
                            Container(
                              alignment: Alignment.centerLeft,
                              // color: Colors.yellow,
                              child: DropdownButtonHideUnderline(
                                child: DropdownButton2(
                                  customButton: Row(
                                    children: [
                                      Text('Output'),
                                      Icon(Icons.arrow_drop_down)
                                    ],
                                  ),
                                  items: [
                                    DropdownMenuItem(child: Text('Option 1'), value: '1'),
                                    DropdownMenuItem(child: Text('Option 2'), value: '2',),
                                  ],
                                  onChanged: (v) {},
                                  isDense: true,
                                  dropdownStyleData: DropdownStyleData(
                                    width: MediaQuery.of(context).size.width * .7,
                                    decoration: BoxDecoration(
                                      borderRadius: BorderRadius.circular(8)
                                    )
                                  ),
                                )
                              ),
                            ),
                            Container(
                              height: 60,
                              child: TextField(
                                textAlign: TextAlign.end,
                                keyboardType: TextInputType.none,
                                decoration: InputDecoration(
                                  focusedBorder: InputBorder.none,
                                  border: InputBorder.none
                                ),
                              ),
                            ),
                          ],
                        ),
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
              horizontalGridSpacing: 0,
              verticalGridSpacing: 0,
              children: [
                _buildInputBtn(text: '7'),
                _buildInputBtn(text: '8'),
                _buildInputBtn(text: '9'),
                _buildInputBtn(text: '⌫'),

                _buildInputBtn(text: '4'),
                _buildInputBtn(text: '5'),
                _buildInputBtn(text: '6'),
                _buildInputBtn(text: 'C'),

                _buildInputBtn(text: '1'),
                _buildInputBtn(text: '2'),
                _buildInputBtn(text: '3'),
                _buildInputBtn(child: Icon(HugeIcons.strokeRoundedArrowUp01)),

                _buildInputBtn(text: '+/-'),
                _buildInputBtn(text: '0'),
                _buildInputBtn(text: '.'),
                _buildInputBtn(child: Icon(HugeIcons.strokeRoundedArrowDown01)),

              ],
            ),
          )
        ],
      )
    );
  }

  Widget _buildInputBtn({String? text, Widget? child}) {
    return Container(
      width: 80,
      height: 80,
      decoration: BoxDecoration(
        border: Border.all(color: Colors.grey[300]!)
      ),
      child: child ?? Center(child: Text(text ?? '', style: TextStyle(
        fontWeight: FontWeight.bold,
        fontSize: 18
      ))),
    );
  }
}