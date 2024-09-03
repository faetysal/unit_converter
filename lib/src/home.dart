import 'package:dropdown_button2/dropdown_button2.dart';
import 'package:flutter/material.dart';
import 'package:gradient_borders/gradient_borders.dart';
import 'package:hugeicons/hugeicons.dart';
import 'package:responsive_grid_list/responsive_grid_list.dart';
import 'package:simple_gradient_text/simple_gradient_text.dart';

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
            child: SelectorList(
              value: 2, 
              onChanged: (v) {},
              children: quantities,
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

  /*Widget _buildSelectorItem(Quantity q, {bool selected = false}) {
    return Container(
      padding: EdgeInsets.symmetric(vertical: 4),
      width: 100,
      height: 40,
      decoration: BoxDecoration(
        color: Colors.white,
        border: selected
          ? GradientBoxBorder(
              gradient: LinearGradient(
                colors: [Colors.pink, Colors.purple],
              )
            )
          : null,
        borderRadius: BorderRadius.circular(8)
        // border: Border.all(color: Colors.green)
      ),
      child: Column(
        mainAxisAlignment: MainAxisAlignment.spaceBetween,
        children: [
          selected
            ? RadiantGradientMask(
              child: Icon(q.icon, size: 30, color: Colors.white,),
            )
            : Icon(q.icon, size: 30, color: Colors.grey[600]),
          const SizedBox(height: 4),
          Container(
            alignment: Alignment.center,
            width: double.infinity,
            //color: Colors.grey,
            child: selected 
              ? GradientText(
                q.title, 
                colors: [Colors.pink, Colors.purple],
                style: TextStyle(
                  fontSize: 12,
                  fontWeight: FontWeight.bold
                ),
              )
              : Text(q.title, style: TextStyle(
              fontSize: 12,
              fontWeight: FontWeight.bold,
              color: Colors.grey[600]
            ))
          )
        ],
      ),
    );
  }*/

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

class RadiantGradientMask extends StatelessWidget {
  RadiantGradientMask({required this.child});
  final Widget child;

  @override
  Widget build(BuildContext context) {
    return ShaderMask(
      shaderCallback: (bounds) => RadialGradient(
        center: Alignment.center,
        radius: 0.5,
        colors: [Colors.pink, Colors.purple],
        tileMode: TileMode.mirror,
      ).createShader(bounds),
      child: child,
    );
  }
}

class SelectorList extends StatelessWidget {
  const SelectorList({super.key,
    required this.value,
    required this.onChanged,
    this.children = const []
  });

  final int value;
  final void Function(int i) onChanged;
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
          value: quantity.id!,
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
  final int value;
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