import 'package:appflowy/generated/flowy_svgs.g.dart';
import 'package:appflowy/generated/locale_keys.g.dart';
import 'package:easy_localization/easy_localization.dart';
import 'package:flowy_infra_ui/flowy_infra_ui.dart';
import 'package:flutter/material.dart';

class SettingListTile extends StatelessWidget {
  const SettingListTile({
    super.key,
    this.resetTooltipText,
    this.resetButtonKey,
    required this.label,
    this.hint,
    this.trailing,
    this.subtitle,
    this.onResetRequested,
  });

  final String label;
  final String? hint;
  final String? resetTooltipText;
  final Key? resetButtonKey;
  final List<Widget>? trailing;
  final List<Widget>? subtitle;
  final VoidCallback? onResetRequested;

  @override
  Widget build(BuildContext context) {
    return Row(
      children: [
        Expanded(
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              FlowyText.medium(
                label,
                fontSize: 14,
                overflow: TextOverflow.ellipsis,
              ),
              if (hint != null)
                Padding(
                  padding: const EdgeInsets.only(bottom: 4),
                  child: FlowyText.regular(
                    hint!,
                    fontSize: 10,
                    color: Theme.of(context).hintColor,
                  ),
                ),
              if (subtitle != null) ...subtitle!,
            ],
          ),
        ),
        if (trailing != null) ...trailing!,
        if (onResetRequested != null)
          SettingsResetButton(
            key: resetButtonKey,
            resetTooltipText: resetTooltipText,
            onResetRequested: onResetRequested,
          ),
      ],
    );
  }
}

class SettingsResetButton extends StatelessWidget {
  const SettingsResetButton({
    super.key,
    this.resetTooltipText,
    this.onResetRequested,
  });

  final String? resetTooltipText;
  final VoidCallback? onResetRequested;

  @override
  Widget build(BuildContext context) {
    return FlowyIconButton(
      hoverColor: Theme.of(context).colorScheme.secondaryContainer,
      width: 24,
      icon: FlowySvg(
        FlowySvgs.restore_s,
        color: Theme.of(context).iconTheme.color,
        size: const Size.square(20),
      ),
      iconColorOnHover: Theme.of(context).colorScheme.onPrimary,
      tooltipText:
          resetTooltipText ?? LocaleKeys.settings_appearance_resetSetting.tr(),
      onPressed: onResetRequested,
    );
  }
}
