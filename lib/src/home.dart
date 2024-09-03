import 'package:dropdown_button2/dropdown_button2.dart';
import 'package:flutter/material.dart';
import 'package:responsive_grid_list/responsive_grid_list.dart';

class Home extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
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
            color: Colors.orange,
            child: ListView.separated(
              scrollDirection: Axis.horizontal,
              itemCount: 12,
              separatorBuilder: (context, index) => const SizedBox(width: 8),
              itemBuilder: (context, index) {
                return Container(
                  width: 80,
                  height: 30,
                  decoration: BoxDecoration(
                    border: Border.all(color: Colors.green)
                  ),
                  child: Column(
                    mainAxisAlignment: MainAxisAlignment.center,
                    children: [
                      Icon(Icons.circle, size: 30,),
                      const SizedBox(height: 4),
                      Text('Title')
                    ],
                  ),
                );
              },
            )
          ),
          Expanded(
            child: Container(
              child: LayoutBuilder(
                builder: (context, constraints) {
                  return Column(
                    children: [
                      Container(
                        height: constraints.maxHeight / 2,
                        child: Column(
                          mainAxisAlignment: MainAxisAlignment.spaceBetween,
                          children: [
                            Container(
                              alignment: Alignment.centerLeft,
                              color: Colors.yellow,
                              child: DropdownButtonHideUnderline(
                                child: DropdownButton2(
                                  customButton: Row(
                                    children: [
                                      Text('Option 1'),
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
                              color: Colors.lightBlueAccent,
                              child: TextField(textAlign: TextAlign.end,),
                            ),
                          ],
                        ),
                      ),
                      Container(
                        height: constraints.maxHeight / 2,
                        color: Colors.blue,
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

  Widget _buildInputBtn(String text) {
    return Container(
      width: 80,
      height: 80,
      child: Center(child: Text(text)),
    );
  }
}