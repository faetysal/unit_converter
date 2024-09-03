import 'package:dropdown_button2/dropdown_button2.dart';
import 'package:flutter/material.dart';
import 'package:hugeicons/hugeicons.dart';
import 'package:responsive_grid_list/responsive_grid_list.dart';

class Home extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    List<Quantity> quantities = [
      Quantity(id: 1, title: 'Length', icon: HugeIcons.strokeRoundedRuler),
      Quantity(id: 2, title: 'Area', icon: HugeIcons.strokeRoundedBoundingBox),
      Quantity(id: 3, title: 'Temperature', icon: HugeIcons.strokeRoundedTemperature),
      Quantity(id: 4, title: 'Volume', icon: HugeIcons.strokeRoundedDroplet),
      Quantity(id: 5, title: 'Mass', icon: HugeIcons.strokeRoundedWeightScale01),
      Quantity(id: 6, title: 'Data', icon: HugeIcons.strokeRoundedDatabase),
      Quantity(id: 7, title: 'Speed', icon: HugeIcons.strokeRoundedDashboardSpeed02),
      Quantity(id: 8, title: 'Time', icon: HugeIcons.strokeRoundedClock02),
      Quantity(id: 9, title: 'Tip', icon: HugeIcons.strokeRoundedTips)
    ];

    return Scaffold(
      appBar: AppBar(
        backgroundColor: Colors.white,
        elevation: 0,
        shadowColor: Colors.transparent,
        title: Text('Converter', style: TextStyle(color: Colors.black),),
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
            child: ListView.separated(
              scrollDirection: Axis.horizontal,
              itemCount: quantities.length,
              separatorBuilder: (context, index) => const SizedBox(width: 8),
              itemBuilder: (context, index) {
                final quantity = quantities[index];
                return _buildSelectorItem(quantity);
              },
            )
          ),
          Expanded(
            child: Container(
              padding: EdgeInsets.symmetric(horizontal: 16),
              child: LayoutBuilder(
                builder: (context, constraints) {
                  return Column(
                    children: [
                      Container(
                        height: constraints.maxHeight / 2,
                        padding: const EdgeInsets.only(top: 8),
                        decoration: BoxDecoration(
                          border: Border(
                            top: BorderSide(color: Colors.grey),
                            bottom: BorderSide(color: Colors.grey)
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
                              decoration: BoxDecoration(
                                border: Border(
                                  bottom: BorderSide(color: Colors.grey)
                                )
                              ),
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
                _buildInputBtn('7'),
                _buildInputBtn('8'),
                _buildInputBtn('9'),
                _buildInputBtn('x'),

                _buildInputBtn('4'),
                _buildInputBtn('5'),
                _buildInputBtn('6'),
                _buildInputBtn('C'),

                _buildInputBtn('1'),
                _buildInputBtn('2'),
                _buildInputBtn('3'),
                _buildInputBtn('U'),

                _buildInputBtn('+/-'),
                _buildInputBtn('0'),
                _buildInputBtn('.'),
                _buildInputBtn('D'),

              ],
            ),
          )
        ],
      )
    );
  }

  Widget _buildSelectorItem(Quantity q) {
    return Container(
      padding: EdgeInsets.symmetric(vertical: 4),
      width: 100,
      height: 40,
      decoration: BoxDecoration(
        color: Colors.white
        // border: Border.all(color: Colors.green)
      ),
      child: Column(
        mainAxisAlignment: MainAxisAlignment.spaceBetween,
        children: [
          Icon(q.icon, size: 30,),
          const SizedBox(height: 4),
          Container(
            alignment: Alignment.center,
            width: double.infinity,
            //color: Colors.grey,
            child: Text(q.title, style: TextStyle(
              fontSize: 12,
              fontWeight: FontWeight.bold
            ))
          )
        ],
      ),
    );
  }

  Widget _buildInputBtn(String text) {
    return Container(
      width: 80,
      height: 80,
      child: Center(child: Text(text)),
    );
  }
}

class Quantity {
  final int? _id;
  String title;
  IconData? icon;

  Quantity({
    int? id,
    this.title = '',
    this.icon
  }): _id = id;

  int? get id => _id;
}