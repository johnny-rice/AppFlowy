import 'package:appflowy/generated/flowy_svgs.g.dart';
import 'package:appflowy/generated/locale_keys.g.dart';
import 'package:appflowy/plugins/database/application/cell/bloc/date_cell_bloc.dart';
import 'package:appflowy/plugins/database/grid/presentation/layout/sizes.dart';
import 'package:appflowy/plugins/database/widgets/cell_editor/date_cell_editor.dart';
import 'package:appflowy/plugins/database/widgets/row/cells/cell_container.dart';
import 'package:easy_localization/easy_localization.dart';
import 'package:flowy_infra_ui/flowy_infra_ui.dart';
import 'package:flutter/widgets.dart';

import '../editable_cell_skeleton/date.dart';

class DesktopGridDateCellSkin extends IEditableDateCellSkin {
  @override
  Widget build(
    BuildContext context,
    CellContainerNotifier cellContainerNotifier,
    ValueNotifier<bool> compactModeNotifier,
    DateCellBloc bloc,
    DateCellState state,
    PopoverController popoverController,
  ) {
    return AppFlowyPopover(
      controller: popoverController,
      triggerActions: PopoverTriggerFlags.none,
      direction: PopoverDirection.bottomWithLeftAligned,
      constraints: BoxConstraints.loose(const Size(260, 620)),
      margin: EdgeInsets.zero,
      child: Align(
        alignment: AlignmentDirectional.centerStart,
        child: state.fieldInfo.wrapCellContent ?? false
            ? _buildCellContent(state, compactModeNotifier)
            : SingleChildScrollView(
                physics: const NeverScrollableScrollPhysics(),
                scrollDirection: Axis.horizontal,
                child: _buildCellContent(state, compactModeNotifier),
              ),
      ),
      popupBuilder: (BuildContext popoverContent) {
        return DateCellEditor(
          cellController: bloc.cellController,
          onDismissed: () => cellContainerNotifier.isFocus = false,
        );
      },
      onClose: () {
        cellContainerNotifier.isFocus = false;
      },
    );
  }

  Widget _buildCellContent(
    DateCellState state,
    ValueNotifier<bool> compactModeNotifier,
  ) {
    final wrap = state.fieldInfo.wrapCellContent ?? false;
    final dateStr = getDateCellStrFromCellData(
      state.fieldInfo,
      state.cellData,
    );
    return ValueListenableBuilder(
      valueListenable: compactModeNotifier,
      builder: (context, compactMode, _) {
        final padding = compactMode
            ? GridSize.compactCellContentInsets
            : GridSize.cellContentInsets;
        return Padding(
          padding: padding,
          child: Row(
            mainAxisSize: MainAxisSize.min,
            children: [
              Flexible(
                child: FlowyText(
                  dateStr,
                  overflow: wrap ? null : TextOverflow.ellipsis,
                  maxLines: wrap ? null : 1,
                ),
              ),
              if (state.cellData.reminderId.isNotEmpty) ...[
                const HSpace(4),
                FlowyTooltip(
                  message: LocaleKeys.grid_field_reminderOnDateTooltip.tr(),
                  child: const FlowySvg(FlowySvgs.clock_alarm_s),
                ),
              ],
            ],
          ),
        );
      },
    );
  }
}
